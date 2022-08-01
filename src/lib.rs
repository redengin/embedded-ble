#![cfg_attr(not(test), no_std)]

pub mod link_layer;
use link_layer::{Channel};
pub mod gap;
#[cfg(feature="nrf5x")]
pub mod nrf5x;
#[cfg(feature="nrf5x")]
use nrf5x::{Nrf5xHci as HCI};

use crate::link_layer::{AdvPdu, ChSel, AdvA};

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

    pub fn advertise(&self, _pdu_type:link_layer::ADV_PDU_TYPE, channel:Channel) {
        // advertising channels are CH37, CH38, CH39
        assert!([Channel::CH37, Channel::CH38, Channel::CH39].contains(&channel));

        let mut buffer:[u8;ADV_PDU_SIZE_MAX] = [0;ADV_PDU_SIZE_MAX];
        let adv_a = AdvA::Public([0;6]);
        let pdu = AdvPdu::AdvInd(ChSel::Unsupported, &adv_a, &self.ad_fields)
            .to_buffer(&mut buffer);
        self.hci.send(
            channel,
            gap::AD_ACCESS_ADDRESS,
            gap::AD_CRCINIT,
            pdu
        );
    }
}


/// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=225952
trait BleHci {
}