#![cfg_attr(not(test), no_std)]

pub mod link_layer;
pub mod gap;

// select the hardware interface
#[cfg(test)]
use FakeHci as HCI;  /* implemented at end of this file */
#[cfg(feature="nrf5x")] 
pub mod nrf5x;
#[cfg(feature="nrf5x")]
use nrf5x::{Nrf5xHci as HCI};

use rtt_target::{rprintln};

pub struct Ble<'a> {
    hci: HCI,
    ad_fields: gap::AdFields<'a>,
    buffer: link_layer::PduBuffer,
}

impl<'a> Ble<'a> {
    pub fn new(hci: HCI, ad_fields: gap::AdFields<'a>) -> Self
    {
        Self {
            hci,
            ad_fields,
            buffer: [0; link_layer::PDU_SIZE_MAX],
        }
    }

    pub fn is_connected(&self) -> bool {
        // FIXME
        return false;
    }

    /// send out a BlueTooth non-connectable advertisement
    pub fn advertise(&self, channel: link_layer::Channel, pdu_type: link_layer::PDU_TYPE) -> bool {
        // advertising channels are CH37, CH38, CH39
        debug_assert!([link_layer::Channel::CH37, link_layer::Channel::CH38, link_layer::Channel::CH39].contains(&channel));

        let mut buffer:[u8; link_layer::ADV_PDU_SIZE_MAX] = [0; link_layer::ADV_PDU_SIZE_MAX];
        let pdu_slice= match pdu_type {

            link_layer::PDU_TYPE::ADV_NONCONN_IND => {
                    let pdu =
                            link_layer::AdvNonConnIndPdu{adv_a: &self.hci.adv_a,
                                                         adv_data: &self.ad_fields};
                    pdu.write(&mut buffer)
            }

            link_layer::PDU_TYPE::ADV_IND => {
                let pdu =
                        link_layer::AdvIndPdu{ch_sel: link_layer::ChSel::Unsupported,
                                              adv_a: &self.hci.adv_a,
                                              adv_data: &self.ad_fields};
                pdu.write(&mut buffer)
            }

            _ => { panic!("not implemented") }
        };

        return self.hci.send(
            pdu_slice,
            channel,
            link_layer::ADV_ACCESS_ADDRESS,
            link_layer::ADV_CRCINIT,
        )
    }

    pub fn listen(&mut self,
                  channel:link_layer::Channel,
                  access_address:link_layer::AccessAddress) -> bool
    {
        self.hci.listen(&mut self.buffer, channel, access_address)
    }

    /// handle a received packet
    pub fn handle_packet(&self) {
        // handle the hardware
        self.hci.handle_receive();

        // determine pdu type
        match link_layer::PDU_TYPE::pdu_type(&self.buffer) {
            Some(pdu_type) => match pdu_type {
                _ => rprintln!("Unhandled PDU (hex) {:X?}", self.buffer),
            }
            None => debug_assert!(false, "Unknown PDU (hex) {:X?}", self.buffer)
        }
    }
}



pub struct FakeHci {
    pub adv_a: link_layer::AdvA,
}
impl FakeHci {
    pub fn send(&self, _: link_layer::Channel, _: link_layer::AccessAddress, _: link_layer::CrcInit, _: &[u8]) -> bool { true }
}