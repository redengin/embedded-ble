#![allow(unused)]

/// https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#G24.1294570
pub(crate) struct Channel { cid: u16, }
impl Channel {
    /// null channel identifier. Must not be used.
    pub const NULL: Self = Self{cid:0x0000};

    /// Legacy L2CAP signaling (https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#G24.367213)
    pub const SIGNALING: Self = Self{cid:0x0001};

    /// Legacy Connectionless (https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#G24.374182)
    pub const CONNECTIONLESS: Self = Self{cid:0x0002};

    /// Attribute Protocol (ATT) (https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#M29.9.70316.Part.Part.C).
    pub const ATT: Self = Self{cid:0x0004};

    /// LE L2CAP signaling (https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#G24.367213)
    pub const LE_SIGNALING: Self = Self{cid:0x0005};

    /// LE Security Manager (https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#M31.9.74052.Part.Part.E)
    pub const LE_SECURITY_MANAGER: Self = Self{cid:0x0006};

    /// https://btprodspecificationrefs.blob.core.windows.net/assigned-numbers/Assigned%20Number%20Types/Logical%20Link%20Control.pdf
    pub fn is_assigned(&self) -> bool {
         matches!(self.cid, 0x0020..=0x003E)
    }

    /// https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#G24.619023
    /// https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#G24.926494
    pub fn is_dynamic(&self) -> bool {
         matches!(self.cid, 0x0040..=0x007F)
    }

    /// https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#G24.565154
    pub fn is_connection_oriented(&self) -> bool {
         !self.is_connectionless()
    }

    /// https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#G24.565154
    pub fn is_connectionless(&self) -> bool {
        self.cid == Channel::CONNECTIONLESS.cid
    }

    /// https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#G24.565154
    pub fn is_signaling(&self) -> bool {
        (self.cid == Channel::SIGNALING.cid) || (self.cid == Channel::LE_SIGNALING.cid)
    }
}

struct Builder<'a> {
    channel: &'a Channel,
    buffer: &'a mut [u8],
    payload_length: usize,
    /// connectionless channels use PSM
    psm_length: usize,
}

impl<'a> Builder<'a> {
    fn new(channel: &'a Channel, buffer:&'a mut [u8]) -> Self {
        // set the frame channel
        buffer[2 .. 4].copy_from_slice(&channel.cid.to_le_bytes());
        Self {
            channel,
            buffer,
            payload_length: 0,
            psm_length: 0,
        }
    }

    const MIN_PSM_LENGTH:usize = 2;
    // fn psm(&'a mut self, psm: &[u8]) -> &'a mut Self {
    //     if self.channel.is_connectionless() {
    //         panic!("connectionless channels don't use PSM")
    //     }
    //     else if psm.len() < Builder::MIN_PSM_LENGTH {
    //         panic!("PSM too short")
    //     }
    //     self.psm_length = psm.len();
    //     let start_index = 4;
    //     let end_index = start_index + psm.len();
    //     self.buffer[start_index .. end_index].copy_from_slice(&psm);
    //     self
    // }

    // /// for signaling channels
    // fn L2CAP_COMMAND_REJECT_RSP() -> Result<usize, &`static str> {

    // }

    // fn payload(&'a mut self, payload: &[u8]) -> &'a mut Self {
    //     if self.channel.is_signaling() {
    //         panic!("signaling packets should use command() rather than payload()")
    //     }
    //     // copy in the payload
    //     self.payload_length = payload.len();
    //     if self.channel.is_connection_oriented() {
    //         let start_index = 4;
    //         let end_index = start_index + payload.len();
    //         self.buffer[start_index .. end_index].copy_from_slice(&payload);
    //     }
    //     else {
    //         if self.psm_length < Builder::MIN_PSM_LENGTH {
    //             panic!("for connectionless channels, the PSM must be set before the payload")
    //         }
    //         let start_index = 4 + self.psm_length;
    //         let end_index = start_index + payload.len();
    //         self.buffer[start_index .. end_index].copy_from_slice(&payload);
    //     }

