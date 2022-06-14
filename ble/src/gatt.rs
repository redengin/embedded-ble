/// Common GATT interface used for both server and client

enum GattServiceType {PRIMARY, SECONDARY}
pub(crate) struct GattService<'a> {
    /// https://www.bluetooth.org/docman/handlers/downloaddoc.ashx?doc_id=478726#G25.598629
    uuid: u32,
    service_type: GattServiceType,
    services: Option<&'a [GattService<'a>]>,
    // FIXME binding traits to enum variants is not yet supported https://github.com/rust-av/rust-av/issues/48
    // workaround: use specific types for each variant to achieve compiler time validation
    // characteristics: &'a [GattCharacteristic<'a>],
    client_readable_characteristics: &'a [ClientReadableGattCharacteristic<'a>],
    client_writable_characteristics: &'a [ClientWritableGattCharacteristic<'a>],
    client_readable_writable_characteristics: &'a [ClientReadableWritableGattCharacteristic<'a>],
}
// FIXME binding traits to enum variants is not yet supported https://github.com/rust-av/rust-av/issues/48
// pub enum GattCharacteristic<'a> {
//     /// is readable by a client (written by the server)
//     ClientReadable(BaseGattCharacteristic<'a>),
//     /// is writable by a client (read by the server)
//     ClientWritable(BaseGattCharacteristic<'a>),
//     /// is readable and writable by both client and server
//     ReadableWritable(BaseGattCharacteristic<'a>),
// }
struct BaseGattCharacteristic<'a> {
    uuid: u16,
    value: &'a mut [u8],
}
const MAX_GATT_VALUE_LENGTH:usize = 512;
struct ClientReadableGattCharacteristic<'a> {
    base: BaseGattCharacteristic<'a>,
}
struct ClientWritableGattCharacteristic<'a> {
    base: BaseGattCharacteristic<'a>,
}
struct ClientReadableWritableGattCharacteristic<'a> {
    base: BaseGattCharacteristic<'a>,
}