use num_enum::{TryFromPrimitive};
use core::convert::TryFrom;

use rtt_target::rprintln;

/// Core_v5.3.pdf#G41.405690
/// actual max is 258, but most hardware is limited to 255
pub const PDU_SIZE_MAX:usize = 255;
pub type PduBuffer = [u8;PDU_SIZE_MAX];
pub const ADV_PDU_SIZE_MAX:usize = 37;

pub type AccessAddress = u32;
/// Core_v5.3.pdf#G41.455603
pub const ADV_ACCESS_ADDRESS:AccessAddress = 0x8E89BED6_u32.to_le();
pub type CrcInit = u32;
/// Core_v5.3.pdf#G41.453964
pub const ADV_CRCINIT:CrcInit = 0x555555;

/// Core_v5.3.pdf#G41.453964
pub const CRC_POLYNOMIAL:u32 = 0x65B;
/// Core_v5.3.pdf#G41.699341  (Inter Frame Space 150us)
pub const T_IFS_US:u8 = 150;

#[derive(TryFromPrimitive)]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
/// https://www.rfwireless-world.com/Terminology/BLE-Advertising-channels-and-Data-channels-list.html
/// Core_v5.3.pdf#G41.455772
pub enum Channel {
    CH0,  CH1,  CH2,  CH3,  CH4,  CH5,  CH6,  CH7,  CH8,  CH9,
    CH10, CH11, CH12, CH13, CH14, CH15, CH16, CH17, CH18, CH19,
    CH20, CH21, CH22, CH23, CH24, CH25, CH26, CH27, CH28, CH29,
    CH30, CH31, CH32, CH33, CH34, CH35, CH36,
    // advertising channels
    CH37, CH38, CH39
}
/// https://www.rfwireless-world.com/Terminology/BLE-Advertising-channels-and-Data-channels-list.html
/// Core_v5.3.pdf#G41.403618
impl Channel {
    pub fn frequency(&self) -> u8 {
        /// actual frequency (MHz) = 2400 + value
        const FREQUENCIES:[u8;40] = [
            4,  6,  8,  10, 12, 14, 16, 18, 20, 22, /* channels 0-9 */
            24, 28, 30, 32, 34, 36, 38, 40, 42, 44, /* channels 10-19 */
            46, 48, 50, 52, 54, 56, 58, 60, 62, 64, /* channels 20-29 */
            66, 68, 70, 72, 74, 76, 78, 2,  26, 80  /* channels 30-39 */
        ];
        return FREQUENCIES[*self as usize];
    }
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
#[allow(non_camel_case_types)]
/// Core_v5.3.pdf#G41.403922
pub enum PDU_TYPE {
// Advertising - sent by the Link Layer in the Advertising state and received by a Link Layer in the Scanning state or Initiating stat
    /// Connectable Scannable Undirected advertising
    ADV_IND         = 0b0000,
    /// Connectable Directed advertising
    ADV_DIRECT_IND  = 0b0001,
    /// Non-Connectable Non-Scannable Undirected advertising
    ADV_NONCONN_IND = 0b0010,
    /// Scannable Undirected advertising 
    ADV_SCAN_IND    = 0b0110,
    /// aka AUX_ADV_IND, AUX_SYNC_IND, AUX_CHAIN_IND (also used for AUX_SCAN_RSP during scanning)
    ADV_EXT_IND     = 0b0111,
// Scanning - enables devices to broadcast more advertising data than is allowed in a single advertising packet.
    /// aka AUX_SCAN_REQ
    SCAN_REQ        = 0b0011,
    /// aka AUX_SCAN_RSP
    SCAN_RSP        = 0b0100,
// Initiating - establishing a connection between a peripheral device and a central device
    /// connection request (aka AUX_CONNECT_REQ)
    CONNECT_IND     = 0b0101,
    /// connection request response
    AUX_CONNECT_RSP = 0b1000,
}
impl PDU_TYPE {
    /// return the type of a PDU
    pub(crate) fn of(pdu: &[u8]) -> Option<PDU_TYPE> {
        const PDU_TYPE_MASK:u8 = 0b1111;
        return match PDU_TYPE::try_from(pdu[0] & PDU_TYPE_MASK) {
            Ok(pdu_type)  => Some(pdu_type),
            Err(_)                  => None
        }
    }
}

///Core_v5.3.pdf#G41.783004
pub enum ChSel {
    Unsupported = 0,
    Supported = 1,
}

const ADDRESS_LEN:usize = 6;
pub(crate) type Address = [u8;ADDRESS_LEN];
pub enum TxRxAdvAddress {
    Public(Address),
    RandomStatic(Address),
    PrivateStatic(Address),
}
impl TxRxAdvAddress {
    fn write_address(&self, buffer: &mut [u8]) -> usize {
        match self {
            TxRxAdvAddress::Public(address) 
            | TxRxAdvAddress::RandomStatic(address) 
            | TxRxAdvAddress::PrivateStatic(address) => {
                buffer[0..ADDRESS_LEN].copy_from_slice(address);
            }
        };
        return ADDRESS_LEN;
    }
}
/* type aliases to reflect naming in Bluetooth standard for PDUs */
pub type AdvA = TxRxAdvAddress;
// type TargetA = TxRxAdvAddress;
// type InitA = TxRxAdvAddress;
type ScanA = TxRxAdvAddress;
type AdvData<'a> = crate::gap::AdFields<'a>;

pub struct AdvIndPdu<'a> {
    pub ch_sel: ChSel,
    pub adv_a: &'a AdvA,
    pub adv_data: &'a AdvData<'a>,
}
impl<'a> AdvIndPdu<'a> {
    /// returns the used slice of the destination buffer
    pub(crate) fn write(&self, buffer: &'a mut [u8]) -> &'a [u8] {
        let mut pdu_size = 0;

        const TYPE_SHIFT:usize = 0;
        const CHSEL_SHIFT:usize = 5;
        const TXADD_SHIFT:usize = 6;
        buffer[0] = // base pdu type
                    ((PDU_TYPE::ADV_IND as u8) << TYPE_SHIFT)
                    // chSel bit
                    | (match self.ch_sel { ChSel::Supported => 1, _ => 0 } << CHSEL_SHIFT)
                    // txadd bit
                    | (match self.adv_a { TxRxAdvAddress::Public(..) => 0, _ => 1 } << TXADD_SHIFT);
        pdu_size += 1;
        
        // skip a byte for length (will be set at end)
        pdu_size += 1;

        // write the AdvA
        pdu_size += self.adv_a.write_address(&mut buffer[pdu_size..(pdu_size+6)]);

        // append the adv_data
        pdu_size += self.adv_data.write(&mut buffer[pdu_size..]);

        // set the length
        const PDU_HEADER_SIZE:usize = 2;
        buffer[1] = (pdu_size - PDU_HEADER_SIZE) as u8;

        &buffer[0..pdu_size]
    }
}

// TODO pub struct AdvDirectInd

pub struct AdvNonConnIndPdu<'a> {
    pub adv_a: &'a AdvA,
    pub adv_data: &'a AdvData<'a>,
}
impl<'a> AdvNonConnIndPdu<'a> {
    /// returns the used slice of the destination buffer
    pub(crate) fn write(&self, buffer: &'a mut [u8]) -> &'a [u8] {
        let mut pdu_size = 0;

        const TYPE_SHIFT:usize = 0;
        const TXADD_SHIFT:usize = 6;
        buffer[0] = // base pdu type
                    ((PDU_TYPE::ADV_NONCONN_IND as u8) << TYPE_SHIFT)
                    // txadd bit
                    | (match self.adv_a { TxRxAdvAddress::Public(..) => 0, _ => 1 } << TXADD_SHIFT);
        pdu_size += 1;
        
        // skip a byte for length (will be set at end)
        pdu_size += 1;

        // write the AdvA
        pdu_size += self.adv_a.write_address(&mut buffer[pdu_size..(pdu_size+6)]);

        // append the adv_data
        pdu_size += self.adv_data.write(&mut buffer[pdu_size..]);

        // set the length
        const PDU_HEADER_SIZE:usize = 2;
        buffer[1] = (pdu_size - PDU_HEADER_SIZE) as u8;

        &buffer[0..pdu_size]
    }
}

// TODO pub struct AdvScanIndPdu
// TODO pub struct AdvExtIndPdu

pub struct ScanReqPdu<'a> {
    pub scan_a: &'a ScanA,
    pub adv_a: &'a AdvA,
}
#[allow(unused)]
impl<'a> ScanReqPdu<'a> {
    /// returns the used slice of the destination buffer
    pub(crate) fn write(&self, buffer: &'a mut [u8]) -> &'a [u8] {
        let mut pdu_size = 0;

        const TYPE_SHIFT:usize = 0;
        const TXADD_SHIFT:usize = 6;
        buffer[0] = // base pdu type
                    ((PDU_TYPE::SCAN_REQ as u8) << TYPE_SHIFT)
                    // txadd bit
                    | (match self.adv_a { TxRxAdvAddress::Public(..) => 0, _ => 1 } << TXADD_SHIFT);
                    // TODO set the RXADD bit per the AdvA::advertiser address
        pdu_size += 1;
        
        // skip a byte for length (will be set at end)
        pdu_size += 1;

        // append the ScanA
        pdu_size += self.scan_a.write_address(&mut buffer[pdu_size..(pdu_size+6)]);

        // append the AdvA
        pdu_size += self.adv_a.write_address(&mut buffer[pdu_size..(pdu_size+6)]);

        // set the length
        const PDU_HEADER_SIZE:usize = 2;
        buffer[1] = (pdu_size - PDU_HEADER_SIZE) as u8;

        &buffer[0..pdu_size]
    }
}


pub struct ScanRspPdu<'a> {
    pub adv_a: &'a AdvA,
    pub scan_rsp_data: &'a AdvData<'a>,
}
impl<'a> ScanRspPdu<'a> {
    /// returns the used slice of the destination buffer
    pub(crate) fn write(&self, buffer: &'a mut [u8]) -> &'a [u8] {
        let mut pdu_size = 0;

        const TYPE_SHIFT:usize = 0;
        const TXADD_SHIFT:usize = 6;
        buffer[0] = // base pdu type
                    ((PDU_TYPE::SCAN_RSP as u8) << TYPE_SHIFT)
                    // txadd bit
                    | (match self.adv_a { TxRxAdvAddress::Public(..) => 0, _ => 1 } << TXADD_SHIFT);
        pdu_size += 1;
        
        // skip a byte for length (will be set at end)
        pdu_size += 1;

        // write the AdvA
        pdu_size += self.adv_a.write_address(&mut buffer[pdu_size..]);

        // append the adv_data
        pdu_size += self.scan_rsp_data.write(&mut buffer[pdu_size..]);

        // set the length
        const PDU_HEADER_SIZE:usize = 2;
        buffer[1] = (pdu_size - PDU_HEADER_SIZE) as u8;

        &buffer[0..pdu_size]
    }
}

// TODO pub struct ConnectIndPdu
// TODO pub struct AuxConnectRspPdu



// archived code (to be deleted)
// --------------------------------------------------------------------------------

// pub enum AdvPdu<'a> {
//     AdvInd(ChSel, &'a AdvA, &'a AdvData<'a>),
//     AdvDirectInd(ChSel, &'a AdvA, &'a TargetA),
//     AdvNonConnInd(&'a AdvA, &'a AdvData<'a>),
//     // AdvScanInd(&'a AdvA, &'a AdFields<'a>),
//     // AdvExtInd(),
//     AuxAdvInd(&'a AuxAdvExtendedHeader<'a>, &'a AdFields<'a>),

//     // ScanReq(&'a ScanA, &'a AdvA, &'a AdFields<'a>),
//     // ScanRsp(&'a AdvA, &'a AdFields<'a>),
//     // AuxAdvInd()
// }

// #[derive(Default)]
// /// Core_v5.3-5.pdf#G41.686208
// pub struct AuxAdvExtendedHeader<'a> {
//     pub adv_a: Option<&'a AdvA>,
//     pub target_a: Option<&'a TargetA>,
//     pub cte_info: Option<CteInfo>,
//     pub adi: Option<Adi>,
//     pub aux_ptr: Option<AuxPtr>,
//     pub sync_info: Option<SyncInfo>,
//     /// tx power (-127 to 127 dbM)
//     pub txpower: Option<i8>
// }

// /// Core_v5.3-5.pdf#G41.685952
// pub enum AuxAdvMode {
//     NonConnectableNonScannable = 0b00,
//     ConnectableNonScannable = 0b01,
//     NonConnectableScannable = 0b100,
// }

// #[derive(Copy,Clone)]
// /// Core_v5.3-5.pdf#G41.1317593
// pub struct CteInfo {
//     time: u8,   /* 5 bits */
//     cte_type: CteType,
// }

// #[derive(Copy,Clone)]
// enum CteType {
//     // AoAConstantToneExtension = 0,
//     // AoAConstantToneExtensionWith1uSlots = 1,
//     // AoAConstantToneExtensionWith2uSlots = 2,
// }
// /// Core_v5.3-5.pdf#G41.693403
// #[derive(Copy,Clone)]
// pub struct Adi {
//     pub did: u16,   /* 12 bits data ID */
//     pub sid: u8,    /* 4 bits set ID */
// }

// #[derive(Copy,Clone)]
// /// Core_v5.3-5.pdf#G41.693502
// pub struct AuxPtr {
//     channel_index: Channel,
//     ca: ClockAccuracy,       /* clock accuracy */
//     offset_units: OffsetUnits,
//     aux_offset: u16,    /* 13 bits, phase offset */
//     aux_phy: AuxPhy,
// }

// #[derive(Copy,Clone)]
// /// Core_v5.3-5.pdf#G41.694084
// enum ClockAccuracy {
//     // Low     = 0,    /* 51 ppm to 500 ppm */
//     // High    = 1,    /* 0 ppm to 50 ppm */
// }

// #[derive(Copy,Clone)]
// /// Core_v5.3-5.pdf#G41.693560
// enum OffsetUnits {
//     // Small   = 0,    /* 30 us offset */
//     // Large   = 1,    /* 300 us offset */
// }

// #[derive(Copy,Clone)]
// /// Core_v5.3-5.pdf#G41.693876
// enum AuxPhy {
//     // Le1M    = 0b000,
//     // Le2M    = 0b001,
//     // LeCoded = 0b010,
// }

// #[derive(Copy,Clone)]
// /// Core_v5.3-5.pdf#G41.783247
// pub struct SyncInfo {
//     _offset_base: u16,   /* 13 bits */
//     _offset_units: OffsetUnits,
//     /// if true, add 2.4576 seconds to sync period
//     _offset_adjust: bool,
//     /// period (interval * 1.25ms)
//     _interval: u16,
//     /// 37 bit bitmap of channels used
//     _ch_map: u64,
//     /// sleep clock accuracy
//     _sca: Sca,
//     _aa: AccessAddress,
//     _crc_init: [u8;3],
//     /// counter of sync events TODO wtf is this?
//     _periodic_event_counter: u16
// }

// /// Core_v5.3-5.pdf#G41.459735
// #[allow(unused)]
// #[derive(Copy,Clone)]
// enum Sca {
//     Ppm251to500 = 0,
//     Ppm151to250 = 1,
//     Ppm101to150 = 2,
//     Ppm76to100  = 3,
//     Ppm51to75   = 4,
//     Ppm31to50   = 5,
//     Ppm21to30   = 6,
//     Ppm0to20    = 7,
// }


// impl<'a> AdvPdu<'a> {
//     /// writes the data into the buffer and returns the slice of actual data
//     pub(crate) fn write(&self, buffer: &'a mut [u8]) -> &'a [u8]
//     {
//         let mut pdu_size = 0;

//         // note the BLE spec is LSB -> MSB
//         const TYPE_SHIFT:usize = 0;
//         const CHSEL_SHIFT:usize = 5;
//         const TXADD_SHIFT:usize = 6;
//         const RXADD_SHIFT:usize = 7;

//         // set the pdu type
//         buffer[0] = match self {
//             AdvPdu::AdvNonConnInd(adv_a, ..) => {
//                 // base pdu type
//                 ((ADV_PDU_TYPE::ADV_NONCONN_IND as u8) << TYPE_SHIFT)
//                 // txadd bit
//                 | (match adv_a { TxRxAdvAddress::Public(..) => 0, _ => 1 } << TXADD_SHIFT)
//             },
//             // FIXME appears ADV_IND is no longer supported (instead everything uses AUX_ADV_IND)
//             AdvPdu::AdvInd(chsel, adv_a, ..) => {
//                 // base pdu type
//                 ((ADV_PDU_TYPE::ADV_IND as u8) << TYPE_SHIFT)
//                 // chsel bit
//                 | (match chsel { ChSel::Supported => 1, _ => 0 } << CHSEL_SHIFT)
//                 // txadd bit
//                 | (match adv_a { TxRxAdvAddress::Public(..) => 0, _ => 1 } << TXADD_SHIFT)
//             },
//             AdvPdu::AdvDirectInd(chsel, adv_a, target_a, ..) => {
//                 // base pdu type
//                 ((ADV_PDU_TYPE::ADV_DIRECT_IND as u8) << TYPE_SHIFT)
//                 // chsel bit
//                 | (match chsel { ChSel::Supported => 1, _ => 0 } << CHSEL_SHIFT)
//                 // txadd bit
//                 | (match adv_a { TxRxAdvAddress::Public(..) => 0, _ => 1 } << TXADD_SHIFT)
//                 // rxadd bit
//                 | (match target_a { TxRxAdvAddress::Public(..) => 0, _ => 1 } << RXADD_SHIFT)
//             },
//             AdvPdu::AuxAdvInd(..) => {
//                 // base pdu type
//                 (ADV_PDU_TYPE::ADV_EXT_IND as u8) << TYPE_SHIFT
//             }
//         };
//         pdu_size += 1;

//         // skip a byte for length (will be set at end)
//         pdu_size += 1;

//         // set the base pdu data
//         match self {
//             AdvPdu::AdvInd(_, adv_a, ..)
//             | AdvPdu::AdvNonConnInd(adv_a, ..) => {
//                 match adv_a {
//                     TxRxAdvAddress::Public(address) 
//                     | TxRxAdvAddress::RandomStatic(address) 
//                     | TxRxAdvAddress::PrivateStatic(address) => {
//                         buffer[pdu_size..(pdu_size+address.len())].copy_from_slice(address);
//                         pdu_size += address.len();
//                     },
//                 }
//             },
//             AdvPdu::AdvDirectInd(_, adv_a, target_a, ..) => {
//                 match adv_a {
//                     TxRxAdvAddress::Public(address) 
//                     | TxRxAdvAddress::RandomStatic(address) 
//                     | TxRxAdvAddress::PrivateStatic(address) => {
//                         buffer[pdu_size..(pdu_size+address.len())].copy_from_slice(address);
//                         pdu_size += address.len();
//                     },
//                 }
//                 match target_a {
//                     TxRxAdvAddress::Public(address) 
//                     | TxRxAdvAddress::RandomStatic(address) 
//                     | TxRxAdvAddress::PrivateStatic(address) => {
//                         buffer[pdu_size..(pdu_size+address.len())].copy_from_slice(address);
//                         pdu_size += address.len();
//                     },
//                 }
//             },
//             AdvPdu::AuxAdvInd(header, ..) => {
//                 // write the extended header
//                 pdu_size += header.write(AuxAdvMode::ConnectableNonScannable, &mut buffer[pdu_size..]);
//             }
//         }

//         // add the gap elements (AdvData)
//         match self {
//             AdvPdu::AdvInd(_, _, adv_data)
//             | AdvPdu::AdvNonConnInd(_, adv_data)
//             | AdvPdu::AuxAdvInd(_, adv_data) => {
//                 pdu_size += adv_data.write(&mut buffer[pdu_size..]);
//             }
//             AdvPdu::AdvDirectInd(..) => { /* advData not supported */ }
//         };

//         // set the length field
//         const PDU_HEADER_SIZE:usize = 2;
//         buffer[1] = (pdu_size - PDU_HEADER_SIZE) as u8;
//         return &buffer[..pdu_size];
//     }
// }

// impl<'a> AuxAdvExtendedHeader<'a> {
//     fn write(&'a self, mode: AuxAdvMode, buffer: &'a mut [u8]) -> usize
//     {
//         let mut header_size: usize = 0;

//         // skip first byte (length and mode) - will be set at end
//         header_size += 1;


//         // set the extended header flags
//         const ADVA_SHIFT:usize = 0;
//         const TARGETA_SHIFT:usize = 1;
//         const CTEINFO_SHIFT:usize = 2;
//         const ADI_SHIFT:usize = 3;
//         const AUXPTR_SHIFT:usize = 4;
//         const SYNCINFO_SHIFT:usize = 5;
//         const TXPOWER_SHIFT:usize = 6;
//         buffer[header_size] =
//               (match self.adv_a { Some(_) => 1, _ => 0 } << ADVA_SHIFT)
//             | (match self.target_a { Some(_) => 1, _ => 0 } << TARGETA_SHIFT)
//             | (match self.cte_info { Some(_) => 1, _ => 0 } << CTEINFO_SHIFT)
//             | (match self.adi { Some(_) => 1, _ => 0 } << ADI_SHIFT)
//             | (match self.aux_ptr { Some(_) => 1, _ => 0 } << AUXPTR_SHIFT)
//             | (match self.sync_info { Some(_) => 1, _ => 0 } << SYNCINFO_SHIFT)
//             | (match self.txpower { Some(_) => 1, _ => 0 } << TXPOWER_SHIFT);
//         header_size += 1;

//         // (optional) set AdvA
//         match self.adv_a {
//             Some(adv_a) => {
//                 match adv_a {
//                     TxRxAdvAddress::Public(address) 
//                     | TxRxAdvAddress::RandomStatic(address) 
//                     | TxRxAdvAddress::PrivateStatic(address) => {
//                         buffer[header_size..(header_size+address.len())].copy_from_slice(address);
//                         header_size += address.len();
//                     }
//                 }
//             }
//             None => {}
//         }

//         // (optional) set TargetA
//         match self.target_a {
//             Some(target_a) => {
//                 match target_a {
//                     TxRxAdvAddress::Public(address) 
//                     | TxRxAdvAddress::RandomStatic(address) 
//                     | TxRxAdvAddress::PrivateStatic(address) => {
//                         buffer[header_size..(header_size+address.len())].copy_from_slice(address);
//                         header_size += address.len();
//                     }
//                 }
//             }
//             None => {}
//         }

//         // (optional) set CTEInfo
//         match self.cte_info {
//             Some(cte_info) => {
//                 const TIME_SHIFT:usize = 3;
//                 const TYPE_SHIFT:usize = 0;
//                 buffer[header_size] =
//                       (cte_info.time << TIME_SHIFT)
//                     | ((cte_info.cte_type as u8) << TYPE_SHIFT);
//                 header_size += 1;
//             }
//             None => {}
//         }

//         // (optional) set AdvDataInfo (ADI)
//         match self.adi {
//             Some(adi) => {
//                 buffer[header_size] = (adi.did >> 4) as u8;
//                 header_size += 1;
//                 buffer[header_size] =
//                       (adi.did << 4) as u8
//                     | adi.sid;
//                 header_size += 1;
//             }
//             None => {}
//         }

//         // (optional) set AuxPtr
//         match self.aux_ptr {
//             Some(aux_ptr) => {
//                 const CHANNEL_SHIFT:usize = 2;
//                 const CA_SHIFT:usize = 1;
//                 const OFFSETUNITS_SHIFT:usize = 0;
//                 buffer[header_size] =
//                       ((aux_ptr.channel_index as u8) << CHANNEL_SHIFT)
//                     | ((aux_ptr.ca as u8) << CA_SHIFT)
//                     | ((aux_ptr.offset_units as u8) << OFFSETUNITS_SHIFT);
//                 header_size += 1;
//                 buffer[header_size] = (aux_ptr.aux_offset >> 5) as u8;
//                 header_size += 1;
//                 buffer[header_size] =
//                       ((aux_ptr.aux_offset << 3) as u8)
//                     | (aux_ptr.aux_phy as u8);
//                 header_size += 1;
//             }
//             None => {}
//         }

//         // (optional) set SyncInfo
//         match self.sync_info {
//             Some(_) => {
//                 panic!("not implemented");
//             }
//             None => {}
//         }

//         // (optional) set TxPower
//         match self.txpower {
//             Some(tx_power) => {
//                 buffer[header_size] = tx_power as u8;
//                 header_size += 1;
//             }
//             None => {}
//         }

//         // (optional) set ACAD
//         // FIXME not implemented

//         // set the header length and AdvMode
//         const LENGTH_SHIFT: usize = 6;
//         const ADV_MODE_SHIFT: usize = 0;
//         buffer[0] = (((header_size - 1) as u8) << LENGTH_SHIFT)
//                     | ((mode as u8) << ADV_MODE_SHIFT);
        
//         return header_size;
//     }
// }

// #[cfg(test)]
// #[allow(non_snake_case)]
// mod AdvPdu_to_buffer {
//     use super::*;

//     const ADV_PDU_SIZE_MAX:usize = 39;
//     const ADV_PDU_HEADER_SIZE:usize = 2;

//     const ADVA_PUBLIC:AdvA = AdvA::Public([0, 0, 0, 0, 0, 0]);
//     // const ADVA_RANDOM:AdvA = AdvA::RandomStatic([0, 0, 0, 0, 0, 0]);
//     // const TARGETA_PUBLIC:TargetA = TargetA::Public([0, 0, 0, 0, 0, 0]);
//     // const TARGETA_RANDOM:TargetA = TargetA::RandomStatic([0, 0, 0, 0, 0, 0]);

//     // #[test]
//     // fn adv_ind_public() {
//     //     let empty_ad_fields:AdFields = AdFields{..Default::default()};
//     //     let mut buffer:[u8; ADV_PDU_SIZE_MAX] = [0; ADV_PDU_SIZE_MAX];
//     //     let pdu = AdvPdu::AdvInd(ChSel::Unsupported, &ADVA_PUBLIC, &empty_ad_fields).to_buffer(&mut buffer);
//     //     const ADVA_SIZE:usize = 6;
//     //     assert_eq!(ADV_PDU_HEADER_SIZE + ADVA_SIZE, pdu.len());  // pdu size
//     //     assert_eq!(ADV_PDU_TYPE::ADV_IND as u8, pdu[0]); // pdu type
//     //     assert_eq!(ADVA_SIZE as u8, pdu[1]);  // pdu size
//     // }
//     // #[test]
//     // fn adv_ind_chsel_public() {
//     //     let empty_ad_fields:AdFields = AdFields{..Default::default()};
//     //     let mut buffer:[u8; ADV_PDU_SIZE_MAX] = [0; ADV_PDU_SIZE_MAX];
//     //     let pdu = AdvPdu::AdvInd(ChSel::Supported, &ADVA_PUBLIC, &empty_ad_fields).to_buffer(&mut buffer);
//     //     assert_eq!(ADV_PDU_TYPE::ADV_IND as u8, pdu[0]); // pdu type
//     //     assert_eq!(96, pdu[0]); // pdu type
//     //     assert_eq!(ADV_PDU_HEADER_SIZE, pdu[1]);  // pdu size
//     // }
//     // #[test]
//     // fn adv_ind_random() {
//     //     let mut buffer:[u8; ADV_PDU_SIZE_MAX + 1] = [0; ADV_PDU_SIZE_MAX + 1];

//     //     let empty_ad_fields:AdFields = AdFields{..Default::default()};
//     //     let pdu = AdvPdu::AdvInd(ChSel::Unsupported, &ADVA_RANDOM, &empty_ad_fields).to_buffer(&mut buffer);
//     //     assert_eq!(0, pdu[0]); // pdu type
//     //     assert_eq!(6, pdu[1]);  // pdu size
//     // }
//     // #[test]
//     // fn adv_ind_chsel_random() {
//     //     let mut buffer:[u8; ADV_PDU_SIZE_MAX + 1] = [0; ADV_PDU_SIZE_MAX + 1];

//     //     let empty_ad_fields:AdFields = AdFields{..Default::default()};
//     //     let pdu = AdvPdu::AdvInd(ChSel::Supported, &ADVA_RANDOM, &empty_ad_fields).to_buffer(&mut buffer);
//     //     assert_eq!(32, pdu[0]); // pdu type
//     //     assert_eq!(6, pdu[1]);  // pdu size
//     // }
//     // #[test]
//     // fn adv_directind_public_public() {
//     //     let mut buffer:[u8; ADV_PDU_SIZE_MAX + 1] = [0; ADV_PDU_SIZE_MAX + 1];

//     //     let empty_ad_fields:AdFields = AdFields{..Default::default()};
//     //     let pdu = AdvPdu::AdvDirectInd(ChSel::Unsupported, &ADVA_PUBLIC, &TARGETA_PUBLIC, &empty_ad_fields).to_buffer(&mut buffer);
//     //     assert_eq!(193, pdu[0]); // pdu type
//     //     assert_eq!(12, pdu[1]);  // pdu size
//     // }
//     // #[test]
//     // fn adv_directind_random_public() {
//     //     let mut buffer:[u8; ADV_PDU_SIZE_MAX + 1] = [0; ADV_PDU_SIZE_MAX + 1];

//     //     let empty_ad_fields:AdFields = AdFields{..Default::default()};
//     //     let pdu = AdvPdu::AdvDirectInd(ChSel::Unsupported, &ADVA_RANDOM, &TARGETA_PUBLIC, &empty_ad_fields).to_buffer(&mut buffer);
//     //     assert_eq!(129, pdu[0]); // pdu type
//     //     assert_eq!(12, pdu[1]);  // pdu size
//     // }
//     // #[test]
//     // fn adv_directind_public_random() {
//     //     let mut buffer:[u8; ADV_PDU_SIZE_MAX + 1] = [0; ADV_PDU_SIZE_MAX + 1];

//     //     let empty_ad_fields:AdFields = AdFields{..Default::default()};
//     //     let pdu = AdvPdu::AdvDirectInd(ChSel::Unsupported, &ADVA_PUBLIC, &TARGETA_RANDOM, &empty_ad_fields).to_buffer(&mut buffer);
//     //     assert_eq!(65, pdu[0]); // pdu type
//     //     assert_eq!(12, pdu[1]);  // pdu size
//     // }
//     // #[test]
//     // fn adv_directind_random_random() {
//     //     let mut buffer:[u8; ADV_PDU_SIZE_MAX + 1] = [0; ADV_PDU_SIZE_MAX + 1];

//     //     let empty_ad_fields:AdFields = AdFields{..Default::default()};
//     //     let pdu = AdvPdu::AdvDirectInd(ChSel::Unsupported, &ADVA_RANDOM, &TARGETA_RANDOM, &empty_ad_fields).to_buffer(&mut buffer);
//     //     assert_eq!(1, pdu[0]); // pdu type
//     //     assert_eq!(12, pdu[1]);  // pdu size
//     // }
//     // #[test]
//     // fn adv_nonconnind_public() {
//     //     let mut buffer:[u8; ADV_PDU_SIZE_MAX + 1] = [0; ADV_PDU_SIZE_MAX + 1];

//     //     let empty_ad_fields:AdFields = AdFields{..Default::default()};
//     //     let pdu = AdvPdu::AdvNonConnInd(&ADVA_PUBLIC, &empty_ad_fields).to_buffer(&mut buffer);
//     //     assert_eq!(66, pdu[0]); // pdu type
//     //     assert_eq!(6, pdu[1]);  // pdu size
//     // }
//     // #[test]
//     // fn adv_nonconnind_random() {
//     //     let mut buffer:[u8; ADV_PDU_SIZE_MAX + 1] = [0; ADV_PDU_SIZE_MAX + 1];

//     //     let empty_ad_fields:AdFields = AdFields{..Default::default()};
//     //     let pdu = AdvPdu::AdvNonConnInd(&ADVA_RANDOM, &empty_ad_fields).to_buffer(&mut buffer);
//     //     assert_eq!(2, pdu[0]); // pdu type
//     //     assert_eq!(6, pdu[1]);  // pdu size
//     // }
// }
