
// reserved channels 0x0000 - 0x003F
// L2CAP signalling 0x0001
// L2CAP LE signalling 0x0005 with 0x0004, 0x0006

struct L2Cap_Pdu {
    length: u16,
    channel: u16,
    // payload: []
}