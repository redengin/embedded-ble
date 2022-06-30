use crate::gap;

/// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1070566
/// https://www.novelbits.io/bluetooth-low-energy-advertisements-part-1/
/// note: the order of these properties identify the priority for inclusion in the 
///     advertisement (unless there is an explicit comment on the property).
#[derive(Default)]
pub(crate) struct Advertisement<'a> {
    pub local_name: Option<&'a str>,
    pub short_local_name: Option<&'a str>,
    pub uri: Option<&'a str>,

    /// see enum Flags
    pub flags: Option<u8>,

    pub tx_power_level: Option<i8>,

    pub advertising_interval: Option<u16>,
    /// note: only 24 bits are used
    pub advertising_interval_long: Option<u32>,
    pub peripheral_connection_interval_range: Option<PeripheralConnectionIntervalRange>,

    pub manufacturer_data: Option<&'a [u8]>,

    /// https://specificationrefs.bluetooth.com/assigned-values/Appearance%20Values.pdf
    pub appearance: Option<u16>,

    pub services_uuid16: Option<&'a [u16]>,
    pub services_uuid32: Option<&'a [u32]>,
    pub services_uuid128: Option<&'a [u128]>,
    // TODO determine if service_data and service_uuid should be combined as a Service type
        // service_data: Option<u8>,

    // TODO implement secure simple pairing
        // secure_simple_pairing_oob: Option<TODO>,
        // security_manager_oob: Option<TODO>,
        // security_manager_tk_value: Option<TODO>,

// TODO support mesh
    // services_solicitation_uuid16: Option<&'a [u16]>,
    // services_solicitation_uuid32: Option<&'a [u32]>,
    // services_solicitation_uuid128: Option<&'a [u128]>,

    // public_target_address: Option<&'a [Address]>,
    // random_target_address: Option<&'a [Address]>,
    // le_bluetooth_device_address: Option<DeviceAddress>,

    // le_role: Option<LeRole>,
    // le_supported_features: Option<u8>, // FIXME determine type
    // channel_map_update_indication: Option<ChannelMapIndication>,

    // BIGInfo: Option<u8>, // FIXME determine type
    // broadcast_code: Option<u8>, // FIXME determine type
}

enum Flags {
    LeLimitedDiscoverable = 0b00001,
    LeGeneralDiscoverable = 0b00010,
    LeAndBrEdrCapable = 0b00100,
}

/// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.999870
pub struct PeripheralConnectionIntervalRange {
    min: u16,
    max: u16,
}

struct Address([u8;6]);
/// can be random (bool = true), or public (bool = false)
struct DeviceAddress(bool, [u8;6]);

struct ChannelMapIndication {
    ch_m: u32,
    instant: u32,
}

/// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1005342
enum LeRole {
    PeripheralOnly = 0x00,
    CentralOnly = 0x01,
    PeripheralPreferred = 0x02,
    CentralPreferred = 0x03,
}

/// https://www.novelbits.io/bluetooth-low-energy-advertisements-part-1/#h-pdu-header
struct PduTypes;
#[allow(unused)]
impl PduTypes {
    const PDU_ADV_IND:u8 = 0b0000;
    const PDU_ADV_DIRECT_IND:u8 = 0b0001;
    const PDU_ADV_NONCONN_IND:u8 = 0b0010;
    const PDU_SCAN_REQ:u8 = 0b0011;
    const PDU_AUX_SCAN_REQ:u8 = 0b0011;
    const PDU_SCAN_RSP:u8 = 0b0100;
    const PDU_CONNECT_IND:u8 = 0b0101;
    const PDU_AUX_CONNECT_REQ:u8 = 0b0101;
    const PDU_ADV_SCAN_IND:u8 = 0b0110;
    const PDU_ADV_EXT_IND:u8 = 0b0111;
    const PDU_AUX_ADV_IND:u8 = 0b0111;
    const PDU_AUX_SCAP_RSP:u8 = 0b0111;
    const PDU_AUX_SYNC_IND:u8 = 0b0111;
    const PDU_AUX_CHAIN_IND:u8 = 0b0111;
    const PDU_AUX_CONNECT_RSP:u8 = 0b1000;
}

struct ScanResponse {
    adva: [u8; 6],
    data: [u8]      // max 31 bytes
}

impl<'a> Advertisement<'a> {
    pub fn adv_ind_pdu(&'a self, packet: &mut [u8]) -> Result<usize, &'static str> {
        // TODO handle ChSel, TxAdd, RxAdd
        packet[0] = PduTypes::PDU_ADV_EXT_IND;
        match self.payload(packet[2..].as_mut()) {
            Ok(payload_length) => {
                // set the pdu payload length
                packet[1] = payload_length as u8;
                return Ok(2 + payload_length)
            }
            Err(err) => return Err(err)
        }
    }

    fn payload(&'a self, packet: &mut [u8]) -> Result<usize, &'static str> {
        let mut remaining = packet.len();

        let mut cursor = 0;
        // add local name
        match self.local_name {
            Some(name) => {
                const MAX_NAME_LENGTH:usize = 100;
                if remaining < (2 + name.len()) {
                    return Err("local name won't fit in packet");
                }
                else if name.len() > MAX_NAME_LENGTH {
                    return Err("local name too large");
                }
                else {
                    packet[cursor] = gap::DataTypes::COMPLETE_LOCAL_NAME as u8;
                    cursor += 1; remaining -= 1;
                    packet[cursor] = name.len() as u8;
                    cursor += 1; remaining -= 1;
                    packet[cursor..].clone_from_slice(name.as_bytes());
                    cursor += name.len(); remaining -= name.len();
                    
                }
            }
            None => {}
        }

        todo!()
    }
}