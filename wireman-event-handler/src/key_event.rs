use std::fmt;

/// A key event.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Hash)]
pub struct KeyEvent {
    /// The key code.
    pub code: KeyCode,
    /// The key modifier.
    pub modifiers: KeyModifiers,
}

impl fmt::Display for KeyEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = format!("{}", self.code);
        let modifiers = self.modifiers.iter().map(|m| match m {
            KeyModifier::Shift => "",
            KeyModifier::Control => "C-",
            KeyModifier::Alt => "A-",
            KeyModifier::Super => "Su-",
            KeyModifier::Hyper => "H-",
            KeyModifier::Meta => "M-",
        });
        let mods = modifiers.collect::<String>();

        write!(f, "{mods}{code}",)
    }
}

impl KeyEvent {
    /// Creates a new `KeyEvent` instance.
    pub fn new(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: KeyModifiers::new(),
        }
    }
    /// Creates a new `KeyEvent` instance with the shift modifier.
    pub fn shift(code: KeyCode) -> Self {
        Self {
            modifiers: KeyModifiers::shift(),
            code,
        }
    }

    /// Creates a new `KeyEvent` instance with the ctrl modifier.
    pub fn ctrl(code: KeyCode) -> Self {
        Self {
            modifiers: KeyModifiers::ctrl(),
            code,
        }
    }

    /// Creates a new `KeyEvent` instance from a KeyCode.
    pub fn modifier(mut self, modifier: KeyModifier) -> Self {
        self.modifiers = self.modifiers.add_modifier(modifier);
        self
    }
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum KeyModifier {
    Shift,
    Control,
    Alt,
    Super,
    Hyper,
    Meta,
}

impl fmt::Display for KeyModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyModifier::Shift => write!(f, "Shift"),
            KeyModifier::Control => write!(f, "Ctrl"),
            KeyModifier::Alt => write!(f, "Alt"),
            KeyModifier::Super => write!(f, "Super"),
            KeyModifier::Hyper => write!(f, "Hyper"),
            KeyModifier::Meta => write!(f, "Meta"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Hash, Default)]
pub struct KeyModifiers(u32);

impl KeyModifiers {
    /// Creates a new `KeyModifiers` instance with no modifiers.
    pub fn new() -> Self {
        KeyModifiers(0)
    }

    /// Creates a new `KeyModifiers` instance with the shift modifier.
    pub fn shift() -> Self {
        KeyModifiers(0).add_modifier(KeyModifier::Shift)
    }

    /// Creates a new `KeyModifiers` instance with the ctrl modifier.
    pub fn ctrl() -> Self {
        KeyModifiers(0).add_modifier(KeyModifier::Control)
    }

    pub fn add_modifier(mut self, modifier: KeyModifier) -> Self {
        match modifier {
            KeyModifier::Shift => self.0 |= 0b0000_0001,
            KeyModifier::Control => self.0 |= 0b0000_0010,
            KeyModifier::Alt => self.0 |= 0b0000_0100,
            KeyModifier::Super => self.0 |= 0b0000_1000,
            KeyModifier::Hyper => self.0 |= 0b0001_0000,
            KeyModifier::Meta => self.0 |= 0b0010_0000,
        };
        self
    }

    pub fn remove_modifier(mut self, modifier: KeyModifier) -> Self {
        match modifier {
            KeyModifier::Shift => self.0 &= !0b0000_0001,
            KeyModifier::Control => self.0 &= !0b0000_0010,
            KeyModifier::Alt => self.0 &= !0b0000_0100,
            KeyModifier::Super => self.0 &= !0b0000_1000,
            KeyModifier::Hyper => self.0 &= !0b0001_0000,
            KeyModifier::Meta => self.0 &= !0b0010_0000,
        };
        self
    }

    pub fn contains(&self, modifier: KeyModifier) -> bool {
        match modifier {
            KeyModifier::Shift => self.0 & 0b0000_0001 != 0,
            KeyModifier::Control => self.0 & 0b0000_0010 != 0,
            KeyModifier::Alt => self.0 & 0b0000_0100 != 0,
            KeyModifier::Super => self.0 & 0b0000_1000 != 0,
            KeyModifier::Hyper => self.0 & 0b0001_0000 != 0,
            KeyModifier::Meta => self.0 & 0b0010_0000 != 0,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = KeyModifier> + '_ {
        [
            KeyModifier::Shift,
            KeyModifier::Control,
            KeyModifier::Alt,
            KeyModifier::Super,
            KeyModifier::Hyper,
            KeyModifier::Meta,
        ]
        .into_iter()
        .filter(|&m| self.contains(m))
    }
}

/// Represents a key.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
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

impl fmt::Display for KeyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyCode::Backspace => write!(f, "Backspace"),
            KeyCode::Enter => write!(f, "Enter"),
            KeyCode::Left => write!(f, "←"),
            KeyCode::Right => write!(f, "→"),
            KeyCode::Up => write!(f, "↑"),
            KeyCode::Down => write!(f, "↓"),
            KeyCode::Home => write!(f, "Home"),
            KeyCode::End => write!(f, "End"),
            KeyCode::PageUp => write!(f, "PageUp"),
            KeyCode::PageDown => write!(f, "PageDown"),
            KeyCode::Tab => write!(f, "Tab"),
            KeyCode::BackTab => write!(f, "BackTab"),
            KeyCode::Delete => write!(f, "Delete"),
            KeyCode::Insert => write!(f, "Insert"),
            KeyCode::F(n) => write!(f, "F{}", n),
            KeyCode::Char(c) => write!(f, "{}", c),
            KeyCode::Null => write!(f, "Null"),
            KeyCode::Esc => write!(f, "Esc"),
            KeyCode::Unknown => write!(f, "Unknown"),
        }
    }
}
