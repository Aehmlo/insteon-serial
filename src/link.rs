use crate::device::Address;
use crate::message::Group;

/// Stores link data from link messages.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LinkData {
    pub data: [u8; 3],
}

impl From<[u8; 3]> for LinkData {
    fn from(data: [u8; 3]) -> Self {
        Self { data }
    }
}

/// Encodes the result of a linking attempt.

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LinkResult {
    is_controller: Option<bool>,
    group: Group,
    id: Address,
    category: [u8; 2],
    firmware: Option<u8>,
}

impl LinkResult {
    /// The group number assigned to this link.
    pub fn group(&self) -> Group {
        self.group
    }
    /// Whether the modem is a slave to this device.
    pub fn is_slave(&self) -> bool {
        self.is_controller == Some(false)
    }
    /// Identical to `is_slave`, but using Insteon's nomenclature.
    pub fn is_responder(&self) -> bool {
        self.is_slave()
    }
    /// Whether the modem is a master of this device.
    pub fn is_master(&self) -> bool {
        self.is_controller == Some(true)
    }
    /// Identical to `is_master`, but using Insteon's nomenclature.
    pub fn is_controller(&self) -> bool {
        self.is_master()
    }
    /// Whether the link was deleted.
    pub fn deleted(&self) -> bool {
        self.is_controller.is_none()
    }
    /// The ID of the device that was linked.
    pub fn id(&self) -> Address {
        self.id
    }
    /// The category of the slave (responder) device that was linked.
    ///
    /// ## Notes
    ///
    /// This byte is just junk when the modem is not the master (controller), so `None` is
    /// returned in the case that the modem is the slave (responder).
    pub fn category(&self) -> Option<u8> {
        if self.is_controller() {
            Some(self.category[0])
        } else {
            None
        }
    }
    /// The subcategory of the slave (responder) device that was linked.
    ///
    /// ## Notes
    ///
    /// This byte is just junk when the modem is not the master (controller), so `None` is
    /// returned in the case that the modem is the slave (responder).
    pub fn subcategory(&self) -> Option<u8> {
        if self.is_controller() {
            Some(self.category[1])
        } else {
            None
        }
    }
    /// The firmware version of the slave (responder) that was linked.
    ///
    /// ## Notes
    ///
    /// This method is only useful when:
    /// 1. the modem is the master (controller) and
    /// 2. the device is old (newer devices do not report firmware versions).
    ///
    /// This makes this method pretty much useless.
    pub fn firmware(&self) -> Option<u8> {
        if self.is_controller() {
            self.firmware
        } else {
            None
        }
    }
}

impl From<[u8; 8]> for LinkResult {
    fn from(bytes: [u8; 8]) -> Self {
        let is_controller = match bytes[0] {
            0x00 => Some(false),
            0x01 => Some(true),
            0xFF | _ => None,
        };
        let group = bytes[1];
        let address = [bytes[2], bytes[3], bytes[4]].into();
        let category = [bytes[5], bytes[6]];
        let vers = bytes[7];
        let firmware = if vers == 0xFF { None } else { Some(vers) };
        Self {
            is_controller,
            group,
            id: address,
            category,
            firmware,
        }
    }
}
