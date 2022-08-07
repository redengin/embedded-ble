use nrf52832_pac::{self as pac};

use crate::{link_layer};
use pac::{FICR, ficr::deviceaddrtype::DEVICEADDRTYPE_A};
// use pac::{RADIO, radio::txpower::TXPOWER_A};
use pac::{RADIO};
use core::ptr::{write_volatile, read_volatile};
use core::sync::atomic::{compiler_fence, Ordering};
use rtt_target::{rprintln};

pub struct Nrf5xHci {
    radio: RADIO,
    pub(crate) adv_a: link_layer::AdvA, // hw address
}


pub enum RadioMode { Ble1Mbit, Ble2Mbit }
impl Nrf5xHci {
    pub fn new(radio:RADIO, mode:RadioMode, ficr:FICR) -> Self {
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
                    .s0len().set_bit()
                    .lflen().bits(8)
                    .s1len().bits(0)
                    .plen()._16bit()
                });
            }
        }
        radio.pcnf1.write(|w| unsafe{ w
            .maxlen().bits(255)
            .statlen().bits(0)
            .balen().bits(3)    // (prefix:1 + base:3) address per BLE spec
            .endian().little()
            .whiteen().enabled()
        });

        // enables fast ramp-up
        radio.modecnf0.write(|w| w.ru().set_bit());

        // this driver only uses first address entry(0)
        radio.txaddress.write(|w| unsafe{ w.bits(0) });
        radio.rxaddresses.write(|w| w
            .addr0().enabled()
            .addr1().disabled()
            .addr2().disabled()
            .addr3().disabled()
            .addr4().disabled()
            .addr5().disabled()
            .addr6().disabled()
            .addr7().disabled()
        );

        // configure for CRC24 per BLE spec
        radio.crccnf.write(|w| w
            .skipaddr().skip()  // skip address (only CRC the PDU)
            .len().three());    // CRC32 (3 bytes)
        radio.crcpoly.write(|w| unsafe{ w.crcpoly().bits(0x0000065B) });

        // configure interframe spacing per BLE spec
        radio.tifs.write(|w| unsafe{ w.tifs().bits(150) });

        // set transmit power (max)
        radio.txpower.write(|w| w.txpower().pos4d_bm());

        // TODO support encryption (CCM)
        // TODO support privacy (AAR)

        Self{
            radio,
            adv_a : Self::get_address(ficr),
        }
    }

    fn get_address(ficr:FICR) -> link_layer::AdvA {
        let mut address:link_layer::Address = [0; 6];
        address[..4].copy_from_slice( &ficr.deviceaddr[0].read().bits().to_le_bytes());
        address[4..].copy_from_slice( &(ficr.deviceaddr[1].read().bits() as u16).to_le_bytes());

        return match ficr.deviceaddrtype.read().deviceaddrtype().variant() {
            DEVICEADDRTYPE_A::PUBLIC => link_layer::AdvA::Public(address),
            DEVICEADDRTYPE_A::RANDOM => link_layer::AdvA::RandomStatic(address),
        }
    }

    fn set_channel(&self, channel:link_layer::Channel, access_address:u32) {
        // set channel
        self.radio.frequency.write(|w| unsafe{ w.frequency().bits(channel.frequency()) });
        self.radio.datawhiteiv.write(|w| unsafe{ w.datawhiteiv().bits(channel as u8)});

        // set the access address
        self.radio.prefix0.write(|w| unsafe{ w.ap0().bits((access_address >> 24) as u8) });
        // set top three bytes of base0 per balen(3)
        self.radio.base0.write(|w| unsafe{ w.base0().bits(access_address << 8) });

        // apply errata https://infocenter.nordicsemi.com/pdf/nRF52832_Rev_2_Errata_v1.7.pdf#%5B%7B%22num%22%3A318%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C85.039%2C296.523%2Cnull%5D
        // *(volatile uint32_t *) 0x4000173C |= (1 << 10)
        // unsafe{ 
        //     const undocumented:*mut u32 = 0x4000173C as *mut u32;
        //     write_volatile(undocumented, read_volatile(undocumented) | (1 << 10));
        // }
        // using nimble's implementation for errata
        // unsafe{ 
        //     const UNDOCUMENTED:*mut u32 = 0x40001774 as *mut u32;
        //     write_volatile(UNDOCUMENTED, 
        //         (read_volatile(UNDOCUMENTED) & 0xfffffffe) | 0x01000000);
        // }
    }

    // fn set_txpower(&self, power:TXPOWER_A) {
    //     self.radio.txpower.write(|w| w.txpower().variant(power))
    // }

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
    pub(crate) fn send(&self, channel:link_layer::Channel, access_address:u32, crcinit:u32, pdu: &[u8]) -> bool {
        if ! self.radio.state.read().state().is_disabled() {
            return false;
        }

        // TODO debug: validate pdu buffer
        // assert()

        // TODO determine if access_address and/or crcinit are relevant for listen
        self.set_channel(channel, access_address);

        // TODO support encryption (CCM)
        // TODO support privacy (AAR)

        rprintln!("{:?}", pdu);
        self.radio.packetptr.write(|w| unsafe{ w.bits(pdu.as_ptr() as u32) });
        // self.radio.crcinit.write(|w| unsafe{ w.crcinit().bits(crcinit) });
        self.radio.crcinit.write(|w| unsafe{ w.crcinit().bits(link_layer::ADV_CRCINIT) });

        // allow hardware to handle packet and disable radio upon completion
        self.radio.shorts.write(|w| w
            .ready_start().enabled()
            .end_disable().enabled()
        );

        // "Preceding reads and writes cannot be moved past subsequent writes."
        compiler_fence(Ordering::Release);
        // kick off the transmission
        self.radio.tasks_txen.write(|w| unsafe{ w.bits(1) });

        while ! self.radio.state.read().state().is_disabled() {}
        // assert!(! self.radio.state.read().state().is_disabled());

        return true
    }
}