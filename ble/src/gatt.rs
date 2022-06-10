struct GattServer<'a> {
    services: &'a [GattService<'a>],
}

enum GattServiceType {PRIMARY, SECONDARY}
struct GattService<'a> {
    /// https://www.bluetooth.org/docman/handlers/downloaddoc.ashx?doc_id=478726#G25.598629
    uuid: u32,
    service_type: GattServiceType,
    services: Option<&'a [GattService<'a>]>,
    characteristics: &'a [GattCharacteristic<'a>],
}

pub enum GattCharacteristic<'a> {
    /// is readable by a client (written by the server)
    ClientReadable(BaseGattCharacteristic<'a>),
    /// is writable by a client (read by the server)
    ClientWritable(BaseGattCharacteristic<'a>),
    /// is readable and writable by both client and server
    ReadableWritable(BaseGattCharacteristic<'a>),
}
pub struct BaseGattCharacteristic<'a> {
    uuid: u16,
    value: &'a [u8],
}
trait Read {
    fn read() -> usize;
}

const MAX_GATT_VALUE_LENGTH:usize = 512;
impl Read for ClientReadable {
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gatt_server() {
        let gatt_server = GattServer {
            services: &[
                GattService {
                    uuid: 0,
                    service_type: GattServiceType::PRIMARY,
                    characteristics: &[
                        GattCharacteristic::Readable(BaseGattCharacteristic{
                            uuid: 0,
                            value: &[0,100],
                        }),
                    ],
                    services: None,
                },
            ],
        };
    }
}