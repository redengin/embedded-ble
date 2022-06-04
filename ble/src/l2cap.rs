/// https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#G24.1294570
pub struct Channel { cid: u16, }
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
        matches!(self.cid, 0x0002)
    }

    /// https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#G24.565154
    pub fn is_signaling(&self) -> bool {
         matches!(self.cid, 0x0001 | 0x0005)
    }
}

pub struct Builder<'a> {
    channel: Channel,
    buffer: &'a mut [u8],
    payload_length: usize,
    /// connectionless channels use PSM
    psm_length: usize,
}
const MIN_PSM_LENGTH:usize = 2;

impl<'a> Builder<'a> {
    fn new(channel: Channel, buffer:&'a mut [u8]) -> Self {
        // set the frame channel
        buffer[2 .. 4].copy_from_slice(&channel.cid.to_le_bytes());
        Self {
            channel,
            buffer,
            payload_length: 0,
            psm_length: 0,
        }
    }

    fn psm(&'a mut self, psm: &[u8]) -> &'a mut Self {
        if self.channel.is_connectionless() {
            panic!("connectionless channels don't use PSM")
        }
        self.psm_length = psm.len();
        let start_index = 4;
        let end_index = start_index + psm.len();
        self.buffer[start_index .. end_index].copy_from_slice(&psm);
        self
    }

    fn payload(&'a mut self, payload: &[u8]) -> &'a mut Self {
        // copy in the payload
        self.payload_length = payload.len();
        if self.channel.is_connection_oriented() {
            let start_index = 4;
            let end_index = start_index + payload.len();
            self.buffer[start_index .. end_index].copy_from_slice(&payload);
        }
        else {
            if self.psm_length < MIN_PSM_LENGTH {
                panic!("for connectionless channels, the PSM must be set before the payload")
            }
            let start_index = 4 + self.psm_length;
            let end_index = start_index + payload.len();
            self.buffer[start_index .. end_index].copy_from_slice(&payload);
        }

        // set the frame size
        let pdu_length = self.psm_length + self.payload_length;
        if pdu_length > u16::MAX as usize { panic!("packet too large") }
        self.buffer[0 .. 2].copy_from_slice(&(pdu_length as u16).to_le_bytes());
        self
    }

    /// https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#G24.366340
    fn build(&self) -> Result<usize, &'static str> {
        const HEADER_LENGTH:usize = 4;
        let frame_length = HEADER_LENGTH + self.psm_length + self.payload_length;
        Ok(frame_length)
    }
}

// FIXME implement S-frame and I-frame support https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#G24.366399

// FIXME implement K-frame https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#G24.618632


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_connection_oriented() {
        let mut buffer:[u8; 1024] = [0; 1024];
        // test without payload
        {
            let channel = Channel::ATT;
            assert!(channel.is_connection_oriented());
            match Builder::new(channel, &mut buffer).build() {
                Ok(length) => {
                    const HEADER_LENGTH:usize = 4;
                    assert_eq!(HEADER_LENGTH, length);
                    let HEADER:[u8; HEADER_LENGTH] = [0, 0, 4, 0];
                    assert_eq!(HEADER, buffer[.. HEADER_LENGTH]);
                }
                Err(_) => assert!(false),
            };
        }
        // test with payload
        {
            let channel = Channel::ATT;
            let payload:[u8;100] = [0xa5; 100];
            match Builder::new(channel, &mut buffer).payload(&payload).build() {
                Ok(length) => {
                    const HEADER_LENGTH:usize = 4;
                    assert_eq!(HEADER_LENGTH + payload.len(), length);
                    let HEADER:[u8; HEADER_LENGTH] = [100, 0, 4, 0];
                    assert_eq!(HEADER, buffer[.. HEADER_LENGTH]);
                    assert_eq!(payload[HEADER_LENGTH .. payload.len()], buffer[HEADER_LENGTH .. payload.len()]);
                }
                Err(_) => assert!(false),
            };
        }
    }

}