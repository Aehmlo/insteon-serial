//! Commands to be executed by the recipient.

/// An ALL-Link group number.
///
/// This structure and `message::Group` will be consolidated into a better structure at a later point.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GroupNumber(pub u8);

/// The "on level" associated with an on command.
///
/// It's not clear from the documentation what this really means.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OnLevel(pub u8);

/// Encodes a desired movement direction for dimming.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BrightDim {
    /// Move in the direction of increasing brightness.
    Bright,
    /// Move in the direction of decreasing brightness.
    Dim,
}

/// The payload of an on command; either a group number or an "on level."
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OnPayload {
    GroupNumber(GroupNumber),
    OnLevel(OnLevel),
}

/// Encodes a command to be faithfully executed by the recipient.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Command {
    On(OnPayload),
    FastOn(Option<GroupNumber>),
    Off(Option<GroupNumber>),
    FastOff(Option<GroupNumber>),
    Bright(Option<GroupNumber>),
    Dim(Option<GroupNumber>),
    Start(BrightDim),
    Stop,
    IdRequest,
    StatusRequest,
    BeginLinking(GroupNumber),
    BeginUnlinking(GroupNumber),
    CancelLinking,
}

fn group_or_none(byte: u8) -> Option<GroupNumber> {
    if byte == 0 {
        None
    } else {
        Some(GroupNumber(byte))
    }
}

impl Command {
    /// Attempts to parse a pair of bytes as a command.
    pub fn try_from(bytes: [u8; 2]) -> Option<Self> {
        use self::Command::*;
        match bytes[0] {
            0x11 => Some(On(OnPayload::GroupNumber(GroupNumber(bytes[1])))),
            0x12 => Some(FastOn(group_or_none(bytes[1]))),
            0x13 => Some(Off(group_or_none(bytes[1]))),
            0x14 => Some(FastOff(group_or_none(bytes[1]))),
            0x15 => Some(Bright(group_or_none(bytes[1]))),
            0x16 => Some(Dim(group_or_none(bytes[1]))),
            0x17 => {
                let dir = match bytes[1] {
                    0x01 => BrightDim::Bright,
                    _ => BrightDim::Dim,
                };
                Some(Start(dir))
            }
            0x18 => Some(Stop),
            0x10 => Some(IdRequest),
            0x19 => Some(StatusRequest),
            0x09 => Some(BeginLinking(GroupNumber(bytes[1]))),
            0x0A => Some(BeginUnlinking(GroupNumber(bytes[1]))),
            0x08 => Some(CancelLinking),
            _ => None,
        }
    }
}

fn group_or_zero(group: Option<GroupNumber>) -> u8 {
    group.map(|g| g.0).unwrap_or(0)
}

impl Into<[u8; 2]> for Command {
    fn into(self) -> [u8; 2] {
        use self::Command::*;
        match self {
            On(payload) => {
                let two = match payload {
                    OnPayload::OnLevel(level) => level.0,
                    OnPayload::GroupNumber(group) => group.0,
                };
                [0x11, two]
            }
            FastOn(group) => [0x12, group_or_zero(group)],
            Off(group) => [0x13, group_or_zero(group)],
            FastOff(group) => [0x14, group_or_zero(group)],
            Bright(group) => [0x15, group_or_zero(group)],
            Dim(group) => [0x16, group_or_zero(group)],
            Start(dir) => {
                let two = match dir {
                    BrightDim::Bright => 0x01,
                    BrightDim::Dim => 0x00,
                };
                [0x17, two]
            }
            Stop => [0x18, 0],
            IdRequest => [0x10, 0],
            StatusRequest => [0x19, 0],
            BeginLinking(group) => [0x09, group.0],
            BeginUnlinking(group) => [0x0A, group.0],
            CancelLinking => [0x08, 0],
        }
    }
}
