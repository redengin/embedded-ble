use crate::{gap::AdFields};

type AccessAddress = u32;
/// https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#G41.455603
pub const ADV_ACCESS_ADDRESS:AccessAddress = 0x8E89BED6_u32.to_le();
/// https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#G41.453964
pub const ADV_CRCINIT:u32 = 0x555555;

/// https://www.rfwireless-world.com/Terminology/BLE-Advertising-channels-and-Data-channels-list.html
/// Core_v5.3.pdf#G41.455772
#[derive(Copy, Clone, PartialEq)]
pub enum Channel {
    CH0,  CH1,  CH2,  CH3,  CH4,  CH5,  CH6,  CH7,  CH8,  CH9,
    CH10, CH11, CH12, CH13, CH14, CH15, CH16, CH17, CH18, CH19,
    CH20, CH21, CH22, CH23, CH24, CH25, CH26, CH27, CH28, CH29,
    CH30, CH31, CH32, CH33, CH34, CH35, CH36, CH37, CH38, CH39
}
/// https://www.rfwireless-world.com/Terminology/BLE-Advertising-channels-and-Data-channels-list.html
/// Core_v5.3.pdf#G41.403618
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

/// Core_v5.3.pdf#G41.403922
#[allow(non_camel_case_types)]
pub enum ADV_PDU_TYPE {
    ADV_IND         = 0b0000,   // Connectable Scannable Undirected advertising
    ADV_DIRECT_IND  = 0b0001,   // Connectable Directed advertising
    ADV_NONCONN_IND = 0b0010,   // Non-Connectable Non-Scannable Undirected advertising
    // Scanning: enables devices to broadcast more advertising data than is allowed in a single advertising packet.
    SCAN_REQ        = 0b0011,   // aka AUX_SCAN_REQ
    SCAN_RSP        = 0b0100,
    // Initiating: establishing a connection between a peripheral device and a central device
    CONNECT_IND     = 0b0101,   // this is the connection request packet sent on one of the Primary advertising channels
                                // aka AUX_CONNECT_REQ
    // Extending: option to advertise on the Secondary advertising channels in addition to the Primary advertising channels
    ADV_SCAN_IND    = 0b0110,   // Scannable Undirected advertising
    ADV_EXT_IND     = 0b0111,   // aka AUX_ADV_IND, AUX_SCAN_RSP, AUX_SYNC_IND, AUX_CHAIN_IND
    AUX_CONNECT_RSP = 0b1000,
}

pub(crate) type Address = [u8;6];
pub enum TxRxAdvAddress {
    Public(Address),
    RandomStatic(Address),
    PrivateStatic(Address),
}

pub type AdvA = TxRxAdvAddress;
pub type TargetA = TxRxAdvAddress;
pub type InitA = TxRxAdvAddress;
pub type ScanA = Address;

pub enum ChSel {
    Supported,
    Unsupported,
}

