use crate::{Channel};
use nrf52832_pac::{RADIO, FICR, radio::txpower::TXPOWER_A};
use core::ptr::{write_volatile, read_volatile};
use core::sync::atomic::{compiler_fence, Ordering};

pub struct Nrf5xHci {
    radio: RADIO
}


pub enum RadioMode { Ble1Mbit, Ble2Mbit }
impl Nrf5xHci {
    pub fn new(radio:RADIO, mode:RadioMode, _ficr:FICR) -> Self {
        // NOTE: the unsafe blocks are required per the PAC
        //      as they perform direct register access, their is no real concern.
        match mode {
            RadioMode::Ble1Mbit => {
                radio.mode.write(|w| w.mode().ble_1mbit());
                radio.pcnf0.write(|w| unsafe{ w
                    .s0len().set_bit()
                    .lflen().bits(8)
                    .s1len().bits(0)
                    .plen()._8bit()
            });
            }
            RadioMode::Ble2Mbit => {
                radio.mode.write(|w| w.mode().ble_2mbit());
                radio.pcnf0.write(|w| unsafe{ w
                    .lflen().bits(8)
                    .s0len().set_bit()
                    .s1len().bits(0)
                    .plen()._16bit()
                });
            }
        }
        radio.pcnf1.write(|w| unsafe{ w
            .endian().little()
            .balen().bits(3)
            .whiteen().enabled()
        });

        // enables fast ramp-up
        radio.modecnf0.write(|w| w.ru().set_bit());

        radio.txaddress.write(|w| unsafe{ w.bits(0) });
        radio.rxaddresses.write(|w| w.addr0().enabled());

        // configure for CRC24
        radio.crccnf.write(|w| w.skipaddr().skip().len().three());
        radio.crcpoly.write(|w| unsafe{ w.crcpoly().bits(0x0000065B) });

        // configure interframe spacing
        radio.tifs.write(|w| unsafe{ w.tifs().bits(150) });

        // TODO support encryption (CCM)
        // TODO support privacy (AAR)
        
        Self{radio}
    }

    fn set_channel(&self, channel:Channel, access_address:u32, crcinit:u32) {
        // set the access address
        self.radio.base0.write(|w| unsafe{ w.base0().bits(access_address << 8) });
        self.radio.prefix0.write(|w| unsafe{ w
            .ap0().bits((access_address >> 24) as u8)
            .ap1().bits(0xFF)
            .ap2().bits(0xFF)
            .ap3().bits(0xFF)
        });

        // apply errata https://infocenter.nordicsemi.com/pdf/nRF52832_Rev_2_Errata_v1.7.pdf#%5B%7B%22num%22%3A318%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C85.039%2C296.523%2Cnull%5D
        // *(volatile uint32_t *) 0x4000173C |= (1 << 10)
        // unsafe{ 
        //     const undocumented:*mut u32 = 0x4000173C as *mut u32;
        //     write_volatile(undocumented, read_volatile(undocumented) | (1 << 10));
        // }
        // using nimble's implementation for errata
        unsafe{ 
            const UNDOCUMENTED:*mut u32 = 0x40001774 as *mut u32;
            write_volatile(UNDOCUMENTED, 
                (read_volatile(UNDOCUMENTED) & 0xfffffffe) | 0x01000000);
        }

        self.radio.crcinit.write(|w| unsafe{ w.crcinit().bits(crcinit) });

        self.radio.frequency.write(|w| unsafe{ w.frequency().bits(channel.frequency()) });
        self.radio.datawhiteiv.write(|w| unsafe{ w.datawhiteiv().bits(channel as u8) });
    }

    fn set_txpower(&self, power:TXPOWER_A) {
        self.radio.txpower.write(|w| w.txpower().variant(power))
    }

    /// starts receive for a single packet
    /// upon a packet, the RADIO interrupt will fire (and the RADIO will be disabled)
    // pub(crate) fn listen(&self, channel:Channel, buffer: &mut [u8]) -> bool {
    //     // radio must be in disabled mode to start a new listen
    //     if ! self.radio.state.read().state().is_disabled() {
    //         return false;
    //     }

    //     // TODO determine if access_address and/or crcinit are relevant for listen
    //     self.set_channel(channel, 0, 0);

    //     // TODO support encryption (CCM)
    //     // TODO support privacy (AAR)

    //     self.radio.packetptr.write(|w| unsafe{ w.bits(buffer.as_ptr() as u32) });

    //     // allow hardware to handle packet and disable radio upon completion
    //     self.radio.shorts.write(|w| w
    //         .ready_start().set_bit()    // start listening
    //         .end_disable().set_bit()    // disable radio upon a packet
    //         .address_bcstart().set_bit()
    //         .address_rssistart().set_bit()
    //         .disabled_rssistop().set_bit()
    //     );

    //     // enable RADIO disabled interrupt (as "shorts" are enabled, the radio will be disabled upon a packet)
    //     self.radio.intenset.write(|w| w.disabled().set());

    //     todo!()
    // }

    /// attempts to send a PDU (hardware takes care of preamble, access-address, and CRC)
    pub(crate) fn send(&self, channel:Channel, access_address:u32, crcinit:u32, pdu: &[u8]) -> bool {
        if ! self.radio.state.read().state().is_disabled() {
            return false;
        }

        // TODO debug: validate pdu buffer
        // assert()

        // TODO determine if access_address and/or crcinit are relevant for listen
        self.set_channel(channel, access_address, crcinit);

        // TODO support encryption (CCM)
        // TODO support privacy (AAR)

        self.radio.packetptr.write(|w| unsafe{ w.bits(pdu.as_ptr() as u32) });

        // allow hardware to handle packet and disable radio upon completion
        self.radio.shorts.write(|w| w
            .ready_start().enabled()
            .end_disable().enabled()
        );

        // "Preceding reads and writes cannot be moved past subsequent writes."
        compiler_fence(Ordering::Release);
        // kick off the transmission
        self.radio.tasks_txen.write(|w| unsafe{ w.bits(1) });

        return true
    }
}