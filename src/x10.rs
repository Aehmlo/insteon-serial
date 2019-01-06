//! Utilities for X10 messages over Insteon's network.

use std::fmt;

/// The house code for the X10 message (A–P).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HouseCode {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
}

impl HouseCode {
    /// Attempts to convert the argument to a house code.
    pub fn try_from<T: Into<u8>>(byte: T) -> Option<Self> {
        use self::HouseCode::*;
        match byte.into() {
            0x6 => Some(A),
            0xE => Some(B),
            0x2 => Some(C),
            0xA => Some(D),
            0x1 => Some(E),
            0x9 => Some(F),
            0x5 => Some(G),
            0xD => Some(H),
            0x7 => Some(I),
            0xF => Some(J),
            0x3 => Some(K),
            0xB => Some(L),
            0x0 => Some(M),
            0x8 => Some(N),
            0x4 => Some(O),
            0xC => Some(P),
            _ => None,
        }
    }
}

impl Into<char> for HouseCode {
    fn into(self) -> char {
        use self::HouseCode::*;
        match self {
            A => 'A',
            B => 'B',
            C => 'C',
            D => 'D',
            E => 'E',
            F => 'F',
            G => 'G',
            H => 'H',
            I => 'I',
            J => 'J',
            K => 'K',
            L => 'L',
            M => 'M',
            N => 'N',
            O => 'O',
            P => 'P',
        }
    }
}

/// The unit code for an X10 message.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UnitCode(pub u8);

impl UnitCode {
    /// Attempts to convert the argument to a unit code.
    pub fn try_from<T: Into<u8>>(byte: T) -> Option<Self> {
        match byte.into() {
            0x6 => Some(UnitCode(1)),
            0xE => Some(UnitCode(2)),
            0x2 => Some(UnitCode(3)),
            0xA => Some(UnitCode(4)),
            0x1 => Some(UnitCode(5)),
            0x9 => Some(UnitCode(6)),
            0x5 => Some(UnitCode(7)),
            0xD => Some(UnitCode(8)),
            0x7 => Some(UnitCode(9)),
            0xF => Some(UnitCode(10)),
            0x3 => Some(UnitCode(11)),
            0xB => Some(UnitCode(12)),
            0x0 => Some(UnitCode(13)),
            0x8 => Some(UnitCode(14)),
            0x4 => Some(UnitCode(15)),
            0xC => Some(UnitCode(16)),
            _ => None,
        }
    }
}

impl Into<u8> for UnitCode {
    fn into(self) -> u8 {
        self.0
    }
}

impl fmt::Display for UnitCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// An X10 command.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Command {
    AllLightsOff,
    StatusOff,
    On,
    PresetDim,
    AllLightsOn,
    HailAcknowledge,
    Bright,
    StatusOn,
    ExtendedCode,
    StatusRequest,
    Off,
    AllUnitsOff,
    HailRequest,
    Dim,
    ExtendedAnalogData,
}

impl Command {
    /// Attempts to convert the argument to an X10 command.
    pub fn try_from<T: Into<u8>>(byte: T) -> Option<Self> {
        match byte.into() {
            0x6 => Some(Command::AllLightsOff),
            0xE => Some(Command::StatusOff),
            0x2 => Some(Command::On),
            0xA => Some(Command::PresetDim),
            0x1 => Some(Command::AllLightsOn),
            0x9 => Some(Command::HailAcknowledge),
            0x5 => Some(Command::Bright),
            0xD => Some(Command::StatusOn),
            0x7 => Some(Command::ExtendedCode),
            0xF => Some(Command::StatusRequest),
            0x3 => Some(Command::Off),
            0xB => Some(Command::PresetDim),
            0x0 => Some(Command::AllUnitsOff),
            0x8 => Some(Command::HailRequest),
            0x4 => Some(Command::Dim),
            0xC => Some(Command::ExtendedAnalogData),
            _ => None,
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Command::AllLightsOff => "All Lights Off",
                Command::StatusOff => "Status = Off",
                Command::On => "On",
                Command::PresetDim => "Preset Dim",
                Command::AllLightsOn => "All Lights On",
                Command::HailAcknowledge => "Hail Acknowledge",
                Command::Bright => "Bright",
                Command::StatusOn => "Status = On",
                Command::ExtendedCode => "Extended Code",
                Command::StatusRequest => "Status Request",
                Command::Off => "Off",
                Command::AllUnitsOff => "All Units Off",
                Command::HailRequest => "Hail Request",
                Command::Dim => "Dim",
                Command::ExtendedAnalogData => "Extended Data (analog)",
            }
        )
    }
}

/// The X10 message payload; either a unit code or a command.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Payload {
    UnitCode(UnitCode),
    Command(Command),
}

/// An X10 message, as communicated by Insteon's network.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Message {
    /// The house code from the message.
    pub house: HouseCode,
    /// The message payload (either a unit code or a command).
    pub payload: Payload,
    /// Whether the command was successful.
    pub success: bool,
}

impl Message {
    /// Attempts to parse the given byte array as an X10 message.
    pub fn try_from<B: Into<[u8; 3]>>(bytes: B) -> Option<Self> {
        let bytes = bytes.into();
        let byte = bytes[0];
        let high = byte >> 4;
        let low = byte & 0x0f;
        let flag = bytes[1];
        let success = bytes[2] == 0x06;
        HouseCode::try_from(high).and_then(|house| match flag {
            0x00 => UnitCode::try_from(low).map(|unit| Message {
                house,
                payload: Payload::UnitCode(unit),
                success,
            }),
            0x80 => Command::try_from(low).map(|cmd| Message {
                house,
                payload: Payload::Command(cmd),
                success,
            }),
            _ => None,
        })
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let h: char = self.house.into();
        let status = if self.success { "succeeded" } else { "failed" };
        match &self.payload {
            Payload::UnitCode(unit) => write!(f, "{}{} {}", h, unit, status),
            Payload::Command(cmd) => write!(f, "Command {} ({}) {}", cmd, h, status),
        }
    }
}
