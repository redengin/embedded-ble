#![cfg_attr(not(test), no_std)]

use heapless::Vec;

const MAX_GATT_SERVICE_COUNT:usize = 100;
struct GattServer {
    services: Vec<GattService, MAX_GATT_SERVICE_COUNT>,
}

impl GattServer {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }

    pub fn add(&mut self, service:GattService) -> bool {
        if let Some(_) = self.services.iter().position(|s| s.uuid == service.uuid) {
            panic!("attempt to add a GattService with the same uuid")
        }
        self.services.push(service).is_ok()
    }

    pub fn remove(&mut self, uuid:u32) -> Option<GattService> {
        if let Some(index) = self.services.iter().position(|s| s.uuid == uuid) {
            return Some(self.services.swap_remove(index))
        }
        None
    }
}



struct GattService {
    /// https://www.bluetooth.org/docman/handlers/downloaddoc.ashx?doc_id=478726#G25.598629
    uuid: u32,
    service_type: GattServiceType,
}
enum GattServiceType {PRIMARY, SECONDARY}




/// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1070566
struct Advertisement<'a> {
    local_name: Option<&'a [u8]>,
    flags: Option<u8>,

    // service_uuid16_list: Option(uuid),
}

