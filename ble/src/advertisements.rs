use core::mem::MaybeUninit;

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
    LE_LIMITED_DISCOVERABLE = 0b00001,
    LE_GENERAL_DISCOVERABLE = 0b00010,
    LE_AND_BR_EDR_CAPABLE = 0b00100,
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
    ChM: u32,
    Instant: u32,
}

/// https://www.bluetooth.org/docman/handlers/DownloadDoc.ashx?doc_id=519976#G3.1005342
enum LeRole {
    PERIPHERAL_ONLY = 0x00,
    CENTRAL_ONLY = 0x01,
    PERIPHERAL_PREFERRED = 0x02,
    CENTRAL_PREFERRED = 0x03,
}

/// https://www.novelbits.io/bluetooth-low-energy-advertisements-part-1/#h-pdu-header
type pdu = u8;
const PUD_ADV_DIRECT_IND:pdu = 0b0001;
const PDU_ADV_NONCONN_IND:pdu = 0b0010;
const PDU_SCAN_REQ:pdu = 0b0011;
const PDU_AUX_SCAN_REQ:pdu = 0b0011;
const PDU_SCAN_RSP:pdu = 0b0100;
const PDU_CONNECT_IND:pdu = 0b0101;
const PDU_AUX_CONNECT_REQ:pdu = 0b0101;
const PDU_ADV_SCAN_IND:pdu = 0b0110;
const PDU_ADV_EXT_IND:pdu = 0b0111;
const PDU_AUX_ADV_IND:pdu = 0b0111;
const PDU_AUX_SCAP_RSP:pdu = 0b0111;
const PDU_AUX_SYNC_IND:pdu = 0b0111;
const PDU_AUX_CHAIN_IND:pdu = 0b0111;
const PDU_AUX_CONNECT_RSP:pdu = 0b1000;

struct ScanResponse {
    adva: [u8; 6],
    data: [u8]      // max 31 bytes
}

impl<'a> Advertisement<'a> {
    pub fn adv_ind_pdu(&'a self, packet: &mut [u8]) -> Result<usize, &'static str> {
        const PDU_ADV_IND:pdu = 0b0000;
        // TODO handle ChSel, TxAdd, RxAdd
        packet[0] = PDU_ADV_EXT_IND;
        match self.payload(packet[2..].as_mut()) {
            Ok(length) => {
                packet[1] = length as u8;
                return Ok(length)
            }
            Err(err) => return Err(err)
        }
    }

    fn payload(&'a self, packet: &mut [u8]) -> Result<usize, &'static str> {
        let mut remaining = packet.len();

        // add local name
        if self.local_name.is_some() {

        }

        todo!()
    }
}