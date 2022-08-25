use nrf52832_hal::{pac};

use crate::{link_layer};
use pac::{FICR, ficr::deviceaddrtype::DEVICEADDRTYPE_A};
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
    pub fn new(radio:RADIO, mode:RadioMode, ficr:FICR) -> Self
    {
        // TODO for nrf51 us FICR to tune radio parameters

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
            .maxlen().bits(link_layer::PDU_SIZE_MAX as u8)
            .statlen().bits(0)
            .balen().bits(3)    // (prefix:1 + base:3) address per BLE spec
            .endian().little()
            .whiteen().enabled()
        });

        // enables fast ramp-up
        radio.modecnf0.write(|w| w.ru().set_bit());

        // this driver only transmits using first address entry(0)
        radio.txaddress.write(|w| unsafe{ w.bits(0) });

        // configure for CRC24 per BLE spec
        radio.crccnf.write(|w| w
            .skipaddr().skip()  // skip address (only CRC the PDU)
            .len().three());    // CRC32 (3 bytes)
        radio.crcpoly.write(|w| unsafe{ w.crcpoly().bits(link_layer::BLE_CRC_POLYNOMIAL) });

        // configure interframe spacing per BLE spec
        radio.tifs.write(|w| unsafe{ w.tifs().bits(link_layer::T_IFS_US) });

        // TODO support encryption (CCM)
        // TODO support privacy (AAR)

        // configure for maximum power
        radio.txpower.write(|w| w.txpower().pos4d_bm());

        Self{
            radio,
            adv_a : Self::get_address(ficr),
        }
    }

    fn get_address(ficr:FICR) -> link_layer::AdvA {
        let mut address:link_layer::Address = [0; 6];
        address[2..6].copy_from_slice( &ficr.deviceaddr[0].read().bits().to_be_bytes());
        address[..2].copy_from_slice( &(ficr.deviceaddr[1].read().bits() as u16).to_be_bytes());

        return match ficr.deviceaddrtype.read().deviceaddrtype().variant() {
            DEVICEADDRTYPE_A::PUBLIC => link_layer::AdvA::Public(address),
            DEVICEADDRTYPE_A::RANDOM => link_layer::AdvA::RandomStatic(address),
        }
    }

    fn set_channel(&self, channel:link_layer::Channel, access_address:u32) {
        // set channel
        self.radio.frequency.write(|w| unsafe{ w.frequency().bits(channel.frequency()) });
        self.radio.datawhiteiv.write(|w| unsafe{ w.datawhiteiv().bits(channel as u8)});

        // set the access address (using entry(0))
        self.radio.prefix0.write(|w| unsafe{ w.ap0().bits((access_address >> 24) as u8) });
        // set top three bytes of base0 per balen(3)
        self.radio.base0.write(|w| unsafe{ w.base0().bits(access_address << 8) });

        // apply errata https://infocenter.nordicsemi.com/pdf/nRF52832_Rev_2_Errata_v1.7.pdf#%5B%7B%22num%22%3A318%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C85.039%2C296.523%2Cnull%5D
        unsafe{ 
            const UNDOCUMENTED:*mut u32 = 0x4000173C as *mut u32;
            write_volatile(UNDOCUMENTED, read_volatile(UNDOCUMENTED) | (1 << 10));
        }
        // nimble's implementation for errata
        unsafe{ 
            const UNDOCUMENTED:*mut u32 = 0x40001774 as *mut u32;
            write_volatile(UNDOCUMENTED, 
            (read_volatile(UNDOCUMENTED) & 0xfffffffe) | 0x01000000);
        }
    }
    /// set the transmit power
    /// Core_v5.3.pdf#G37.564979
    pub fn set_txpower(&self, db: i8) {
        self.radio.txpower.write(|w| w.txpower().variant(
            match db {
                i8::MIN..=-19 => pac::radio::txpower::TXPOWER_A::NEG40DBM,
                -20..=-15 => pac::radio::txpower::TXPOWER_A::NEG20DBM,
                -16..=-11 => pac::radio::txpower::TXPOWER_A::NEG16DBM,
                -12..=-7 => pac::radio::txpower::TXPOWER_A::NEG12DBM,
                -8..=-3 => pac::radio::txpower::TXPOWER_A::NEG8DBM,
                -4..=2 => pac::radio::txpower::TXPOWER_A::NEG4DBM,
                3 => pac::radio::txpower::TXPOWER_A::POS3DBM,
                4.. => pac::radio::txpower::TXPOWER_A::POS4DBM,
            }
        ));
    }

    /// attempts to send a PDU (hardware takes care of preamble, access-address, and CRC)
    pub(crate) fn send(&self, channel:link_layer::Channel, access_address:u32, crcinit:u32, buffer: &[u8]) -> bool
    {
        assert!(buffer.len() < link_layer::PDU_SIZE_MAX);

        rprintln!("Sending (hex) {:02X?}", buffer);

        // abort if the radio is busy
        if ! self.radio.state.read().state().is_disabled() {
            return false;
        }

        // setup the radio
        self.set_channel(channel, access_address);

        // TODO support encryption (CCM)
        // TODO support privacy (AAR)

        // initialize the crc value
        self.radio.crcinit.write(|w| unsafe{ w.crcinit().bits(crcinit) });
        // set the buffer
        self.radio.packetptr.write(|w| unsafe{ w.bits(buffer.as_ptr() as u32) });

        // configure radio to automatically disable itself after send
        self.radio.shorts.write(|w| w
            .ready_start().enabled()
            .end_disable().enabled()
        );

        // "Preceding reads and writes cannot be moved past subsequent writes."
        compiler_fence(Ordering::Release);
        // kick off the transmission
        self.radio.tasks_txen.write(|w| unsafe{ w.bits(1) });

        // await send completion (required as the radio is holding onto the buffer data)
        // TODO return a Future that holds on to the buffer (to allow work during transmission)
        while ! self.radio.state.read().state().is_disabled() {}

        return true
    }
}