#![no_main]
#![no_std]

use panic_rtt_target;

use rtic::app;

#[cfg(feature = "nrf52832")]
use nrf52832_pac as pac;

#[app(device = crate::pac)]
mod app {
    use rtt_target::{rtt_init_print, rprintln};
    use embedded_ble::Gatt;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        let gatt = Gatt{};

        (Shared {}, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            rprintln!("idle");
        }
    }
}