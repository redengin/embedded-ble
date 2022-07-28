/// https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#G41.455603
pub const AD_ACCESS_ADDRESS:u32 = 0x8E89BED6;
/// https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=521059#G41.453964
pub const AD_CRCINIT:u32 = 0x555555;

const PDU_HEADER_SIZE:usize = 2;
const PDU_AD_STRUCTURE_LENGTH_SIZE:usize = 1;

#[derive(Default)]
/// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1070566
/// https://www.novelbits.io/bluetooth-low-energy-advertisements-part-1/
pub struct AdFields<'a> {
    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999520
    // TODO Service UUID
    pub _service_uuid:u8,

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
    pub _secure_simple_pairing_oob:u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999768
    // TODO SECURITY MANAGER OUT OF BAND
    pub _security_manager_oob:u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999818
    // TODO SECURITY MANAGER TK VALUE
    pub _security_manager_tk_value:u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999838
    // TODO PERIPHERAL CONNECTION INTERVAL RANGE
    pub _peripheral_connection_interval_range:u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999871
    // TODO SERVICE SOLICITATION
    pub _service_solicitation:u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999894
    // TODO SERVICE DATA
    pub _service_data:u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999913
    /// https://specificationrefs.bluetooth.com/assigned-values/Appearance%20Values.pdf
    pub appearance: Option<u16>,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999932
    // TODO PUBLIC TARGET ADDRESS
    pub _public_target_address:u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999953
    // TODO RANDOM TARGET ADDRESS
    pub _random_target_address:u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1004048
    // TODO ADVERTISING INTERVAL
    pub _advertising_interval:u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1005265
    pub le_bluetooth_device_address: Option<&'a LeBluetoothDeviceAddress>,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1005342
    pub le_role: Option<LE_ROLE>,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1005559
    pub uri: Option<&'a str>,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1054608
    // TODO LE SUPPORTED FEATURES
    pub _le_supported_features:u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1055008
    // TODO CHANNEL MAP UPDATE INDICATION
    pub _channel_map_update_indication:u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1176955
    // TODO BIGINFO
    pub _biginfo:u8,

    /// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1177011
    // TODO BROADCAST_CODE
    pub _broadcast_code:u8,
}

