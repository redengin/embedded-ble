#![cfg_attr(not(test), no_std)]

pub mod gap;
#[cfg(feature="nrf5x")]
pub mod nrf5x;
#[cfg(feature="nrf5x")]
use nrf5x::{Nrf5xHci as HCI};

#[allow(unused)]
const BLE_PDU_SIZE_MAX:usize = 258; // [header (1 byte), length(1 byte), payload(1-255 bytes)]

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

    pub fn advertise(&self, pdu_type:gap::PDU_TYPE, channel:Channel) {
        // advertising channels are CH37, CH38, CH39
        assert!([Channel::CH37, Channel::CH38, Channel::CH39].contains(&channel));

        let mut buffer:[u8;255] = [0;255];
        self.hci.send(
            channel,
            gap::AD_ACCESS_ADDRESS,
            gap::AD_CRCINIT,
            self.ad_fields.to_pdu(&mut buffer, pdu_type)
        );
    }
}

/// https://www.rfwireless-world.com/Terminology/BLE-Advertising-channels-and-Data-channels-list.html
#[derive(Copy, Clone, PartialEq)]
pub enum Channel {
    CH0,  CH1,  CH2,  CH3,  CH4,  CH5,  CH6,  CH7,  CH8,  CH9,
    CH10, CH11, CH12, CH13, CH14, CH15, CH16, CH17, CH18, CH19,
    CH20, CH21, CH22, CH23, CH24, CH25, CH26, CH27, CH28, CH29,
    CH30, CH31, CH32, CH33, CH34, CH35, CH36, CH37, CH38, CH39
}
/// https://www.rfwireless-world.com/Terminology/BLE-Advertising-channels-and-Data-channels-list.html
impl Channel {
    pub fn frequency(&self) -> u8 {
        /// actual frequency (MHz) = 2400 + value
        const FREQUENCIES:[u8;40] = [
            4,  6,  8,  10, 12, 14, 16, 18, 20, 22, /* 0-9 */
            24, 28, 30, 32, 34, 36, 38, 40, 42, 44, /* 10-19 */
            46, 48, 50, 52, 54, 56, 58, 60, 62, 64, /* 20-29 */
            66, 68, 70, 72, 74, 76, 78, 2,  26, 80  /* 30-39 */
        ];
        return FREQUENCIES[*self as usize];
    }
}



/// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=225952
trait BleHci {
}