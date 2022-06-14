#![no_main]
#![no_std]

use panic_rtt_target;

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
    use rtt_target::{rtt_init_print, rprintln};
    // choose the hardware hal
    #[cfg(feature = "nrf52832")]
    use nrf52832_hal::{self as hal};
    use bluetooth_hci::Controller;

    use embedded_ble::Ble;

    #[shared]
    struct Shared {
        ble: Ble,
    }

    #[local]
    struct Local {
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        // let hci = nrf5x_hci::Hci.new();
        // let ble = Ble::new(hci);
        let ble = Ble::new();

        (Shared {
            ble,
         },
         Local {
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

    /// schedule for **lowest** priority (1)
    #[task(shared=[ble], priority=1)]
    fn ble_advertiser(mut cx:ble_advertiser::Context) {
        cx.shared.ble.lock(|ble| {
            // only advertise if we're not connected
            if ! ble.is_connected() {
                ble.advertise();
            }
        });
        ble_advertiser::spawn().unwrap();
    }

    /// schedule for **highest** priority
    #[task(binds=RADIO, shared=[ble], priority=8)]
    fn ble_handler(mut cx:ble_handler::Context) {
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
            ble.work();
        });
    }

}