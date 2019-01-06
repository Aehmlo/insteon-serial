//! Constructs for communication with the modem.

use std::fmt;

pub use crate::button::*;
use crate::command::Command;
use crate::device::Address;
pub use crate::link::*;
use crate::x10::Message as X10Message;

// TODO: Make this a real type.
/// Represents an ALL-Link device group.
pub type Group = u8;

/// Messages are notifications delivered by the modem to us.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Message {
    /// A message (either standard or extended) was received.
    ///
    /// This message type is merely an acknowledgement.
    Received(Address, Option<Command>, u8, Option<[u8; 14]>),
    /// An X10 message was received.
    X10Received(X10Message),
    /// An ALL-Link event completed.
    LinkComplete(LinkResult),
    /// A button on the device was pressed, held, or released.
    ButtonEvent(ButtonEvent),
    /// The user reset the modem (by pushing and holding SET while powering).
    UserResetDetected,
    /// A requested ALL-Link cleanup failed.
    LinkCleanupFailed(Group, Address),
    /// An ALL-Link record response.
    LinkRecordResponse(u8, Group, Address, crate::link::LinkData),
    /// The All-Link cleanup completed (successfully or not).
    LinkCleanupStatus(bool),
    DatabaseRecordFound([u8; 2], u8, u8, Address, crate::link::LinkData),
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Message::*;
        match self {
            Received(_addr, _flags, _cmds, data) => match data {
                None => write!(f, "Received standard message."),
                Some(msg) => write!(f, "Received extended message with data: {:x?}", msg),
            },
            X10Received(msg) => write!(f, "Received X10 result: {}", msg),
            LinkComplete(_result) => write!(f, "ALL-Link completed (details omitted)."),
            ButtonEvent(e) => write!(f, "{}", e),
            UserResetDetected => write!(f, "User reset initiated."),
            LinkCleanupFailed(group, address) => write!(
                f,
                "ALL-Link cleanup failed (group {:x}, id {:x?})",
                group, address
            ),
            LinkRecordResponse(_flags, _group, _id, _link) => {
                write!(f, "ALL-Link record received (omitted).")
            }
            LinkCleanupStatus(finished) => {
                if *finished {
                    write!(f, "ALL-Link cleanup completed.")
                } else {
                    write!(f, "ALL-Link cleanup aborted due to traffic.")
                }
            }
            DatabaseRecordFound(_address, _flags, _group, _id, _link) => {
                write!(f, "Database record found (omitted).")
            }
        }
    }
}

/// Encodes a modem configuration.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Config {
    /// Whether linking should be initiated when the user presses and holds the SET button.
    pub auto_link: bool,
    /// Whether the modem is in monitor mode.
    pub monitor: bool,
    /// Whether the LED should be managed by the host (and not the modem).
    pub manual_led: bool,
    /// Whether the modem should timeout after 240 ms.
    pub timeout: bool,
    /// Whether the modem should reject commands (NAK) if it's busy processing.
    pub busy_reject: bool,
}

impl Into<u8> for Config {
    fn into(self) -> u8 {
        ((!self.auto_link as u8) << 7)
            | ((self.monitor as u8) << 6)
            | ((self.manual_led as u8) << 5)
            | ((!self.timeout as u8) << 4)
            | ((self.busy_reject as u8) << 3)
    }
}

impl From<u8> for Config {
    fn from(byte: u8) -> Self {
        Self {
            auto_link: byte & 0b1000_0000 == 0,
            monitor: byte & 0b0100_0000 == 0b0100_0000,
            manual_led: byte & 0b0010_0000 == 0b0010_0000,
            timeout: byte & 0b0001_0000 == 0,
            busy_reject: byte & 0b0000_1000 == 0b0000_1000,
        }
    }
}

impl Default for Config {
    /// Returns the default configuration (`0`).
    fn default() -> Self {
        Config {
            auto_link: true,
            monitor: false,
            manual_led: false,
            timeout: true,
            busy_reject: false,
        }
    }
}

/// Responses are delivered from the modem to us in response to issued commands.
///
/// They therefore differ in significance from messages because we request and expect them.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Response {
    /// The device info was retrieved.
    GotInfo(Address, [u8; 2], Option<u8>),
    /// The requested link command was sent to the group.
    SentLinkCommand(Group, u8, u8),
    // TODO: Fix this
    SentMessage(Message),
    /// The requested X10 message was sent.
    SentX10(X10Message),
    StartedLink(u8, Group),
    CanceledLink,
    /// The host device category (and subcategory) were successfully set.
    ///
    /// If applicable, this command also returns the firmware version.
    SetCategory([u8; 2], Option<u8>),
    /// The modem was successfully reset to factory settings, wiping the ALL-Link database.
    Reset,
    /// The ACK byte (`0x06`) will be followed by the requested (and returned) byte.
    ///
    /// This is required for some direct commands.
    SetAckByte(u8),
    /// The first ALL-Link record was retrieved and will follow in an ALL-Link Record Response
    /// message (`0x57`).
    GotFirstLinkRecord,
    /// The next ALL-Link record was retrieved and will follow in an ALL-Link Record Response
    /// message (`0x57`).
    GotNextLinkRecord,
    /// The modem configuration was set as specified.
    SetConfig(Config),
    /// The ALL-Link record for the most recent known message sender was retrieved and will follow
    /// in an ALL-Link Record Response Message (`0x57`).
    GotSenderLinkRecord,
    /// The LED was turned on.
    LedOn,
    /// The LED was turned off.
    LedOff,
    /// The specified ALL-Link record was inserted into the database.
    UpdatedLinkRecord(u8, u8, Group, Address, [u8; 3]),
    /// The NAK byte (`0x15`) will be followed by the requested (and returned) byte.
    SetNakByte(u8),
    /// The ACK byte (`0x06`) will be followed by the requested (and returned) bytes.
    ///
    /// This is required for some direct commands.
    SetAckBytes([u8; 2]),
    /// The RF modem was put to sleep and will wake again when sent a byte.
    Sleeping,
    /// Returns the configuration flags for the modem.
    ///
    /// ## Notes
    /// There are two bytes of reserved data that come in this response as well, but this
    /// implementation ignores them, returning only the first field (the configuration byte),
    /// parsed into a convenient format.
    GotConfig(Config),

    // The following commands were added to the spec after the initial release.
    /// ALL-Link cleanup was successfully canceled.
    CanceledCleanup,
    /// Eight bytes were read from the database successfully and will follow in a Database Record
    /// Found Response Message (`0x59`).
    ReadDatabaseBytes([u8; 2]),
    /// The device will beep.
    Beeping,
    SetStatus(u8),

    // The following commands are RF modem-only.
    // It is left unclear whether they are in the initial version of the spec,
    // but looking at their numbering, let's assume not.
    SetLinkData([u8; 3]),
    /// The number of application retries for new links was set as specified.
    SetRetries(u8),
    /// The RF frequency offset was set as requested.
    SetFrequencyOffset(u8),
    /// This message is woefully underdocumented, but its full name is "Set Acknowledge for
    /// TempLinc command."
    ///
    /// Godspeed.
    SetTempLincAck(u8),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_config() {
        let cfg = Config::default();
        let byte: u8 = cfg.into();
        assert_eq!(byte, 0);
        let mut cfg = Config::default();
        cfg.timeout = false;
        let byte: u8 = cfg.into();
        assert!(byte != 0);
    }
}
