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
        self.is_signaling() ||
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
        buffer[2..3].copy_from_slice(&channel.cid.to_le_bytes());
        Self {
            channel,
            buffer,
            payload_length: 0,
            psm_length: 0,
        }
    }

    fn psm(&'a mut self, psm: &[u8]) -> Result<&'a Self, &'static str> {
        if self.channel.is_connectionless() {
            return Err("connectionless channels don't use PSM")
        }
        self.psm_length = psm.len();
        self.buffer[4..].copy_from_slice(&psm);
        Ok(self)
    }

    fn payload(&'a mut self, payload: &[u8]) -> Result<&'a Self, &'static str> {
        self.payload_length = payload.len();
        if self.channel.is_connection_oriented() {
            self.buffer[4..].copy_from_slice(&payload);
        }
        else {
            if self.psm_length < MIN_PSM_LENGTH {
                return Err("for connectionless channels, the PSM must be set before the payload")
            }
            let index = 4 + self.psm_length;
            self.buffer[index..].copy_from_slice(&payload);
        }

        let pdu_length = if self.channel.is_connection_oriented() {self.payload_length}
                                else {self.psm_length + self.payload_length};
        if pdu_length > u16::MAX as usize { return Err("packet too large") }
        let pdu_length_u16 = pdu_length as u16;
        self.buffer[0..1].copy_from_slice(&pdu_length_u16.to_le_bytes());
        Ok(self)
    }

    /// https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#G24.366340
    fn build(&mut self) -> Result<&[u8], &'static str> {
        Ok(self.buffer)
    }
}



#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_connection_oriented() {

    }

}