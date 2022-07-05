#![cfg_attr(not(test), no_std)]

pub trait BleController {
    fn send(&self, data: &[u8]) -> Result<usize, &'static str>;
}