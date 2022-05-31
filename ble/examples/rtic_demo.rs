#![no_main]
#![no_std]

use panic_rtt_target;

use rtic::app;

#[cfg(feature = "nrf52832")]
use nrf52832_pac as pac;

#[app(device = crate::pac)]
mod app {
    use rtt_target::{rtt_init_print, rprintln};
    // use embedded_ble::Ble;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        // ble: Ble,
    }

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        // const ACCESS_ADDRESS:u32 = 0xDEADBEEF;
        // let ble = Ble::new(ACCESS_ADDRESS);

        (Shared {},
         Local {
            // ble,
         },
         init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            // go into deep sleep
            cortex_m::asm::wfe();
        }
    }

    // #[task(binds=RADIO, local=[ble], priority=3)]
    // fn ble_task(ctx : ble_task::Context) {

    // }

}