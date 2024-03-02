// pub enum KeyEventExt {
//     KeyEvent(KeyEvent),
//     AnyChar(char),
// }
//
// impl From<KeyEvent> for KeyEventExt {
//     fn from(value: KeyEvent) -> Self {
//         Self::KeyEvent(value)
//     }
// }
//
/// A key event.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Hash)]
pub struct KeyEvent {
    /// The key code.
    pub code: KeyCode,
    /// The key modifier.
    pub modifiers: KeyModifiers,
}

impl KeyEvent {
    /// Creates a new `KeyEvent` instance.
    pub fn new(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: KeyModifiers::new(),
        }
    }

    /// Creates a new `KeyEvent` instance from a KeyCode.
    pub fn modifier(mut self, modifier: KeyModifier) -> Self {
        self.modifiers.add_modifier(modifier);
        self
    }
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KeyModifier {
    // Shift,
    Control,
    Alt,
    Super,
    Hyper,
    Meta,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Hash, Default)]
pub struct KeyModifiers(u32);

impl KeyModifiers {
    pub fn new() -> Self {
        KeyModifiers(0)
    }

    pub fn add_modifier(&mut self, modifier: KeyModifier) {
        match modifier {
            // KeyModifier::Shift => self.0 |= 0b0000_0001,
            KeyModifier::Control => self.0 |= 0b0000_0010,
            KeyModifier::Alt => self.0 |= 0b0000_0100,
            KeyModifier::Super => self.0 |= 0b0000_1000,
            KeyModifier::Hyper => self.0 |= 0b0001_0000,
            KeyModifier::Meta => self.0 |= 0b0010_0000,
        }
    }

    pub fn remove_modifier(&mut self, modifier: KeyModifier) {
        match modifier {
            // KeyModifier::Shift => self.0 &= !0b0000_0001,
            KeyModifier::Control => self.0 &= !0b0000_0010,
            KeyModifier::Alt => self.0 &= !0b0000_0100,
            KeyModifier::Super => self.0 &= !0b0000_1000,
            KeyModifier::Hyper => self.0 &= !0b0001_0000,
            KeyModifier::Meta => self.0 &= !0b0010_0000,
        }
    }

    pub fn contains(&self, modifier: KeyModifier) -> bool {
        match modifier {
            // KeyModifier::Shift => self.0 & 0b0000_0001 != 0,
            KeyModifier::Control => self.0 & 0b0000_0010 != 0,
            KeyModifier::Alt => self.0 & 0b0000_0100 != 0,
            KeyModifier::Super => self.0 & 0b0000_1000 != 0,
            KeyModifier::Hyper => self.0 & 0b0001_0000 != 0,
            KeyModifier::Meta => self.0 & 0b0010_0000 != 0,
        }
    }
}

/// Represents a key.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KeyCode {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Char(char),
    Null,
    Esc,
    Unknown,
}
