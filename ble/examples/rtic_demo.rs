#![no_main]
#![no_std]

use panic_rtt_target as _;

use rtic::app;

// choose the hardware pac
#[cfg(feature = "nrf51")]
use nrf51_hal::{pac};
#[cfg(feature = "nrf52805")]
use nrf52805_hal::{pac};
#[cfg(feature = "nrf52810")]
use nrf52810_hal::{pac};
#[cfg(feature = "nrf52811")]
use nrf52811_hal::{pac};
#[cfg(feature = "nrf52832")]
use nrf52832_hal::{pac};
#[cfg(feature = "nrf52833")]
use nrf52833_hal::{pac};
#[cfg(feature = "nrf52840")]
use nrf52840_hal::{pac};

#[app(device=crate::pac, dispatchers=[SWI0_EGU0, SWI1_EGU1])]
mod app {
    use embedded_ble::Ble;
    // choose controller
    // use controller::BleController;
    #[cfg(any(
        feature="nrf51",
        feature="nrf52805",
        feature="nrf52810",
        feature="nrf52811",
        feature="nrf52832",
        feature="nrf52833",
        feature="nrf52840",
    ))]
    use embedded_ble_nrf5x::Nrf5xBle;

    use fugit::ExtU32;
    use rtt_target::{rtt_init_print, rprintln};


    // provide monotonic scheduling
    use crate::MonotonicTimer;
    #[monotonic(binds=TIMER1, default=true)]
    type Tonic = MonotonicTimer<crate::pac::TIMER1>;

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

        // TODO determine what this is (i.e. is there a mac address?)
        const ACCESS_ADDRESS:u32 = 0;
        let ble_controller = {

        };
        let hw_ble = Nrf5xBle::init(cx.device.RADIO, ACCESS_ADDRESS);
        let ble = Ble::new(hw_ble, "hello world");
        ble_advertiser::spawn().unwrap();

        (Shared {
            ble,
         },
         Local {
         },
         init::Monotonics(MonotonicTimer::new(cx.device.TIMER1)))
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


// Taken from: https://github.com/kalkyl/nrf-play/blob/47f4410d4e39374c18ff58dc17c25159085fb526/src/mono.rs
// RTIC Monotonic impl for the 32-bit timers
pub use fugit::{self, ExtU32};
// use nrf52832_hal::pac::{timer0, TIMER0, TIMER1, TIMER2};
use nrf52832_hal::pac::{timer0, TIMER1};
use rtic::rtic_monotonic::Monotonic;

pub struct MonotonicTimer<T: Instance32>(T);

impl<T: Instance32> MonotonicTimer<T> {
    pub fn new(timer: T) -> Self {
        timer.prescaler.write(
            |w| unsafe { w.prescaler().bits(4) }, // 1 MHz
        );
        timer.bitmode.write(|w| w.bitmode()._32bit());
        MonotonicTimer(timer)
    }
}

impl<T: Instance32> Monotonic for MonotonicTimer<T> {
    type Instant = fugit::TimerInstantU32<1_000_000>;
    type Duration = fugit::TimerDurationU32<1_000_000>;

    unsafe fn reset(&mut self) {
        self.0.intenset.modify(|_, w| w.compare0().set());
        self.0.tasks_clear.write(|w| w.bits(1));
        self.0.tasks_start.write(|w| w.bits(1));
    }

    #[inline(always)]
    fn now(&mut self) -> Self::Instant {
        self.0.tasks_capture[1].write(|w| unsafe { w.bits(1) });
        Self::Instant::from_ticks(self.0.cc[1].read().bits())
    }

    fn set_compare(&mut self, instant: Self::Instant) {
        self.0.cc[0].write(|w| unsafe { w.cc().bits(instant.duration_since_epoch().ticks()) });
    }

    fn clear_compare_flag(&mut self) {
        self.0.events_compare[0].write(|w| w);
    }

    #[inline(always)]
    fn zero() -> Self::Instant {
        Self::Instant::from_ticks(0)
    }
}

pub trait Instance32: core::ops::Deref<Target = timer0::RegisterBlock> {}
// impl Instance32 for TIMER0 {}    unusable as pac uses it internally
impl Instance32 for TIMER1 {}
// impl Instance32 for TIMER2 {}