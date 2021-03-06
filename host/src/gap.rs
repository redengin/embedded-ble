/// https://btprodspecificationrefs.blob.core.windows.net/assigned-numbers/Assigned%20Number%20Types/Generic%20Access%20Profile.pdf
pub(crate) struct DataTypes;
#[allow(unused)]
impl DataTypes {
    pub const FLAGS:u8 = 0x01;
    pub const INCOMPLETE_LIST_SERVICE_UUID16:u8 = 0x02;
    pub const COMPLETE_LIST_SERVICE_UUID16:u8 = 0x03;
    pub const INCOMPLETE_LIST_SERVICE_UUID32:u8 = 0x04;
    pub const COMPLETE_LIST_SERVICE_UUID32:u8 = 0x05;
    pub const INCOMPLETE_LIST_SERVICE_UUID128:u8 = 0x06;
    pub const COMPLETE_LIST_SERVICE_UUID128:u8 = 0x07;
    pub const SHORTENED_LOCAL_NAME:u8 = 0x08;
    pub const COMPLETE_LOCAL_NAME:u8 = 0x09;
    pub const TX_POWER_LEVEL:u8 = 0x0A;
    pub const CLASS_OF_DEVICE:u8 = 0x0D;
    pub const SIMPLE_PAIRING_HASH_C:u8 = 0x0E;
    pub const SIMPLE_PAIRING_HASH_C192:u8 = 0x0E;
    pub const SIMPLE_PAIRING_RANDOMIZER_R:u8 = 0x0F;
    pub const SIMPLE_PAIRING_RANDOMIZER_R192:u8 = 0x0F;
    pub const DEVICE_ID:u8 = 0x10;
    pub const SECURITY_MANAGER_TK_VALUE:u8 = 0x10;
    pub const SECURITY_MANAGER_OUT_OF_BAND_FLAGS:u8 = 0x11;
    pub const PERIPHERAL_CONNECTION_INTERVAL_RANGE:u8 = 0x12; // currently identified as SLAVE_...
    pub const SOLICITATION_SERVICE_UUID16:u8 = 0x14;
    pub const SOLICITATION_SERVICE_UUID128:u8 = 0x15;
    pub const SERVICE_DATA:u8 = 0x16;
    pub const SERVICE_DATA_UUID16:u8 = 0x16;
    pub const PUBLIC_TARGET_ADDRESS:u8 = 0x17;
    pub const RANDOM_TARGET_ADDRESS:u8 = 0x18;
    pub const APPEARANCE:u8 = 0x19;
    pub const ADVERTISING_INTERVAL:u8 = 0x1A;
    pub const LE_BLUETOOTH_DEVICE_ADDRESS:u8 = 0x1B;
    pub const LE_ROLE:u8 = 0x1C;
    pub const SIMPLE_PAIRING_HASH_C256:u8 = 0x1D;
    pub const SIMPLE_PAIRING_RANDOMIZER_R256:u8 = 0x1E;
    pub const SOLICITATION_SERVICE_UUID32:u8 = 0x1F;
    pub const SERVICE_DATA_UUID32:u8 = 0x20;
    pub const SERVICE_DATA_UUID128:u8 = 0x21;
    pub const LE_SECURE_CONFIRMATION_VALUE:u8 = 0x22;
    pub const LE_SECURE_CONFIRMATION_RANDOM_VALUE:u8 = 0x23;
    pub const URI:u8 = 0x24;
    pub const INDOOR_POSITIONING:u8 = 0x25;
    pub const TRANSPORT_DISCOVERY_DATA:u8 = 0x26;
    pub const LE_SUPPORTED_FEATURES:u8 = 0x27;
    pub const CHANNEL_MAP_UPDATE_INDICATION:u8 = 0x28;
    pub const PB_ADV:u8 = 0x29;
    pub const MESH_MESSAGE:u8 = 0x2A;
    pub const MESH_BEACON:u8 = 0x2B;
    pub const BIG_INFO:u8 = 0x2C;
    pub const BROADCAST_CODE:u8 = 0x2D;
    pub const RESOLVABLE_SET_IDENTIFIER:u8 = 0x2E;
    pub const ADVERTISING_INTERVAL_LONG:u8 = 0x2F;
    pub const THREE_D_INFORMATION_DATA:u8 = 0x3D;
    pub const MANUFACTUER_SPECIFIC_DATA:u8 = 0xFF;
}