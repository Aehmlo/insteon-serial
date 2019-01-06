use std::fmt;

/// Represents a button on an Insteon device.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Button {
    /// The SET button.
    Set,
    /// A secondary button.
    Two,
    /// A tertiary button.
    Three,
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Button::*;
        write!(
            f,
            "{}",
            match self {
                Set => "SET button",
                Two => "Button two",
                Three => "Button three",
            }
        )
    }
}

/// An event related to buttons on a device.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ButtonEvent {
    /// The button was tapped.
    Tapped(Button),
    /// The button was held.
    Held(Button),
    /// The button was held and released.
    Released(Button),
}

impl fmt::Display for ButtonEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ButtonEvent::Tapped(button) => write!(f, "{} tapped.", button),
            ButtonEvent::Held(button) => write!(f, "{} held.", button),
            ButtonEvent::Released(button) => write!(f, "{} released.", button),
        }
    }
}
