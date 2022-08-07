#![cfg_attr(not(test), no_std)]

pub mod link_layer;
pub mod gap;
#[cfg(feature="nrf5x")]
pub mod nrf5x;
#[cfg(feature="nrf5x")]
use nrf5x::{Nrf5xHci as HCI};

use crate::link_layer::{AdvPdu};

#[allow(unused)]
const ADV_PDU_SIZE_MAX:usize = 1 + 1 + 6 + 31; // header + length + AdvA + AdvData
#[allow(unused)]
const PDU_SIZE_MAX:usize = 1 + 1 + 255;

pub struct Ble<'a> {
    hci: HCI,
    ad_fields: gap::AdFields<'a>,
}

impl<'a> Ble<'a> {
    pub fn new(hci:HCI, ad_fields:gap::AdFields<'a>) -> Self
    {
        Self {
            hci,
            ad_fields,
        }
    }

    /// returns the number of active connections
    pub fn connections(&self) -> usize {
        // FIXME determine number of connections
        return 0
    }

    pub fn advertise(&self, channel:link_layer::Channel, _pdu_type:link_layer::ADV_PDU_TYPE) -> bool
    {
        // advertising channels are CH37, CH38, CH39
        debug_assert!([link_layer::Channel::CH37, link_layer::Channel::CH38, link_layer::Channel::CH39].contains(&channel));

        // TODO support no-init
        let mut buffer:[u8;ADV_PDU_SIZE_MAX] = [0;ADV_PDU_SIZE_MAX];
        let pdu = AdvPdu::AdvNonConnInd(&self.hci.adv_a, &self.ad_fields);

        return self.hci.send(
            channel,
            link_layer::ADV_ACCESS_ADDRESS,
            link_layer::ADV_CRCINIT,
            pdu.to_buffer(&mut buffer)
        );
    }
}


/// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=225952
trait BleHci {
}