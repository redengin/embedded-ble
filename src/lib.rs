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

    /// send out a BlueTooth non-connectable advertisement
    pub fn beacon(&self, channel: link_layer::Channel) -> bool {
        // advertising channels are CH37, CH38, CH39
        debug_assert!([link_layer::Channel::CH37, link_layer::Channel::CH38, link_layer::Channel::CH39].contains(&channel));

        let mut buffer:[u8;link_layer::PDU_SIZE_MAX] = [0; link_layer::PDU_SIZE_MAX];
        let pdu = link_layer::AdvPdu::AdvNonConnInd(&self.hci.adv_a, &self.ad_fields);

        return self.hci.send(
            channel,
            link_layer::ADV_ACCESS_ADDRESS,
            link_layer::ADV_CRCINIT,
            pdu.write(&mut buffer)
        )
    }

    /// send out a BlueTooth connectable advertisement
    pub fn advertise(&self, channel: link_layer::Channel) -> bool
    {
        // advertising channels are CH37, CH38, CH39
        debug_assert!([link_layer::Channel::CH37, link_layer::Channel::CH38, link_layer::Channel::CH39].contains(&channel));

        let mut buffer:[u8;link_layer::PDU_SIZE_MAX] = [0; link_layer::PDU_SIZE_MAX];
        let header = link_layer::AuxAdvExtendedHeader{
            advA: Some(&self.hci.adv_a),
            adi: Some(link_layer::Adi{did: 0, sid: 0}),
            ..link_layer::AuxAdvExtendedHeader::default()
        };
        let pdu = link_layer::AdvPdu::AuxAdvInd(&header, &self.ad_fields);

        return self.hci.send(
            channel,
            link_layer::ADV_ACCESS_ADDRESS,
            link_layer::ADV_CRCINIT,
            pdu.write(&mut buffer)
        )
    }
}
