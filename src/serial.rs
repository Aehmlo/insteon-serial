//! Utilities for communicating with a modem over a serial port.

use crate::command::Command;
use crate::message::*;
use serialport::SerialPort;

/// Gets the next byte from the given port.
pub fn next_byte(port: &mut dyn SerialPort) -> Option<u8> {
    while let Ok(n) = port.bytes_to_read() {
        if n == 0 {
            continue;
        }
        let mut buf = [0];
        return port.read_exact(&mut buf).ok().map(|_| buf[0]);
    }
    None
}

/// Reads the next full message from the given port.
pub fn next_message(port: &mut dyn SerialPort) -> Option<Message> {
    use self::Message::*;
    Some(loop {
        // All messages start with 0x02, so if we're not seeing that, we're in the middle of a
        // message and should wait it out.
        if next_byte(port).unwrap() == 0x02 {
            break match next_byte(port).unwrap() {
                0x50 => {
                    let mut buf = [0; 6];
                    port.read_exact(&mut buf).unwrap();
                    Received(
                        [buf[0], buf[1], buf[2]].into(),
                        Command::try_from([buf[3], buf[4]]),
                        buf[5],
                        None,
                    )
                }
                0x51 => {
                    let mut buf = [0; 20];
                    port.read_exact(&mut buf).unwrap();
                    let data = [
                        buf[6], buf[7], buf[8], buf[9], buf[10], buf[11], buf[12], buf[13],
                        buf[14], buf[15], buf[16], buf[17], buf[18], buf[19],
                    ];
                    Received(
                        [buf[0], buf[1], buf[2]].into(),
                        Command::try_from([buf[3], buf[4]]),
                        buf[5],
                        Some(data),
                    )
                }
                0x52 => {
                    let mut buf = [0; 3];
                    port.read_exact(&mut buf).unwrap();
                    if let Some(msg) = crate::x10::Message::try_from(buf) {
                        X10Received(msg)
                    } else {
                        continue;
                    }
                }
                0x53 => {
                    let mut buf = [0; 8];
                    port.read_exact(&mut buf).unwrap();
                    LinkComplete(buf.into())
                }
                0x54 => match next_byte(port).unwrap() {
                    0x02 => ButtonEvent(self::ButtonEvent::Tapped(self::Button::Set)),
                    0x03 => ButtonEvent(self::ButtonEvent::Held(self::Button::Set)),
                    0x04 => ButtonEvent(self::ButtonEvent::Released(self::Button::Set)),
                    0x12 => ButtonEvent(self::ButtonEvent::Tapped(self::Button::Two)),
                    0x13 => ButtonEvent(self::ButtonEvent::Held(self::Button::Two)),
                    0x14 => ButtonEvent(self::ButtonEvent::Released(self::Button::Two)),
                    0x22 => ButtonEvent(self::ButtonEvent::Tapped(self::Button::Three)),
                    0x23 => ButtonEvent(self::ButtonEvent::Held(self::Button::Three)),
                    0x24 => ButtonEvent(self::ButtonEvent::Released(self::Button::Three)),
                    _ => {
                        continue;
                    }
                },
                0x55 => UserResetDetected,
                0x56 => {
                    // The next byte is always 0x01, so we don't need to worry about it.
                    let _ = next_byte(port).unwrap();
                    let mut buf = [0; 4];
                    port.read_exact(&mut buf).unwrap();
                    LinkCleanupFailed(buf[0], [buf[1], buf[2], buf[3]].into())
                }
                0x57 => {
                    let flags = next_byte(port).unwrap();
                    let group = next_byte(port).unwrap();
                    let mut id = [0; 3];
                    port.read_exact(&mut id).unwrap();
                    let mut link = [0; 3];
                    port.read_exact(&mut link).unwrap();
                    LinkRecordResponse(flags, group, id.into(), link.into())
                }
                0x58 => LinkCleanupStatus(next_byte(port).unwrap() == 0x06),
                0x59 => {
                    let mut address = [0; 2];
                    port.read_exact(&mut address).unwrap();
                    let flags = next_byte(port).unwrap();
                    let group = next_byte(port).unwrap();
                    let mut id = [0; 3];
                    port.read_exact(&mut id).unwrap();
                    let mut link = [0; 3];
                    port.read_exact(&mut link).unwrap();
                    DatabaseRecordFound(address, flags, group, id.into(), link.into())
                }
                _ => {
                    continue;
                }
            };
        }
    })
}

/// Reads the next response from the given port. Non-response messages are ignored.
pub fn next_response(port: &mut dyn SerialPort) -> Option<Response> {
    use self::Response::*;
    Some(loop {
        // Wait for start byte
        if next_byte(port).unwrap() == 0x02 {
            break match next_byte(port).unwrap() {
                0x60 => {
                    let mut buf = [0; 6];
                    port.read_exact(&mut buf).unwrap();
                    let address = [buf[0], buf[1], buf[2]].into();
                    let category = [buf[3], buf[4]];
                    let version = buf[5];
                    let version = if version == 0xFF { None } else { Some(version) };
                    GotInfo(address, category, version)
                }
                0x61 => {
                    let mut buf = [0; 3];
                    port.read_exact(&mut buf).unwrap();
                    let group = buf[0];
                    let command = buf[1];
                    let broadcast = buf[2];
                    SentLinkCommand(group, command, broadcast)
                }
                0x62 => unimplemented!(),
                0x63 => {
                    let mut buf = [0; 3];
                    port.read_exact(&mut buf).unwrap();
                    if let Some(msg) = crate::x10::Message::try_from(buf) {
                        SentX10(msg)
                    } else {
                        continue;
                    }
                }
                0x64 => {
                    let mut buf = [0; 2];
                    port.read_exact(&mut buf).unwrap();
                    let role = buf[0];
                    let group = buf[1];
                    StartedLink(role, group)
                }
                0x65 => CanceledLink,
                0x66 => {
                    let mut buf = [0; 3];
                    port.read_exact(&mut buf).unwrap();
                    let category = [buf[0], buf[1]];
                    let firmware = buf[2];
                    let firmware = if firmware == 0x00 {
                        None
                    } else {
                        Some(firmware)
                    };
                    SetCategory(category, firmware)
                }
                0x67 => Reset,
                0x68 => SetAckByte(next_byte(port).unwrap()),
                0x69 => GotFirstLinkRecord,
                0x6A => GotNextLinkRecord,
                0x6B => SetConfig(next_byte(port).unwrap().into()),
                0x6C => GotSenderLinkRecord,
                0x6D => LedOn,
                0x6E => LedOff,
                0x6F => {
                    let mut buf = [0; 9];
                    port.read_exact(&mut buf).unwrap();
                    let control = buf[0];
                    let record = buf[1];
                    let group = buf[2];
                    let addr = [buf[3], buf[4], buf[5]].into();
                    let link = [buf[6], buf[7], buf[8]];
                    UpdatedLinkRecord(control, record, group, addr, link)
                }
                0x70 => SetNakByte(next_byte(port).unwrap()),
                0x71 => {
                    let mut buf = [0; 2];
                    port.read_exact(&mut buf).unwrap();
                    SetAckBytes([buf[0], buf[1]])
                }
                0x72 => Sleeping,
                0x73 => {
                    let mut buf = [0; 3];
                    port.read_exact(&mut buf).unwrap();
                    let config = buf[0].into();
                    let _ = buf[1];
                    let _ = buf[2];
                    GotConfig(config)
                }
                _ => {
                    continue;
                }
            };
        }
    })
}
