#![no_main]
#![no_std]

use panic_rtt_target as _;

use rtic::app;

// choose the hardware pac
#[cfg(feature = "nrf51")]
use nrf51_hal::{pac, Clocks, clocks};
#[cfg(feature = "nrf52805")]
use nrf52805_hal::{pac, Clocks, clocks};
#[cfg(feature = "nrf52810")]
use nrf52810_hal::{pac, Clocks, clocks};
#[cfg(feature = "nrf52811")]
use nrf52811_hal::{pac, Clocks, clocks};
#[cfg(feature = "nrf52832")]
use nrf52832_hal::{pac, Clocks, clocks};
#[cfg(feature = "nrf52833")]
use nrf52833_hal::{pac, Clocks, clocks};
#[cfg(feature = "nrf52840")]
use nrf52840_hal::{pac, Clocks, clocks};

#[app(device=crate::pac, dispatchers=[SWI0_EGU0, SWI1_EGU1])]
mod app {
    use rtt_target::{rtt_init_print, rprintln};

    // provide monotonic scheduling for NRF5x hardware
#[cfg(feature="embedded-ble-nrf5x")]
    use fugit::ExtU64;
#[cfg(feature="embedded-ble-nrf5x")]
    use crate::nrf_monotonic::MonotonicRtc;
#[cfg(feature="embedded-ble-nrf5x")]
    #[monotonic(binds=RTC0, default=true)]
#[cfg(feature="embedded-ble-nrf5x")]
    type Tonic = MonotonicRtc<crate::pac::RTC0>;


    use embedded_ble::Ble;
    // choose controller
#[cfg(feature="embedded-ble-nrf5x")]
    use embedded_ble_nrf5x::Nrf5xBle;

    #[shared]
    struct Shared {
        ble: Ble<'static>,
    }

    #[local]
    struct Local {
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        // configure RTC source clock (LFCLK) for NRF5x hardware
        if cfg!(feature="embedded-ble-nrf5x") {
            let _clocks = crate::Clocks::new(cx.device.CLOCK)
                .set_lfclk_src_external(crate::clocks::LfOscConfiguration::NoExternalNoBypass)
                .start_lfclk();
        }

        // TODO determine what this is (i.e. is there a mac address?)
        const ACCESS_ADDRESS:u32 = 0;
#[cfg(feature="embedded-ble-nrf5x")]
        let ble_controller = Nrf5xBle::init(cx.device.RADIO, ACCESS_ADDRESS);

        let ble = Ble::new(ble_controller, "hello world");
        ble_advertiser::spawn().unwrap();

        (Shared {
            ble,
         },
         Local {
         },
         init::Monotonics(MonotonicRtc::new(cx.device.RTC0)))
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            // go into deep sleep
            rprintln!("sleeping...");
            cortex_m::asm::wfe();
        }
    }

    /// schedule for **lowest** priority (1)
    #[task(shared=[ble], priority=1)]
    fn ble_advertiser(mut cx:ble_advertiser::Context) {
        cx.shared.ble.lock(|ble| {
            // only advertise if we're not connected
            if ! ble.is_connected() {
                rprintln!("advertising...");
                ble.advertise();
            }
        });
        rprintln!("advertising done");
        ble_advertiser::spawn_after(1.secs()).unwrap();
    }

    /// schedule for **highest** priority
    #[task(binds=RADIO, shared=[ble], priority=8)]
    fn ble_handler(mut cx:ble_handler::Context) {
        rprintln!("handling radio event...");
        cx.shared.ble.lock(|ble| {
            let has_work = ble.radio_event();
            if has_work {
                ble_worker::spawn().ok();
            }
        });
    }

    /// schedule for high priority (apps responsive to state changes)
    #[task(shared=[ble], priority=7)]
    fn ble_worker(mut cx:ble_worker::Context) {
        cx.shared.ble.lock(|ble| {
            rprintln!("working...");
            ble.work();
        });
    }
}


#[cfg(feature="embedded-ble-nrf5x")]
mod nrf_monotonic {
    //------------------------------------------------------------------------------
    // RTIC Monotonic impl for the RTCs (https://github.com/eflukx/rtic-rtc-example)
    use crate::pac::{rtc0, RTC0, RTC1, RTC2};

    use rtic::rtic_monotonic::Monotonic;
    pub struct  MonotonicRtc<T: InstanceRtc> {
        overflow: u64,
        rtc: T,
    }

    impl<T: InstanceRtc> MonotonicRtc<T> {
        pub fn new(rtc: T) -> Self {
            unsafe { rtc.prescaler.write(|w| w.bits(0)) };

            Self { overflow: 0, rtc }
        }

        pub fn is_overflow(&self) -> bool {
            self.rtc.events_ovrflw.read().bits() == 1
        }
    }

    impl<T: InstanceRtc> Monotonic for MonotonicRtc<T> {
        type Instant = fugit::TimerInstantU64<32_768>;
        type Duration = fugit::TimerDurationU64<32_768>;

        unsafe fn reset(&mut self) {
            self.rtc.intenset.write(|w| w.compare0().set().ovrflw().set());
            self.rtc.evtenset.write(|w| w.compare0().set().ovrflw().set());

            self.rtc.tasks_clear.write(|w| w.bits(1));
            self.rtc.tasks_start.write(|w| w.bits(1));
        }

        #[inline(always)]
        fn now(&mut self) -> Self::Instant {
            let cnt = self.rtc.counter.read().bits();
            let ovf = if self.is_overflow() { self.overflow.wrapping_add(1) } else { self.overflow };

            Self::Instant::from_ticks((ovf << 24) | cnt as u64)
        }

        fn set_compare(&mut self, instant: Self::Instant) {
            let now = self.now();

            // Since the timer may or may not overflow based on the requested compare val, we check
            // how many ticks are left.
            let val = match instant.checked_duration_since(now) {
                Some(x) if x.ticks() <= 0xffffff => instant.duration_since_epoch().ticks() & 0xffffff, // Will not overflow
                _ => 0, // Will overflow or in the past, set the same value as after overflow to not get extra interrupts
            };

            unsafe { self.rtc.cc[0].write(|w| w.bits(val as u32)) };
        }

        fn clear_compare_flag(&mut self) {
            unsafe { self.rtc.events_compare[0].write(|w| w.bits(0)) };
        }

        #[inline(always)]
        fn zero() -> Self::Instant {
            Self::Instant::from_ticks(0)
        }

        fn on_interrupt(&mut self) {
            if self.is_overflow() {
                self.overflow = self.overflow.wrapping_add(1);
                self.rtc.events_ovrflw.write(|w| unsafe { w.bits(0) });
            }
        }
    }

    pub trait InstanceRtc: core::ops::Deref<Target = rtc0::RegisterBlock> {}
    impl InstanceRtc for RTC0 {}
    impl InstanceRtc for RTC1 {}
    impl InstanceRtc for RTC2 {}
}