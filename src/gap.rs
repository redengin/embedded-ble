const PDU_ADV_STRUCTURE_LENGTH_SIZE:usize = 1;

#[derive(Default)]
/// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1070566
/// https://www.novelbits.io/bluetooth-low-energy-advertisements-part-1/
pub struct AdFields<'a> {
    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999520
    pub incomplete_list_service_uuid_16: Option<&'a [u16]>,
    pub complete_list_service_uuid_16: Option<&'a [u16]>,
    pub incomplete_list_service_uuid_32: Option<&'a [u32]>,
    pub complete_list_service_uuid_32: Option<&'a [u32]>,
    pub incomplete_list_service_uuid_128: Option<&'a [u128]>,
    pub complete_list_service_uuid_128: Option<&'a [u128]>,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1004814
    pub local_name: Option<&'a str>,
    pub short_name: Option<&'a str>,

    /// see enum Flags (https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999589)
    pub flags: Option<u8>,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999668
    pub manufacturer_specific_data: Option<&'a [u8]>,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999686
    pub tx_power_level: Option<i8>,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999709
    // TODO SECURE SIMPLE PAIRING OUT OF BAND
    pub _secure_simple_pairing_oob: u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999768
    // TODO SECURITY MANAGER OUT OF BAND
    pub _security_manager_oob: u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999818
    // TODO SECURITY MANAGER TK VALUE
    pub _security_manager_tk_value: u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999838
    // TODO PERIPHERAL CONNECTION INTERVAL RANGE
    pub _peripheral_connection_interval_range: u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999871
    // TODO SERVICE SOLICITATION
    pub _service_solicitation: u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999894
    // TODO SERVICE DATA
    pub _service_data: u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999913
    /// https://specificationrefs.bluetooth.com/assigned-values/Appearance%20Values.pdf
    pub appearance: Option<u16>,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999932
    // TODO PUBLIC TARGET ADDRESS
    pub _public_target_address: u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999953
    // TODO RANDOM TARGET ADDRESS
    pub _random_target_address:u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1004048
    // TODO ADVERTISING INTERVAL
    pub _advertising_interval:u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1005265
    pub le_bluetooth_device_address: Option<&'a LeBluetoothDeviceAddress>,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1005342
    pub le_role: Option<LeRole>,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1005559
    pub uri: Option<&'a str>,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1054608
    // TODO LE SUPPORTED FEATURES
    pub _le_supported_features: u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1055008
    // TODO CHANNEL MAP UPDATE INDICATION
    pub _channel_map_update_indication: u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1176955
    // TODO BIGINFO
    pub _biginfo: u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1177011
    // TODO BROADCAST_CODE
    pub _broadcast_code: u8,
}

