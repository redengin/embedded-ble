Bluetooth® Low Energy stack for embedded Rust
================================================================================

<!-- ![architecture](https://software-dl.ti.com/lprf/simplelink_cc2640r2_sdk/1.00.00.22/exports/docs/blestack/html/_images/image4.jpeg) -->
```mermaid
classDiagram
    direction LR
    class BLE {
        GAP
        GATT
        SecurityManager
        ATT
        L2CAP
    }

    class HCI {
        Radio
    }

    BLE --> HCI : uses
```

`Host` is implemented in [`ble`](ble/). This implementation is hardware agnostic.

`Controller/HCI` is implemented in `*-hci`
* [nrf52x-hci](nrf5x-hci/) supports [nrf52 family](https://github.com/nrf-rs/nrf-hal)


Demo
================================================================================
The repos's [Embed.toml](Embed.toml) is setup for `chip = "nRF52832_xxAA"` - if
you wish to run on other targets, you'll need to modify accordingly.

```sh
cargo embed --example rtic_demo --features nrf52832 --target thumbv7em-none-eabihf
```
