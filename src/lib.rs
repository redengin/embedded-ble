#![cfg_attr(not(test), no_std)]

pub mod link_layer;
pub mod gap;
#[cfg(feature="nrf5x")]
pub mod nrf5x;
#[cfg(feature="nrf5x")]
use nrf5x::{Nrf5xHci as HCI};

pub struct Ble<'a> {
    hci: HCI,
    ad_fields: gap::AdFields<'a>,
}

impl<'a> Ble<'a> {
    pub fn new(hci: HCI, ad_fields: gap::AdFields<'a>) -> Self
    {
        Self {
            hci,
            ad_fields,
        }
    }

    pub fn is_connected(&self) -> bool {
        return false;
    }

    pub fn advertise(&self, channel: link_layer::Channel, pdu_type: link_layer::ADV_PDU_TYPE) -> bool
    {
        // advertising channels are CH37, CH38, CH39
        debug_assert!([link_layer::Channel::CH37, link_layer::Channel::CH38, link_layer::Channel::CH39].contains(&channel));

        // TODO support no-init
        let mut buffer:[u8;link_layer::ADV_PDU_SIZE_MAX] = [0; link_layer::ADV_PDU_SIZE_MAX];
        let pdu = match pdu_type {
            link_layer::ADV_PDU_TYPE::ADV_IND =>
                    link_layer::AdvPdu::AdvInd(link_layer::ChSel::Unsupported, &self.hci.adv_a, &self.ad_fields),
            link_layer::ADV_PDU_TYPE::ADV_NONCONN_IND =>
                    link_layer::AdvPdu::AdvNonConnInd(&self.hci.adv_a, &self.ad_fields),
            _ => panic!("NOT SUPPORTED")
        };

        return self.hci.send(
            channel,
            link_layer::ADV_ACCESS_ADDRESS,
            link_layer::ADV_CRCINIT,
            pdu.to_buffer(&mut buffer)
        );
    }
}
