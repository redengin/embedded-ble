#![cfg_attr(not(test), no_std)]

use controller::BleController;

// pub mod gatt_server;
// mod gatt;
pub(crate) mod advertisements;
use advertisements::Advertisement;
mod gap;

pub struct Ble<'a> {
    local_name: &'a str,
    controller: &'a dyn BleController,
}

impl<'a> Ble<'a> {
    pub fn new(controller: &'a dyn BleController, local_name: &'a str) -> Self
    {
        // TODO determine what this is (i.e. is there a mac address?)
        const access_address:u32 = 0;

        Self{
            local_name,
            controller,
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