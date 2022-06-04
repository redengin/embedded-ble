#![cfg_attr(not(test), no_std)]

#[cfg(feature = "nrf51")]
use nrf51_pac as pac;

#[cfg(feature = "nrf52805")]
use nrf52805_pac as pac;

#[cfg(feature = "nrf52810")]
use nrf52810_pac as pac;

#[cfg(feature = "nrf52811")]
use nrf52811_pac as pac;

#[cfg(feature = "nrf52832")]
use nrf52832_pac as pac;

#[cfg(feature = "nrf52833")]
use nrf52833_pac as pac;

#[cfg(feature = "nrf52840")]
use nrf52840_pac as pac;

pub struct Hci {
    radio: pac::RADIO,
    // tx_buf: &'static mut PacketBuffer,
    // rx_buf: &'static mut PacketBuffer,
}

use crate::pac::radio::state::STATE_R;

impl Hci {
    #[cfg(feature = "nrf51")]
    pub(crate) fn init(
        radio: pac::RADIO,
        access_address: u32,
        ficr: pac::FICR,
    ) -> Self {

        // for nRF51 manually trim values.
        if ficr.overrideen.read().ble_1mbit().is_override() {
            unsafe {
                radio
                    .override0
                    .write(|w| w.override0().bits(ficr.ble_1mbit[0].read().bits()));
                radio
                    .override1
                    .write(|w| w.override1().bits(ficr.ble_1mbit[1].read().bits()));
                radio
                    .override2
                    .write(|w| w.override2().bits(ficr.ble_1mbit[2].read().bits()));
                radio
                    .override3
                    .write(|w| w.override3().bits(ficr.ble_1mbit[3].read().bits()));
                radio.override4.write(|w| {
                    w.override4()
                        .bits(ficr.ble_1mbit[4].read().bits())
                        .enable()
                        .set_bit()
                });
            }
        }
        Self::initialize(radio, access_address)
    }

    pub(crate) fn init(
        radio: pac::RADIO,
        access_address: u32,
    ) -> Self {
        // make sure this only initializes once
        Self::initialize(radio, access_address)
    }

    fn initialize(
        radio: pac::RADIO,
        access_address: u32,
    ) -> Hci {
        // TODO: figure out what this actually does/can do
        radio.mode.write(|w| w.mode().ble_1mbit());
        radio.txpower.write(|w| w.txpower().pos4d_bm());

        static RX_BUFFER: &'static[u8; u8::MAX as usize] = &[0; u8::MAX as usize];
        unsafe {
            radio.pcnf1.write(|w| {
                w.maxlen()
                .bits(RX_BUFFER.len() as u8)
                .balen()
                .bits(3)
                .whiteen()
                .set_bit()
            });
        }
        radio.crccnf.write(|w| {
            // skip address since only the S0, Length, S1 and Payload need CRC
            // 3 Bytes = CRC24
            w.skipaddr().skip().len().three()
        });
        unsafe {
            const CRC_POLY: u32 = 0b00000001_00000000_00000110_01011011;
            radio.crcpoly
                .write(|w| w.crcpoly().bits(CRC_POLY & 0x00FFFFFF));
        }

        // Configure logical address 0 as the canonical advertising address.
        // Base addresses are up to 32 bits in size. However, an 8 bit Address Prefix is
        // *always* appended, so we must use a 24 bit Base Address and the 8 bit Prefix.
        // BASE0 has, apparently, undocumented semantics: It is a proper 32-bit register, but
        // it ignores the *lowest* 8 bit and instead transmits the upper 24 as the low 24 bits
        // of the Access Address. Shift address up to fix this.
        unsafe {
            radio.base0.write(|w| w.bits(access_address << 8));
            radio.prefix0.write(|w| w.ap0().bits((access_address >> 24) as u8));
        }

        // Configure shortcuts to simplify and speed up sending and receiving packets.
        radio.shorts.write(|w| {
            // start transmission/recv immediately after ramp-up
            // disable radio when transmission/recv is done
            w.ready_start().enabled().end_disable().enabled()
        });

        // TODO: enable TIFS to track time interval between packets

        Hci{ radio }
    }

    pub fn state(&self) -> STATE_R {
        self.radio.state.read().state()
    }

    pub fn transmit(&mut self, buffer:&[u8]) {
        let buffer_ptr = buffer.as_ptr();
        unsafe {
            // "The CPU should reconfigure this pointer every time before the RADIO is started via
            // the START task."
            self.radio
                .packetptr
                .write(|w| w.bits(buffer.as_ptr() as u32));

            // Acknowledge left-over disable event
            self.radio.events_disabled.reset(); // FIXME unnecessary, right?

            // "Preceding reads and writes cannot be moved past subsequent writes."
            // compiler_fence(Ordering::Release);

            // ...and kick off the transmission
            self.radio.tasks_txen.write(|w| w.bits(1));

            // Then wait until disable event is triggered
            while self.radio.events_disabled.read().bits() == 0 {}

            // "Subsequent reads and writes cannot be moved ahead of preceding reads."
            // compiler_fence(Ordering::Acquire);

            // Now our `tx_buf` can be used again.
        }
    }

}