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
    // provide debugging support
    use rtt_target::{rtt_init_print, rprintln};
    // provide scaling of time
    use fugit::ExtU64;

// provide monotonic scheduling using RTC for NRF5x hardware
#[cfg(feature="nrf5x")]
    #[monotonic(binds=RTC0, default=true)]
#[cfg(feature="nrf5x")]
    type Tonic = crate::nrf5x::MonotonicRtc<crate::pac::RTC0>;

    use embedded_ble::{Ble, link_layer, gap};

// choose the hardware controller
#[cfg(feature="nrf5x")]
    use embedded_ble::nrf5x as HCI;

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

// configure NRF5X clocks (RTC for monotonic, hfosc for BLE)
#[cfg(feature="nrf5x")]
        crate::nrf5x::init_clocks(cx.device.CLOCK);

// initialize HCI
#[cfg(feature="nrf5x")]
        let hci = HCI::Nrf5xHci::new(cx.device.RADIO, HCI::RadioMode::Ble1Mbit, cx.device.FICR);

        // create the BLE instance
        let info = gap::AdFields { local_name: Some("Advertise Demo"), ..gap::AdFields::default() };
        let ble = Ble::new(hci, info);

        // upon rtic start, begin advertising
        ble_advertiser::spawn().unwrap();

        // return rtic values
        (Shared { ble, },
         Local { },
#[cfg(feature="nrf5x")]
         init::Monotonics(crate::nrf5x::MonotonicRtc::new(cx.device.RTC0)))
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            // go into deep sleep
            cortex_m::asm::wfe();
        }
    }

    // schedule for **minimal** priority
    #[task(shared=[ble], priority=1)]
    fn ble_advertiser(mut cx: ble_advertiser::Context) {
        cx.shared.ble.lock(|ble| {
            let channel = link_layer::Channel::CH37;
            assert!(ble.advertise(channel, link_layer::PDU_TYPE::ADV_NONCONN_IND));
        });
        // continue advertisement forever
        ble_advertiser::spawn_after(1.secs()).unwrap();
    }
}


/// nrf5x support --------------------------------------------
#[cfg(feature="nrf5x")]
mod nrf5x {
    pub(crate) fn init_clocks(clock: crate::pac::CLOCK) {
        // configure RTC source clock (LFCLK) for NRF5x hardware
        crate::Clocks::new(clock)
            .enable_ext_hfosc() // required for bluetooth radio
            .set_lfclk_src_external(crate::clocks::LfOscConfiguration::NoExternalNoBypass)
            .start_lfclk();
    }

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