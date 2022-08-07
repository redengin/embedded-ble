Overview
================================================================================
`embedded-ble` is an open-source implementation per the
[BlueTooth](https://www.bluetooth.com/specifications/specs/) specification for 
embedded Rust.

This allows Rust developers to create embedded appliances with BLE connectivity
using a scheduler (i.e. [RTIC](http://rtic.rs)) rather than require a full
operating system.


Usage
================================================================================
`Cargo.toml`
```toml
[dependencies]
embedded-ble = { version="0.0", features=["nrf52832"] }
embedded-ble-nrf5x = { version="0.0", features=["nrf52832"] }
```
`feature` chooses the hardware interface:
* Supported hardware 
    * nrf52832
<!-- TODO 
    * nrf51
    * nrf52805
    * nrf52810
    * nrf52811
    * nrf52833
    * nrf52840
-->

API Documentation
--------------------------------------------------------------------------------
TODO <!-- TODO generate rust docs -->

Demo
================================================================================
(if you wish to run on other targets, see [configuring demo](#demo_config))

```sh
cargo embed --example rtic_demo --features nrf52832 --target thumbv7em-none-eabihf
```

<a id="demo_config">Configuring Demos For Other Hardware</a>
--------------------------------------------------------------------------------
TODO

Unit Testing
================================================================================
```sh
cargo test --lib --features nrf52832
```

Contributing
================================================================================
The Bluetooth specifications don't draw a hardline between protocol and hardware
support - the closest is [HCI](https://software-dl.ti.com/lprf/simplelink_cc2640r2_latest/docs/blestack/ble_user_guide/html/ble-stack-3.x/hci.html).
Using the HCI protocol for devices that can provide direct BLE hardware access
wastes large amount of compute.

Unfortunately, hardware manufactures poorly describe how to access BLE hardware
directly, so it is necessary to find an open-source implementation to reverse
engineer into Rust.

The [Apache Mynewt-nimble](https://github.com/apache/mynewt-nimble) project
provides open-source implementation for common [hardware](https://github.com/apache/mynewt-core#overview).




