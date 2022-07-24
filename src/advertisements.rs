// #![cfg_attr(not(test), no_std)]

use crate::gap;

#[derive(Default)]
/// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1070566
/// https://www.novelbits.io/bluetooth-low-energy-advertisements-part-1/
pub struct AdFields<'a> {
    pub local_name: Option<&'a str>,
}

// TODO find BLE specification link
/// https://novelbits.io/bluetooth-low-energy-advertisements-part-1/
#[allow(non_camel_case_types)]
pub enum PDU_TYPE {
    ADV_IND,            // Connectable Scannable Undirected advertising
    ADV_DIRECT_IND,     // Connectable Directed advertising
    ADV_NONCONN_IND,    // Non-Connectable Non-Scannable Undirected advertising
    ADV_SCAN_IND,       // Scannable Undirected advertising
    // Scanning: enables devices to broadcast more advertising data than is allowed in a single advertising packet.
    SCAN_REQ,
    SCAN_RSP,
    // Initiating: establishing a connection between a peripheral device and a central device
    CONNECT_IND,        // this is the connection request packet sent on one of the Primary advertising channels
    // Extending: option to advertise on the Secondary advertising channels in addition to the Primary advertising channels
    ADV_EXT_IND         // used for all advertising types except ADV_IND
}

impl<'a> AdFields<'a> {
    pub fn to_pdu(&'a self, pdu:&mut [u8], pdu_type:PDU_TYPE) -> Result<usize, &'static str>
    {
        let mut pdu_size = 0;

        // add the header
        pdu[0] = pdu_type as u8;  // TODO handle ChSel, TxAdd, RxAdd
        pdu_size += 1;

        // add local_name
        match self.local_name {
            Some(name) => pdu_size += gap::write_local_name(&mut pdu[pdu_size..], name),
            None => {}
        }

        // add the length
        assert!(pdu_size < u8::MAX as usize);
        pdu[1] = pdu_size as u8;
        pdu_size += 1;

        Ok(pdu_size as usize)
    }
}