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
embedded-ble = { version="0.0.1", features=["nrf52832"] }
embedded-ble-nrf5x = { version="0.0.1", features=["nrf52832"] }
```
`feature` chooses the hardware interface:
* Supported `feature` options
    * nrf51 (TODO not currently supported)
    * nrf52805
    * nrf52810
    * nrf52811
    * nrf52832
    * nrf52833
    * nrf52840

For usage example see [rtic_demo.rs](ble/examples/rtic_demo.rs).

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