impl<'a> AdFields<'a> {
    /// places ad structures as long as they will fit in packet
    pub fn to_pdu(&'a self, buffer:&'a mut [u8], pdu_type:PDU_TYPE) -> &[u8]
    {
        let mut pdu_size = 0;

        // specification allows this to be larger than one-byte, but only
        //      single byte types are currently defined
        const AD_TYPE_SIZE:usize = 1;

        // add the header
        buffer[0] = pdu_type as u8;  // TODO handle ChSel, TxAdd, RxAdd
        pdu_size += 2;  // skip over length (which will be set later)

        // add local_name
        match self.local_name {
            Some(name) => if buffer.len() >= (pdu_size + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + name.len()) {
                // set ad structure length
                buffer[pdu_size] = (AD_TYPE_SIZE + name.len()) as u8;
                pdu_size += 1;
                // set ad structure type
                buffer[pdu_size] = DataTypes::CompleteLocalName as u8;
                pdu_size += 1;
                // set ad structure payload
                buffer[pdu_size..(pdu_size + name.len())].copy_from_slice(name.as_bytes());
                pdu_size += name.len();
            }
            None => {}
        }
        // add short_name
        match self.short_name {
            Some(name) => if buffer.len() >= (pdu_size + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + name.len()) {
                // set ad structure length
                buffer[pdu_size] = (AD_TYPE_SIZE + name.len()) as u8;
                pdu_size += 1;
                // set ad structure type
                buffer[pdu_size] = DataTypes::ShortenedLocalName as u8;
                pdu_size += 1;
                // set ad structure payload
                buffer[pdu_size..(pdu_size + name.len())].copy_from_slice(name.as_bytes());
                pdu_size += name.len();
            }
            None => {}
        }
        // add flags
        match self.flags {
            Some(flags) => if buffer.len() >= (pdu_size + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + 1) {
                // set ad structure length
                buffer[pdu_size] = (AD_TYPE_SIZE + 1) as u8;
                pdu_size += 1;
                // set ad structure type
                buffer[pdu_size] = DataTypes::Flags as u8;
                pdu_size += 1;
                // set ad structure payload
                buffer[pdu_size] = flags;
                pdu_size += 1;
            }
            None => {}
        }
        // add manufacturer data
        match self.manufacturer_specific_data {
            Some(data) => if buffer.len() >= (pdu_size + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + data.len()) {
                /// manufacturer data must have 2 byte company identifier to be valid
                assert!(data.len() >= 2);
                // set ad structure length
                buffer[pdu_size] = (AD_TYPE_SIZE + data.len()) as u8;
                pdu_size += 1;
                // set ad structure type
                buffer[pdu_size] = DataTypes::ManufacturerSpecificData as u8;
                pdu_size += 1;
                // set ad structure payload
                buffer[pdu_size..(pdu_size + data.len())].copy_from_slice(data);
                pdu_size += data.len();
            }
            None => {}
        }
        // add tx_power_level
        match self.tx_power_level {
            Some(level) => if buffer.len() >= (pdu_size + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + 1) {
                // set ad structure length
                buffer[pdu_size] = (AD_TYPE_SIZE + 1) as u8;
                pdu_size += 1;
                // set ad structure type
                buffer[pdu_size] = DataTypes::TxPowerLevel as u8;
                pdu_size += 1;
                // set ad structure payload
                buffer[pdu_size] = level as u8;
                pdu_size += 1;
            }
            None => {}
        }
        // add appearance
        match self.appearance {
            Some(id) => if buffer.len() >= (pdu_size + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + 2) {
                // set ad structure length
                buffer[pdu_size] = (AD_TYPE_SIZE + 2) as u8;
                pdu_size += 1;
                // set ad structure type
                buffer[pdu_size] = DataTypes::Appearance as u8;
                pdu_size += 1;
                // set ad structure payload
                buffer[pdu_size..(pdu_size + 2)].copy_from_slice(&id.to_le_bytes());
                pdu_size += 2;
            }
            None => {}
        }
        // add le device address
        match self.le_bluetooth_device_address {
            Some(address) => if buffer.len() >= (pdu_size + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + address.len()) {
                // set ad structure length
                buffer[pdu_size] = (AD_TYPE_SIZE + address.len()) as u8;
                pdu_size += 1;
                // set ad structure type
                buffer[pdu_size] = DataTypes::LeBluetoothDeviceAddress as u8;
                pdu_size += 1;
                // set ad structure payload
                buffer[pdu_size..(pdu_size + address.len())].copy_from_slice(address);
                pdu_size += address.len();
            }
            None => {}
        }
        // add le role
        match self.le_role{
            Some(role) => if buffer.len() >= (pdu_size + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + 1) {
                // set ad structure length
                buffer[pdu_size] = (AD_TYPE_SIZE + 1) as u8;
                pdu_size += 1;
                // set ad structure type
                buffer[pdu_size] = DataTypes::LeRole as u8;
                pdu_size += 1;
                // set ad structure payload
                buffer[pdu_size] = role as u8;
                pdu_size += 1;
            }
            None => {}
        }
        // add uri
        match self.uri {
            Some(uri) => if buffer.len() >= (pdu_size + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + uri.len()) {
                // set ad structure length
                buffer[pdu_size] = (AD_TYPE_SIZE + uri.len()) as u8;
                pdu_size += 1;
                // set ad structure type
                buffer[pdu_size] = DataTypes::Uri as u8;
                pdu_size += 1;
                // set ad structure payload
                buffer[pdu_size..(pdu_size + uri.len())].copy_from_slice(uri.as_bytes());
                pdu_size += uri.len();
            }
            None => {}
        }

        // write the payload length
        assert!(pdu_size <= u8::MAX as usize);
        buffer[1] = (pdu_size - PDU_HEADER_SIZE) as u8;

        &buffer[..pdu_size]
    }
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

/// https://btprodspecificationrefs.blob.core.windows.net/assigned-numbers/Assigned%20Number%20Types/Generic%20Access%20Profile.pdf
pub enum DataTypes {
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
enum Flags {
    LeLimitedDiscoverable   = 0b00001,
    LeGeneralDiscoverable   = 0b00010,
    LeAndBrEdrCapable       = 0b00100,
}

/// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1005365
type LeBluetoothDeviceAddress = [u8;7];

/// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1005585
#[derive(Copy, Clone)]
pub enum LE_ROLE {
    ONLY_PERIPHERAL_ROLE                    = 0x00,
    ONLY_CENTRAL_ROLE                       = 0x01,
    /// peripheral role preferred
    PERIPHERAL_AND_CENTRAL_ROLE_PERIPHERAL  = 0x02,
    /// central role preferred
    PERIPHERAL_AND_CENTRAL_ROLE_CENTRAL     = 0x03,
}

#[cfg(test)]
mod adfields_to_pdu {
    use super::*;

