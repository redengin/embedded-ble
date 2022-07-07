#![cfg_attr(not(test), no_std)]

pub trait BleController: Sync {
    fn send(&self, data: &[u8]) -> Result<usize, &'static str>;
}