pub enum AdvPdu<'a> {
    AdvInd(ChSel, &'a AdvA, &'a AdFields<'a>),
    AdvDirectInd(ChSel, &'a AdvA, &'a InitA, &'a AdFields<'a>),
    AdvNonConnInd(&'a AdvA, &'a AdFields<'a>),
    // ScanReq(&'a ScanA, &'a AdvA, &'a AdFields<'a>),
    // ScanRsp(&'a AdvA, &'a AdFields<'a>),
    // AdvScanInd(&'a AdvA, &'a AdFields<'a>),
}

impl<'a> AdvPdu<'a> {
    pub(crate) fn to_buffer(&self, buffer:&'a mut [u8]) -> &'a [u8]
    {
        let mut pdu_size = 0;

        const TYPE_SHIFT:usize = 4;
        const CHSEL_SHIFT:usize = 2;
        const TXADD_SHIFT:usize = 1;
        const RXADD_SHIFT:usize = 0;

        // set the pdu type
        buffer[0] = match self {
            AdvPdu::AdvInd(chsel, adv_a, ..) => {
                // base pdu type
                ((ADV_PDU_TYPE::ADV_IND as u8) << TYPE_SHIFT)
                // chsel bit
                |   (match chsel { ChSel::Supported => 1, _ => 0} << CHSEL_SHIFT)
                // txadd bit
                |   (match adv_a { TxRxAdvAddress::Public(..) => 1, _ => 0} << TXADD_SHIFT)
            },
            AdvPdu::AdvDirectInd(chsel, adv_a, target_a, ..) => {
                // base pdu type
                ((ADV_PDU_TYPE::ADV_DIRECT_IND as u8) << TYPE_SHIFT)
                // chsel bit
                |   (match chsel { ChSel::Supported => 1, _ => 0} << CHSEL_SHIFT)
                // txadd bit
                |   (match adv_a { TxRxAdvAddress::Public(..) => 1, _ => 0} << TXADD_SHIFT)
                // rxadd bit
                |   (match target_a { TxRxAdvAddress::Public(..) => 1, _ => 0} << RXADD_SHIFT)
            },
            AdvPdu::AdvNonConnInd(adv_a, ..) => {
                // base pdu type
                ((ADV_PDU_TYPE::ADV_NONCONN_IND as u8) << TYPE_SHIFT)
                // txadd bit
                |   (match adv_a { TxRxAdvAddress::Public(..) => 1, _ => 0} << TXADD_SHIFT)
            },
        };
        pdu_size += 1;

        // skip a byte for length (will be set at end)
        pdu_size += 1;

        // set the base pdu data
        match self {
            AdvPdu::AdvInd(_, adv_a, ..)
            | AdvPdu::AdvNonConnInd(adv_a, ..) => {
                match adv_a {
                    TxRxAdvAddress::Public(adv_a) 
                    | TxRxAdvAddress::RandomStatic(adv_a) 
                    | TxRxAdvAddress::PrivateStatic(adv_a) => {
                        buffer[pdu_size..(pdu_size+adv_a.len())].copy_from_slice(adv_a);
                        pdu_size += adv_a.len();
                    },
                }
            },
            AdvPdu::AdvDirectInd(_, adv_a, target_a, ..) => {
                match adv_a {
                    TxRxAdvAddress::Public(adv_a) 
                    | TxRxAdvAddress::RandomStatic(adv_a) 
                    | TxRxAdvAddress::PrivateStatic(adv_a) => {
                        buffer[pdu_size..(pdu_size+adv_a.len())].copy_from_slice(adv_a);
                        pdu_size += adv_a.len();
                    },
                }
                match target_a {
                    TxRxAdvAddress::Public(target_a) 
                    | TxRxAdvAddress::RandomStatic(target_a) 
                    | TxRxAdvAddress::PrivateStatic(target_a) => {
                        buffer[pdu_size..(pdu_size+target_a.len())].copy_from_slice(target_a);
                        pdu_size += target_a.len();
                    },
                }
            },
        }

        // add the gap elements
        let adv_data= match self {
            AdvPdu::AdvInd(_, _, ad_fields)
            | AdvPdu::AdvDirectInd(_, _, _, ad_fields)
            | AdvPdu::AdvNonConnInd(_, ad_fields) => {
                ad_fields.write(&mut buffer[pdu_size..])
            }
        };
        // let advData = self[3].write(&mut buffer[pdu_size..]);
        pdu_size += adv_data.len();

        // set the length field (size - two bytes header)
        buffer[1] = (pdu_size - 2) as u8;
        return &buffer[..pdu_size];
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod AdvPdu_to_buffer {
    // use super::*;

    // #[test]
    // #[should_panic]
    // fn buffer_size_assertion() {
    //     let mut buffer:[u8; crate::ADV_PDU_SIZE_MAX + 1] = [0; crate::ADV_PDU_SIZE_MAX + 1];
    //     let tx_address:AdvAddress = [0;6];
    //     let adv_A = AdvA::Public(tx_address);
    //     let ad_fields = AdFields {..Default::default()};
    //     AdvPdu::ADV_IND(ChSel::Unsupported, &adv_A, &ad_fields).to_buffer(&mut buffer);
    // }


}