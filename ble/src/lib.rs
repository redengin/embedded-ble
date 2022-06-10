#![cfg_attr(not(test), no_std)]

pub mod gatt;



/// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1070566
struct Advertisement<'a> {
    local_name: Option<&'a [u8]>,
    flags: Option<u8>,

    // service_uuid16_list: Option(uuid),
}

