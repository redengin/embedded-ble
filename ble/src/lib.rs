#![cfg_attr(not(test), no_std)]

use bluetooth_hci::Controller;

// pub mod gatt_server;
// mod gatt;
mod advertisements;

pub struct Ble {
    // hci: HCI,
}

impl Ble {
    pub fn new() -> Self
    {
        Self{
        }
    }

    pub fn is_connected(self:&Self) -> bool {
        todo!()
    }

    pub fn advertise(self:&Self) {
        todo!()
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