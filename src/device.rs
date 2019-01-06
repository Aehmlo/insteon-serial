//! Insteon device properties.

use std::{fmt, ops::Index};

/// A device address.
///
/// The ordering of the address bytes is always high, middle, low.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Address([u8; 3]);

impl Address {
    /// Returns the (left-padded) combination of the three address bytes.
    pub fn reduce(self) -> u32 {
        let bytes = self.0;
        let (high, middle, low) = (
            u32::from(bytes[0]),
            u32::from(bytes[1]),
            u32::from(bytes[2]),
        );
        (high << 16) + (middle << 8) + low
    }
}

impl From<[u8; 3]> for Address {
    fn from(bytes: [u8; 3]) -> Self {
        Address(bytes)
    }
}

impl Index<usize> for Address {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl fmt::Display for Address {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}
