use nrf52832_pac::{RADIO, FICR};


pub struct Nrf5xHci {
    radio: RADIO
}

pub enum RadioMode { OneMbit, TwoMbit }
impl Nrf5xHci {
    pub fn new(radio:RADIO, mode:RadioMode, _ficr:FICR) -> Self {
        // NOTE: the unsafe blocks are required per the PAC, as they perform direct
        //  register access, their is no real concern.
        match mode {
            RadioMode::OneMbit => {
                radio.mode.write(|w| w.mode().ble_1mbit());
                unsafe {
                    radio.pcnf0.write(|w| w
                        .lflen().bits(8)
                        .s0len().set_bit()
                        .s1len().bits(0)
                        .plen()._8bit()
                    );
                }
            }
            RadioMode::TwoMbit => {
                radio.mode.write(|w| w.mode().ble_1mbit());
                unsafe {
                    radio.pcnf0.write(|w| w
                        .lflen().bits(8)
                        .s0len().set_bit()
                        .s1len().bits(0)
                        .plen()._16bit()
                    );
                }
            }
        }
        unsafe {
            radio.pcnf1.write(|w| w
                .endian().little()
                .balen().bits(3)
                .whiteen().enabled()
            );
        }

        // enables fast ramp-up
        radio.modecnf0.write(|w| w.ru().set_bit());

        unsafe {
            radio.txaddress.write(|w| w.bits(0));
            radio.rxaddresses.write(|w| w.addr0().enabled());
        }

        // configure for CRC24
        radio.crccnf.write(|w| w.skipaddr().skip().len().three());
        unsafe { radio.crcpoly.write(|w| w.crcpoly().bits(0x0000065B)); }

        // configure interframe spacing
        unsafe { radio.tifs.write(|w| w.tifs().bits(150)); }

        // TODO support encryption (CCM)
        // TODO support privacy (AAR)
        
        Self{radio}
    }
}