    #[test]
    fn pdu_local_name() {
        let name = "LOCAL NAME";
        {
            let ad_fields = AdFields{ local_name:Some(name), ..AdFields::default() };
            let mut buffer:[u8; crate::BLE_PDU_SIZE_MAX] = [0; crate::BLE_PDU_SIZE_MAX];
            let pdu = ad_fields.to_pdu(&mut buffer, PDU_TYPE::ADV_DIRECT_IND);
            const AD_TYPE_SIZE:usize = 1;
            assert_eq!((PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + name.len()), pdu.len());
            assert_eq!((PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + name.len()), pdu[1] as usize);
            assert_eq!(DataTypes::CompleteLocalName as u8, pdu[PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE]);
            assert_eq!(*name.as_bytes(), pdu[PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE..]);
        }
        {
            let ad_fields = AdFields{ short_name:Some(name), ..AdFields::default() };
            let mut buffer:[u8; crate::BLE_PDU_SIZE_MAX] = [0; crate::BLE_PDU_SIZE_MAX];
            let pdu = ad_fields.to_pdu(&mut buffer, PDU_TYPE::ADV_DIRECT_IND);
            const AD_TYPE_SIZE:usize = 1;
            assert_eq!((PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + name.len()), pdu.len());
            assert_eq!((PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + name.len()), pdu[1] as usize);
            assert_eq!(DataTypes::ShortenedLocalName as u8, pdu[PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE]);
            assert_eq!(*name.as_bytes(), pdu[PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE..]);
        }
    }
    #[test]
    fn pdu_flags() {
        let flags = 0xA5 as u8;
        let ad_fields = AdFields{ flags:Some(flags), ..AdFields::default() };
        let mut buffer:[u8; crate::BLE_PDU_SIZE_MAX] = [0; crate::BLE_PDU_SIZE_MAX];
        let pdu = ad_fields.to_pdu(&mut buffer, PDU_TYPE::ADV_DIRECT_IND);
        const AD_TYPE_SIZE:usize = 1;
        assert_eq!((PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + 1), pdu.len());
        assert_eq!((PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + 1), pdu[1] as usize);
        assert_eq!(DataTypes::Flags as u8, pdu[PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE]);
        assert_eq!(flags, pdu[PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE]);
    }
    #[test]
    fn pdu_manufacturer_specific_data() {
        let data:[u8;2] = [0; 2];
        let ad_fields = AdFields{ manufacturer_specific_data:Some(&data), ..AdFields::default() };
        let mut buffer:[u8; crate::BLE_PDU_SIZE_MAX] = [0; crate::BLE_PDU_SIZE_MAX];
        let pdu = ad_fields.to_pdu(&mut buffer, PDU_TYPE::ADV_DIRECT_IND);
        const AD_TYPE_SIZE:usize = 1;
        assert_eq!((PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + data.len()), pdu.len());
        assert_eq!((PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + data.len()), pdu[1] as usize);
        assert_eq!(DataTypes::ManufacturerSpecificData as u8, pdu[PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE]);
        assert_eq!(data, pdu[PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE..]);
    }
    #[test]
    fn pdu_tx_power_level() {
        let tx_power_level = 0;
        let ad_fields = AdFields{ tx_power_level:Some(tx_power_level), ..AdFields::default() };
        let mut buffer:[u8; crate::BLE_PDU_SIZE_MAX] = [0; crate::BLE_PDU_SIZE_MAX];
        let pdu = ad_fields.to_pdu(&mut buffer, PDU_TYPE::ADV_DIRECT_IND);
        const AD_TYPE_SIZE:usize = 1;
        assert_eq!((PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + 1), pdu.len());
        assert_eq!((PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + 1), pdu[1] as usize);
        assert_eq!(DataTypes::TxPowerLevel as u8, pdu[PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE]);
        assert_eq!(tx_power_level as u8, pdu[PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE]);
    }
    #[test]
    fn pdu_appearance() {
        let appearance = 0xA5;
        let ad_fields = AdFields{ appearance:Some(appearance), ..AdFields::default() };
        let mut buffer:[u8; crate::BLE_PDU_SIZE_MAX] = [0; crate::BLE_PDU_SIZE_MAX];
        let pdu = ad_fields.to_pdu(&mut buffer, PDU_TYPE::ADV_DIRECT_IND);
        const AD_TYPE_SIZE:usize = 1;
        assert_eq!((PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + 2), pdu.len());
        assert_eq!((PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + 2), pdu[1] as usize);
        assert_eq!(DataTypes::Appearance as u8, pdu[PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE]);
        assert_eq!(appearance.to_le_bytes(), pdu[PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE..]);
    }
    #[test]
    fn pdu_le_bluetooth_device_address() {
        let le_bluetooth_device_address:LeBluetoothDeviceAddress = [0;7];
        let ad_fields = AdFields{ le_bluetooth_device_address:Some(&le_bluetooth_device_address), ..AdFields::default() };
        let mut buffer:[u8; crate::BLE_PDU_SIZE_MAX] = [0; crate::BLE_PDU_SIZE_MAX];
        let pdu = ad_fields.to_pdu(&mut buffer, PDU_TYPE::ADV_DIRECT_IND);
        const AD_TYPE_SIZE:usize = 1;
        assert_eq!((PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + le_bluetooth_device_address.len()), pdu.len());
        assert_eq!((PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + le_bluetooth_device_address.len()), pdu[1] as usize);
        assert_eq!(DataTypes::LeBluetoothDeviceAddress as u8, pdu[PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE]);
        assert_eq!(le_bluetooth_device_address, pdu[PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE..]);
    }
    #[test]
    fn pdu_le_role() {
        let le_role = LE_ROLE::ONLY_CENTRAL_ROLE;
        let ad_fields = AdFields{ le_role:Some(le_role), ..AdFields::default() };
        let mut buffer:[u8; crate::BLE_PDU_SIZE_MAX] = [0; crate::BLE_PDU_SIZE_MAX];
        let pdu = ad_fields.to_pdu(&mut buffer, PDU_TYPE::ADV_DIRECT_IND);
        const AD_TYPE_SIZE:usize = 1;
        assert_eq!((PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + 1), pdu.len());
        assert_eq!((PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + 1), pdu[1] as usize);
        assert_eq!(DataTypes::LeRole as u8, pdu[PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE]);
        assert_eq!(le_role as u8, pdu[PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE]);
    }
    #[test]
    fn pdu_uri() {
        let uri = "URI";
        let ad_fields = AdFields{ uri:Some(uri), ..AdFields::default() };
        let mut buffer:[u8; crate::BLE_PDU_SIZE_MAX] = [0; crate::BLE_PDU_SIZE_MAX];
        let pdu = ad_fields.to_pdu(&mut buffer, PDU_TYPE::ADV_DIRECT_IND);
        const AD_TYPE_SIZE:usize = 1;
        assert_eq!((PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + uri.len()), pdu.len());
        assert_eq!((PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + uri.len()), pdu[1] as usize);
        assert_eq!(DataTypes::Uri as u8, pdu[PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE]);
        assert_eq!(*uri.as_bytes(), pdu[PDU_HEADER_SIZE + PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE..]);
    }

    #[test]
    fn pdu_concat() {
        let name = "concat";
        let ad_fields = AdFields{ local_name:Some(name), uri:Some(name), ..AdFields::default() };
        let mut buffer:[u8; crate::BLE_PDU_SIZE_MAX] = [0; crate::BLE_PDU_SIZE_MAX];
        let pdu = ad_fields.to_pdu(&mut buffer, PDU_TYPE::ADV_DIRECT_IND);
        const AD_TYPE_SIZE:usize = 1;
        assert_eq!((PDU_HEADER_SIZE + (2 * (PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + name.len()))), pdu.len());
        assert_eq!(((2 * (PDU_AD_STRUCTURE_LENGTH_SIZE + AD_TYPE_SIZE + name.len()))), pdu[1] as usize);
    }
}