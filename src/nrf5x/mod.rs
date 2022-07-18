use nrf52832_pac::{RADIO, FICR};


pub struct Nrf5xHci {
    radio: RADIO
}

impl Nrf5xHci {
    pub fn init(radio:RADIO, ficr:FICR) -> Self {

        Self{radio}
    }
}