impl<'a> AdFields<'a> {
    /// places ad structures as long as they will fit in packet
    pub fn write(&'a self, buffer: &'a mut [u8]) -> usize
    {
        let mut ad_size = 0;

        // specification allows this to be larger than one-byte, but only
        //      single byte types are currently defined
        const AD_TYPE_SIZE:usize = 1;

        // add incomplete service uuid 16 list
        match self.incomplete_list_service_uuid_16 {
            Some(uuids) => if buffer.len() >= (ad_size + PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + (2 * uuids.len())) {
                // set ad structure length
                buffer[ad_size] = (AD_TYPE_SIZE + (2 * uuids.len())) as u8;
                ad_size += 1;
                // set ad structure type
                buffer[ad_size] = DataTypes::Incomplete16bitServiceUuids as u8;
                ad_size += 1;
                // set ad structure payload
                for uuid in uuids {
                    buffer[ad_size..(ad_size + 2)].copy_from_slice(&uuid.to_le_bytes());
                    ad_size += 2;
                }
            }
            None => {}
        }
        // add complete service uuid 16 list
        match self.complete_list_service_uuid_16 {
            Some(uuids) => if buffer.len() >= (ad_size + PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + (2 * uuids.len())) {
                // set ad structure length
                buffer[ad_size] = (AD_TYPE_SIZE + (2 * uuids.len())) as u8;
                ad_size += 1;
                // set ad structure type
                buffer[ad_size] = DataTypes::Complete16bitServiceUuids as u8;
                ad_size += 1;
                // set ad structure payload
                for uuid in uuids {
                    buffer[ad_size..(ad_size + 2)].copy_from_slice(&uuid.to_le_bytes());
                    ad_size += 2;
                }
            }
            None => {}
        }
        // add incomplete service uuid 32 list
        match self.incomplete_list_service_uuid_32 {
            Some(uuids) => if buffer.len() >= (ad_size + PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + (4 * uuids.len())) {
                // set ad structure length
                buffer[ad_size] = (AD_TYPE_SIZE + (4 * uuids.len())) as u8;
                ad_size += 1;
                // set ad structure type
                buffer[ad_size] = DataTypes::Incomplete32bitServiceUuids as u8;
                ad_size += 1;
                // set ad structure payload
                for uuid in uuids {
                    buffer[ad_size..(ad_size + 4)].copy_from_slice(&uuid.to_le_bytes());
                    ad_size += 4;
                }
            }
            None => {}
        }
        // add complete service uuid 32 list
        match self.complete_list_service_uuid_32 {
            Some(uuids) => if buffer.len() >= (ad_size + PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + (4 * uuids.len())) {
                // set ad structure length
                buffer[ad_size] = (AD_TYPE_SIZE + (4 * uuids.len())) as u8;
                ad_size += 1;
                // set ad structure type
                buffer[ad_size] = DataTypes::Complete32bitServiceUuids as u8;
                ad_size += 1;
                // set ad structure payload
                for uuid in uuids {
                    buffer[ad_size..(ad_size + 4)].copy_from_slice(&uuid.to_le_bytes());
                    ad_size += 4;
                }
            }
            None => {}
        }
        // add incomplete service uuid 128 list
        match self.incomplete_list_service_uuid_128 {
            Some(uuids) => if buffer.len() >= (ad_size + PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + (16 * uuids.len())) {
                // set ad structure length
                buffer[ad_size] = (AD_TYPE_SIZE + (16 * uuids.len())) as u8;
                ad_size += 1;
                // set ad structure type
                buffer[ad_size] = DataTypes::Incomplete128bitServiceUuids as u8;
                ad_size += 1;
                // set ad structure payload
                for uuid in uuids {
                    buffer[ad_size..(ad_size + 16)].copy_from_slice(&uuid.to_le_bytes());
                    ad_size += 16;
                }
            }
            None => {}
        }
        // add complete service uuid 128 list
        match self.complete_list_service_uuid_128 {
            Some(uuids) => if buffer.len() >= (ad_size + PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + (16 * uuids.len())) {
                // set ad structure length
                buffer[ad_size] = (AD_TYPE_SIZE + (16 * uuids.len())) as u8;
                ad_size += 1;
                // set ad structure type
                buffer[ad_size] = DataTypes::Complete128bitServiceUuids as u8;
                ad_size += 1;
                // set ad structure payload
                for uuid in uuids {
                    buffer[ad_size..(ad_size + 16)].copy_from_slice(&uuid.to_le_bytes());
                    ad_size += 16;
                }
            }
            None => {}
        }

        // add local_name
        match self.local_name {
            Some(name) => if buffer.len() >= (ad_size + PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + name.len()) {
                // set ad structure length
                buffer[ad_size] = (AD_TYPE_SIZE + name.len()) as u8;
                ad_size += 1;
                // set ad structure type
                buffer[ad_size] = DataTypes::CompleteLocalName as u8;
                ad_size += 1;
                // set ad structure payload
                buffer[ad_size..(ad_size + name.len())].copy_from_slice(name.as_bytes());
                ad_size += name.len();
            }
            None => {}
        }
        // add short_name
        match self.short_name {
            Some(name) => if buffer.len() >= (ad_size + PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + name.len()) {
                // set ad structure length
                buffer[ad_size] = (AD_TYPE_SIZE + name.len()) as u8;
                ad_size += 1;
                // set ad structure type
                buffer[ad_size] = DataTypes::ShortenedLocalName as u8;
                ad_size += 1;
                // set ad structure payload
                buffer[ad_size..(ad_size + name.len())].copy_from_slice(name.as_bytes());
                ad_size += name.len();
            }
            None => {}
        }
        // add flags
        match self.flags {
            Some(flags) => if buffer.len() >= (ad_size + PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + 1) {
                // set ad structure length
                buffer[ad_size] = (AD_TYPE_SIZE + 1) as u8;
                ad_size += 1;
                // set ad structure type
                buffer[ad_size] = DataTypes::Flags as u8;
                ad_size += 1;
                // set ad structure payload
                buffer[ad_size] = flags as u8;
                ad_size += 1;
            }
            None => {}
        }
        // add manufacturer data
        match self.manufacturer_specific_data {
            Some(data) => if buffer.len() >= (ad_size + PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + data.len()) {
                // manufacturer data must have 2 byte company identifier to be valid
                assert!(data.len() >= 2);
                // set ad structure length
                buffer[ad_size] = (AD_TYPE_SIZE + data.len()) as u8;
                ad_size += 1;
                // set ad structure type
                buffer[ad_size] = DataTypes::ManufacturerSpecificData as u8;
                ad_size += 1;
                // set ad structure payload
                buffer[ad_size..(ad_size + data.len())].copy_from_slice(data);
                ad_size += data.len();
            }
            None => {}
        }
        // add tx_power_level
        match self.tx_power_level {
            Some(level) => if buffer.len() >= (ad_size + PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + 1) {
                // set ad structure length
                buffer[ad_size] = (AD_TYPE_SIZE + 1) as u8;
                ad_size += 1;
                // set ad structure type
                buffer[ad_size] = DataTypes::TxPowerLevel as u8;
                ad_size += 1;
                // set ad structure payload
                buffer[ad_size] = level as u8;
                ad_size += 1;
            }
            None => {}
        }
        // add appearance
        match self.appearance {
            Some(id) => if buffer.len() >= (ad_size + PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + 2) {
                // set ad structure length
                buffer[ad_size] = (AD_TYPE_SIZE + 2) as u8;
                ad_size += 1;
                // set ad structure type
                buffer[ad_size] = DataTypes::Appearance as u8;
                ad_size += 1;
                // set ad structure payload
                buffer[ad_size..(ad_size + 2)].copy_from_slice(&id.to_le_bytes());
                ad_size += 2;
            }
            None => {}
        }
        // add le device address
        match self.le_bluetooth_device_address {
            Some(address) => if buffer.len() >= (ad_size + PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + address.len()) {
                // set ad structure length
                buffer[ad_size] = (AD_TYPE_SIZE + address.len()) as u8;
                ad_size += 1;
                // set ad structure type
                buffer[ad_size] = DataTypes::LeBluetoothDeviceAddress as u8;
                ad_size += 1;
                // set ad structure payload
                buffer[ad_size..(ad_size + address.len())].copy_from_slice(address);
                ad_size += address.len();
            }
            None => {}
        }
        // add le role
        match self.le_role{
            Some(role) => if buffer.len() >= (ad_size + PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + 1) {
                // set ad structure length
                buffer[ad_size] = (AD_TYPE_SIZE + 1) as u8;
                ad_size += 1;
                // set ad structure type
                buffer[ad_size] = DataTypes::LeRole as u8;
                ad_size += 1;
                // set ad structure payload
                buffer[ad_size] = role as u8;
                ad_size += 1;
            }
            None => {}
        }
        // add uri
        match self.uri {
            Some(uri) => if buffer.len() >= (ad_size + PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + uri.len()) {
                // set ad structure length
                buffer[ad_size] = (AD_TYPE_SIZE + uri.len()) as u8;
                ad_size += 1;
                // set ad structure type
                buffer[ad_size] = DataTypes::Uri as u8;
                ad_size += 1;
                // set ad structure payload
                buffer[ad_size..(ad_size + uri.len())].copy_from_slice(uri.as_bytes());
                ad_size += uri.len();
            }
            None => {}
        }

        return ad_size;
    }
}

