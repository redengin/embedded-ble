#![cfg_attr(not(test), no_std)]

use bluetooth_hci::Controller;

// pub mod gatt_server;
// mod gatt;
pub(crate) mod advertisements;
use advertisements::Advertisement;

pub struct Ble<'a> {
    local_name: &'a str,
}

impl<'a> Ble<'a> {
    pub fn new(local_name: &'a str) -> Self
    {
        Self{
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
        let pdu = ad.adv_ind_pdu(&mut buffer);
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