    //     // set the frame size
    //     let pdu_length = self.psm_length + self.payload_length;
    //     if pdu_length > u16::MAX as usize { panic!("packet too large") }
    //     self.buffer[0 .. 2].copy_from_slice(&(pdu_length as u16).to_le_bytes());
    //     self
    // }

    // fn build(&self) -> Result<usize, &'static str> {
    //     const HEADER_LENGTH:usize = 4;
    //     let frame_length = HEADER_LENGTH + self.psm_length + self.payload_length;
    //     Ok(frame_length)
    // }
}

// FIXME implement S-frame and I-frame support https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#G24.366399

// FIXME implement K-frame https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#G24.618632

struct Command {
    code: u8,
    id: u8,
    data: [u8]
}
// https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#G24.376991
// const L2CAP_COMMAND_REJECT_RSP_code:u8 = 0x01;
// const L2CAP_CONNECTION_REQ_code:u8 = 0x02;
// const L2CAP_CONNECTION_RSP_code:u8 = 0x03;
// const L2CAP_CONFIGURATION_REQ_code:u8 = 0x04;
// const L2CAP_CONFIGURATION_RSP_code:u8 = 0x05;
// const L2CAP_DISCONNECTION_REQ_code:u8 = 0x06;
// const L2CAP_DISCONNECTION_RSP_code:u8 = 0x07;
// const L2CAP_ECHO_REQ_code:u8 = 0x08;
// const L2CAP_ECHO_RSP_code:u8 = 0x09;
// pub fn L2CAP_COMMAND_REJECT_RSP(channel:&Channel, id:u8, reason:u16, reason_data:Option<&[u8]>, into:&mut [u8]) -> Result<usize, &'static str> {
//     const MIN_LENGTH:usize = 6;
//     let total_length:usize = MIN_LENGTH + reason_data.map_or(0, |a|a.len());
//     if into.len() < total_length {
//         return Err("`into` buffer too small")
//     }
//     Ok(0)
// }
trait LegacySignaling {
    fn reject_rsp(id:u8, reason:u16, reason_data:Option<&[u8]>, into:&mut [u8]) -> Result<usize, &'static str>;
    fn connection_req(id:u8, reason:u16, reason_data:Option<&[u8]>, into:&mut [u8]) -> Result<usize, &'static str>;
    fn connection_rsq(id:u8, reason:u16, reason_data:Option<&[u8]>, into:&mut [u8]) -> Result<usize, &'static str>;
}
trait BleSignaling {
    fn reject_rsp(id:u8, reason:u16, reason_data:Option<&[u8]>, into:&mut [u8]) -> Result<usize, &'static str>;
}



#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn test_build_connection_oriented() {
    //     let mut buffer:[u8; 1024] = [0; 1024];
    //     let channel = Channel::ATT;
    //     assert!(channel.is_connection_oriented());
    //     // test without payload
    //     match Builder::new(&channel, &mut buffer).build() {
    //         Ok(length) => {
    //             const HEADER_LENGTH:usize = 4;
    //             assert_eq!(HEADER_LENGTH, length);
    //             const HEADER:[u8; HEADER_LENGTH] = [0, 0, 4, 0];
    //             //                                  ^..^ - little endian payload length
    //             //                                        ^..^ - little endian channel CID
    //             assert_eq!(HEADER, buffer[.. HEADER_LENGTH]);
    //         }
    //         Err(_) => assert!(false),
    //     };
    //     // test with payload
    //     let payload:[u8;100] = [0xa5; 100];
    //     match Builder::new(&channel, &mut buffer).payload(&payload).build() {
    //         Ok(length) => {
    //             const HEADER_LENGTH:usize = 4;
    //             assert_eq!(HEADER_LENGTH + payload.len(), length);
    //             const HEADER:[u8; HEADER_LENGTH] = [100, 0, 4, 0];
    //             //                                  ^....^ - little endian payload length
    //             //                                          ^..^ - little endian channel CID
    //             assert_eq!(HEADER, buffer[.. HEADER_LENGTH]);
    //             assert_eq!(payload[HEADER_LENGTH .. payload.len()], buffer[HEADER_LENGTH .. payload.len()]);
    //         }
    //         Err(_) => assert!(false),
    //     };
    // }

}