/// https://btprodspecificationrefs.blob.core.windows.net/assigned-numbers/Assigned%20Number%20Types/Generic%20Access%20Profile.pdf
#[allow(unused)]
enum DataTypes {
    Flags                           = 0x01,
    Incomplete16bitServiceUuids     = 0x02,
    Complete16bitServiceUuids       = 0x03,
    Incomplete32bitServiceUuids     = 0x04,
    Complete32bitServiceUuids       = 0x05,
    Incomplete128bitServiceUuids    = 0x06,
    Complete128bitServiceUuids      = 0x07,
    ShortenedLocalName              = 0x08,
    CompleteLocalName               = 0x09,
    TxPowerLevel                    = 0x0A,
    ClassOfDevice                   = 0x0D,
    SimplePairingHashC              = 0x0E,
    SimplePairingRandomizer         = 0x0F,
    SecurityManagerTkValue          = 0x10,
    SecurityManagerOutOfBandFlags   = 0x11,
    SlaveConnectionIntervalRange    = 0x12,
    List16bitServiceSolicitation    = 0x14,
    List128bitServiceSolicitation   = 0x15,
    ServiceDataUuid16               = 0x16,
    PublicTargetAddress             = 0x17,
    RandomTargetAddress             = 0x18,
    Appearance                      = 0x19,
    AdvertisingInterval             = 0x1A,
    LeBluetoothDeviceAddress        = 0x1B,
    LeRole                          = 0x1C,
    SimplePairingHashC56            = 0x1D,
    SimplePairingRandomizerR256     = 0x1E,
    List32bitServiceSolicitation    = 0x1F,
    ServiceData32bitUuid            = 0x20,
    ServiceData128bitUuid           = 0x21,
    LeSecureConnectionsConfirmValue = 0x22,
    LeSecureConnectionsRandomValue  = 0x23,
    Uri                             = 0x24,
    IndoorPositioning               = 0x25,
    TransportDiscoveryData          = 0x26,
    LeSupportedFeatures             = 0x27,
    ChannelMapUpdateIndication      = 0x28,
    PbAdv                           = 0x29,
    MeshMessage                     = 0x2A,
    MeshBeacon                      = 0x2B,
    BIGInfo                         = 0x2C,
    BroadCastCode                   = 0x2D,
    ResolvableSetIdentifier         = 0x2E,
    AdvertisingIntervalLong         = 0x2F,
    BroadcastName                   = 0x30,
    ThreeDInformation               = 0x3D,
    ManufacturerSpecificData        = 0xFF,
}

/// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999589
pub const FLAGS_LE_LIMITED_DISCOVERABLE:u8      = 1 << 0;
pub const FLAGS_LE_GENERAL_DISCOVERABLE:u8      = 1 << 1;
pub const FLAGS_BR_EDR_NOT_SUPPORTED:u8         = 1 << 2;
pub const FLAGS_SIMULTANEOUS_LE_AND_BR_EDR:u8   = 1 << 3;

/// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1005365
type LeBluetoothDeviceAddress = [u8;7];

/// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1005585
#[derive(Copy, Clone)]
pub enum LeRole {
    OnlyPeripheralRole                  = 0x00,
    OnlyCentralRole                     = 0x01,
    /// peripheral role preferred
    PeripheralAndCentralRolePeripheral  = 0x02,
    /// central role preferred
    PeripheralAndCentralRoleCentral     = 0x03,
}

#[cfg(test)]
mod adfields_write {
    use crate::link_layer;

    use super::*;

    const ADV_PDU_SIZE_MAX:usize = link_layer::PDU_SIZE_MAX;
    const AD_TYPE_SIZE:usize = 1; // BLE specification is ambiguous, but current standard only supports this

    #[test]
    fn incomplete_list_service_uuid_16() {
        let service_uuids:[u16;1] = [0xA55A];
        let ad_fields = AdFields{ incomplete_list_service_uuid_16:Some(&service_uuids), ..AdFields::default() };
        let mut buffer:[u8; ADV_PDU_SIZE_MAX] = [0; ADV_PDU_SIZE_MAX];
        let adv_data= ad_fields.write(&mut buffer);
        assert_eq!((PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + (2 * service_uuids.len())), adv_data.len());
        assert_eq!((adv_data.len() - 1), adv_data[0] as usize);
        assert_eq!(DataTypes::Incomplete16bitServiceUuids as u8, adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE]);
        assert_eq!(service_uuids[0].to_le_bytes(), adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE..]);
    }
    #[test]
    fn complete_list_service_uuid_16() {
        let service_uuids:[u16;1] = [0xA55A];
        let ad_fields = AdFields{ complete_list_service_uuid_16:Some(&service_uuids), ..AdFields::default() };
        let mut buffer:[u8; ADV_PDU_SIZE_MAX] = [0; ADV_PDU_SIZE_MAX];
        let adv_data= ad_fields.write(&mut buffer);
        assert_eq!((PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + (2 * service_uuids.len())), adv_data.len());
        assert_eq!((adv_data.len() - 1), adv_data[0] as usize);
        assert_eq!(DataTypes::Complete16bitServiceUuids as u8, adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE]);
        assert_eq!(service_uuids[0].to_le_bytes(), adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE..]);
    }
    #[test]
    fn incomplete_list_service_uuid_32() {
        let service_uuids:[u32;1] = [0xA55A5AA5];
        let ad_fields = AdFields{ incomplete_list_service_uuid_32:Some(&service_uuids), ..AdFields::default() };
        let mut buffer:[u8; ADV_PDU_SIZE_MAX] = [0; ADV_PDU_SIZE_MAX];
        let adv_data= ad_fields.write(&mut buffer);
        assert_eq!((PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + (4 * service_uuids.len())), adv_data.len());
        assert_eq!((adv_data.len() - 1), adv_data[0] as usize);
        assert_eq!(DataTypes::Incomplete32bitServiceUuids as u8, adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE]);
        assert_eq!(service_uuids[0].to_le_bytes(), adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE..]);
    }
    #[test]
    fn complete_list_service_uuid_32() {
        let service_uuids:[u32;1] = [0xA55A5AA5];
        let ad_fields = AdFields{ complete_list_service_uuid_32:Some(&service_uuids), ..AdFields::default() };
        let mut buffer:[u8; ADV_PDU_SIZE_MAX] = [0; ADV_PDU_SIZE_MAX];
        let adv_data= ad_fields.write(&mut buffer);
        assert_eq!((PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + (4 * service_uuids.len())), adv_data.len());
        assert_eq!((adv_data.len() - 1), adv_data[0] as usize);
        assert_eq!(DataTypes::Complete32bitServiceUuids as u8, adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE]);
        assert_eq!(service_uuids[0].to_le_bytes(), adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE..]);
    }
    #[test]
    fn incomplete_list_service_uuid_128() {
        let service_uuids:[u128;1] = [0xA55A5AA5A55A5AA5];
        let ad_fields = AdFields{ incomplete_list_service_uuid_128:Some(&service_uuids), ..AdFields::default() };
        let mut buffer:[u8; ADV_PDU_SIZE_MAX] = [0; ADV_PDU_SIZE_MAX];
        let adv_data= ad_fields.write(&mut buffer);
        assert_eq!((PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + (16 * service_uuids.len())), adv_data.len());
        assert_eq!((adv_data.len() - 1), adv_data[0] as usize);
        assert_eq!(DataTypes::Incomplete128bitServiceUuids as u8, adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE]);
        assert_eq!(service_uuids[0].to_le_bytes(), adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE..]);
    }
    #[test]
    fn complete_list_service_uuid_128() {
        let service_uuids:[u128;1] = [0xA55A5AA5A55A5AA5];
        let ad_fields = AdFields{ complete_list_service_uuid_128:Some(&service_uuids), ..AdFields::default() };
        let mut buffer:[u8; ADV_PDU_SIZE_MAX] = [0; ADV_PDU_SIZE_MAX];
        let adv_data= ad_fields.write(&mut buffer);
        assert_eq!((PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + (16 * service_uuids.len())), adv_data.len());
        assert_eq!((adv_data.len() - 1), adv_data[0] as usize);
        assert_eq!(DataTypes::Complete128bitServiceUuids as u8, adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE]);
        assert_eq!(service_uuids[0].to_le_bytes(), adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE..]);
    }
    #[test]
    fn local_complete_name() {
        let name = "LOCAL NAME";
        let ad_fields = AdFields{ local_name:Some(name), ..AdFields::default() };
        let mut buffer:[u8; ADV_PDU_SIZE_MAX] = [0; ADV_PDU_SIZE_MAX];
        let adv_data= ad_fields.write(&mut buffer);
        assert_eq!((PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + name.len()), adv_data.len());
        assert_eq!((adv_data.len() - 1), adv_data[0] as usize);
        assert_eq!(DataTypes::CompleteLocalName as u8, adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE]);
        assert_eq!(*name.as_bytes(), adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE..]);
    }
    #[test]
    fn local_short_name() {
        let name = "LOCAL NAME";
        let ad_fields = AdFields{ short_name:Some(name), ..AdFields::default() };
        let mut buffer:[u8; ADV_PDU_SIZE_MAX] = [0; ADV_PDU_SIZE_MAX];
        let adv_data= ad_fields.write(&mut buffer);
        assert_eq!((PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + name.len()), adv_data.len());
        assert_eq!((adv_data.len() - 1), adv_data[0] as usize);
        assert_eq!(DataTypes::ShortenedLocalName as u8, adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE]);
        assert_eq!(*name.as_bytes(), adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE..]);
    }
    #[test]
    fn flags() {
        let flags = 0xa5;
        let ad_fields = AdFields{ flags:Some(flags), ..AdFields::default() };
        let mut buffer:[u8; ADV_PDU_SIZE_MAX] = [0; ADV_PDU_SIZE_MAX];
        let adv_data= ad_fields.write(&mut buffer);
        assert_eq!((PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + 1), adv_data.len());
        assert_eq!((adv_data.len() - 1), adv_data[0] as usize);
        assert_eq!(DataTypes::Flags as u8, adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE]);
        assert_eq!(flags as u8, adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE]);
    }
    #[test]
    fn manufacturer_specific_data() {
        let data:[u8;2] = [0; 2];
        let ad_fields = AdFields{ manufacturer_specific_data:Some(&data), ..AdFields::default() };
        let mut buffer:[u8; ADV_PDU_SIZE_MAX] = [0; ADV_PDU_SIZE_MAX];
        let adv_data= ad_fields.write(&mut buffer);
        assert_eq!((PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + 2), adv_data.len());
        assert_eq!((adv_data.len() - 1), adv_data[0] as usize);
        assert_eq!(DataTypes::ManufacturerSpecificData as u8, adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE]);
        assert_eq!(data, adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE..]);
    }
    #[test]
    fn tx_power_level() {
        let tx_power_level = 0;
        let ad_fields = AdFields{ tx_power_level:Some(tx_power_level), ..AdFields::default() };
        let mut buffer:[u8; ADV_PDU_SIZE_MAX] = [0; ADV_PDU_SIZE_MAX];
        let adv_data= ad_fields.write(&mut buffer);
        assert_eq!((PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + 1), adv_data.len());
        assert_eq!((adv_data.len() - 1), adv_data[0] as usize);
        assert_eq!(DataTypes::TxPowerLevel as u8, adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE]);
        assert_eq!(tx_power_level, adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE] as i8);
    }
    #[test]
    fn appearance() {
        let appearance = 0xA5;
        let ad_fields = AdFields{ appearance:Some(appearance), ..AdFields::default() };
        let mut buffer:[u8; ADV_PDU_SIZE_MAX] = [0; ADV_PDU_SIZE_MAX];
        let adv_data= ad_fields.write(&mut buffer);
        assert_eq!((PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + 2), adv_data.len());
        assert_eq!((adv_data.len() - 1), adv_data[0] as usize);
        assert_eq!(DataTypes::Appearance as u8, adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE]);
        assert_eq!(appearance.to_le_bytes(), adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE..]);
    }
    #[test]
    fn le_bluetooth_device_address() {
        let le_bluetooth_device_address:LeBluetoothDeviceAddress = [0;7];
        let ad_fields = AdFields{ le_bluetooth_device_address:Some(&le_bluetooth_device_address), ..AdFields::default() };
        let mut buffer:[u8; ADV_PDU_SIZE_MAX] = [0; ADV_PDU_SIZE_MAX];
        let adv_data= ad_fields.write(&mut buffer);
        assert_eq!((PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + le_bluetooth_device_address.len()), adv_data.len());
        assert_eq!((adv_data.len() - 1), adv_data[0] as usize);
        assert_eq!(DataTypes::LeBluetoothDeviceAddress as u8, adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE]);
        assert_eq!(le_bluetooth_device_address, adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE..]);
    }
    #[test]
    fn le_role() {
        let le_role = LeRole::OnlyCentralRole;
        let ad_fields = AdFields{ le_role:Some(le_role), ..AdFields::default() };
        let mut buffer:[u8; ADV_PDU_SIZE_MAX] = [0; ADV_PDU_SIZE_MAX];
        let adv_data= ad_fields.write(&mut buffer);
        assert_eq!((PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + 1), adv_data.len());
        assert_eq!((adv_data.len() - 1), adv_data[0] as usize);
        assert_eq!(DataTypes::LeRole as u8, adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE]);
        assert_eq!(le_role as u8, adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE]);
    }
    #[test]
    fn uri() {
        let uri = "URI";
        let ad_fields = AdFields{ uri:Some(uri), ..AdFields::default() };
        let mut buffer:[u8; ADV_PDU_SIZE_MAX] = [0; ADV_PDU_SIZE_MAX];
        let adv_data= ad_fields.write(&mut buffer);
        assert_eq!((PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + uri.len()), adv_data.len());
        assert_eq!((adv_data.len() - 1), adv_data[0] as usize);
        assert_eq!(DataTypes::Uri as u8, adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE]);
        assert_eq!(*uri.as_bytes(), adv_data[PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE..]);
    }
    #[test]
    fn multiple_fields() {
        let name = "concat";
        let ad_fields = AdFields{ local_name:Some(name), uri:Some(name), ..AdFields::default() };
        let mut buffer:[u8; ADV_PDU_SIZE_MAX] = [0; ADV_PDU_SIZE_MAX];
        let adv_data= ad_fields.write(&mut buffer);
        assert_eq!(2 * (PDU_ADV_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + name.len()), adv_data.len());
    }
}