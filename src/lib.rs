#![cfg_attr(not(test), no_std)]

pub mod advertisements;
use advertisements::AdFields;
#[cfg(feature="nrf5x")]
pub mod nrf5x;
#[cfg(feature="nrf5x")]
use nrf5x::{Nrf5xHci as HCI};

pub struct Ble<'a> {
    hci: HCI,
    info: AdFields<'a>,
}

impl<'a> Ble<'a> {
    // pub fn new(hci: Nrf5xHci, name:&'a str) -> Self
    pub fn new(hci:HCI, info:AdFields<'a>) -> Self
    {
        Self { hci, info }
    }

    pub fn connections(self:&Self) -> u8 {
        // FIXME determine number of connections
        return 0;
    }

    pub fn advertise(self:&Self) {
        // self.hci.advertise(self.info);
    }
}

/// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=225952
trait BleHci {
}