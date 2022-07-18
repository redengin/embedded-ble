// #![cfg_attr(not(test), no_std)]


#[derive(Default)]
/// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1070566
/// https://www.novelbits.io/bluetooth-low-energy-advertisements-part-1/
pub struct AdFields<'a> {
    pub local_name: Option<&'a str>,
}

impl<'a> AdFields<'a> {
    pub fn create_pdu(&'a self, buffer:&mut [u8]) -> Result<usize, &'static str>
    {
        todo!()
    }
}