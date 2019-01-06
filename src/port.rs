//! Serial port utilities.
use std::ffi::OsStr;

use serialport::prelude::*;

/// Opens the named serial port with appropriate settings.
pub fn open<S: AsRef<OsStr>>(
    name: S,
) -> Result<Box<dyn serialport::SerialPort>, serialport::Error> {
    let mut settings = SerialPortSettings::default();
    settings.baud_rate = 19_200;
    settings.timeout = std::time::Duration::from_millis(500);
    serialport::open_with_settings(&name, &settings)
}
