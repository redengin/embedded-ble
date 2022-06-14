#![cfg_attr(not(test), no_std)]

use bluetooth_hci::Controller;

// pub mod gatt_server;
// mod gatt;


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





/// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1070566
struct Advertisement<'a> {
    local_name: Option<&'a [u8]>,
    flags: Option<u8>,

    // service_uuid16_list: Option(uuid),
}

struct ScanResponse {
    adva: [u8; 6],
    data: [u8]      // max 31 bytes
}

