#![cfg_attr(not(test), no_std)]

// pub mod gatt_server;
// mod gatt;
pub(crate) mod advertisements;
use advertisements::Advertisement;
mod gap;

// choose a hardware driver for BLE
#[cfg(any(
    feature="nrf51",
    feature="nrf52805",
    feature="nrf52810",
    feature="nrf52811",
    feature="nrf52832",
    feature="nrf52833",
    feature="nrf52840",
))]
use embedded_ble_nrf5x::Nrf5xBle as HwBle;

pub struct Ble<'a> {
    hw_ble: HwBle,
    local_name: &'a str,
}

impl<'a> Ble<'a> {
    pub fn new(hw_ble: HwBle, local_name: &'a str) -> Self
    {
        Self{
            hw_ble,
            local_name,
        }
    }

    pub fn is_connected(self:&Self) -> bool {
        todo!()
    }

    pub fn advertise(self:&Self) {
        let ad = Advertisement {
            local_name: Some("hello world"),
            ..Advertisement::default()
        };
        let mut buffer:[u8;255] = [0;255];
        let len = ad.adv_ind_pdu(&mut buffer).unwrap();
        self.hw_ble.send(&buffer[0..len]).unwrap();
    }

    /// returns `true` if there are GATT events, else false
    pub fn radio_event(self:&mut Self) -> bool {
        todo!()
    }

    /// perform GATT callbacks
    pub fn work(self:&mut Self) {
        todo!()
    }
}