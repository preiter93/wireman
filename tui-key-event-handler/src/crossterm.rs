use crossterm::event::KeyCode as CTKeyCode;
use crossterm::event::KeyEvent as CTKeyEvent;
use crossterm::event::KeyEventKind as CTKeyEventKind;
use crossterm::event::KeyEventState as CTKeyEventState;
use crossterm::event::KeyModifiers as CTKeyModifiers;

use crate::KeyCode;
use crate::KeyEvent;
use crate::KeyModifier;
use crate::KeyModifiers;

impl From<CTKeyEvent> for KeyEvent {
    fn from(value: CTKeyEvent) -> Self {
        Self {
            code: value.code.into(),
            modifiers: value.modifiers.into(),
        }
    }
}
impl From<KeyEvent> for CTKeyEvent {
    fn from(value: KeyEvent) -> Self {
        Self {
            code: value.code.into(),
            modifiers: value.modifiers.into(),
            kind: CTKeyEventKind::Press,
            state: CTKeyEventState::empty(),
        }
    }
}

impl From<CTKeyModifiers> for KeyModifiers {
    fn from(value: CTKeyModifiers) -> Self {
        let ct_modifiers = vec![
            // (CTKeyModifiers::SHIFT, KeyModifier::Shift),
            (CTKeyModifiers::CONTROL, KeyModifier::Control),
            (CTKeyModifiers::ALT, KeyModifier::Alt),
            (CTKeyModifiers::SUPER, KeyModifier::Super),
            (CTKeyModifiers::HYPER, KeyModifier::Hyper),
            (CTKeyModifiers::META, KeyModifier::Meta),
        ];
        let mut modifiers = Self::new();
        for modifier in ct_modifiers {
            if value.contains(modifier.0) {
                modifiers.add_modifier(modifier.1);
            }
        }
        modifiers
    }
}

impl From<KeyModifiers> for CTKeyModifiers {
    fn from(value: KeyModifiers) -> Self {
        let ct_modifiers = vec![
            // (CTKeyModifiers::SHIFT, KeyModifier::Shift),
            (CTKeyModifiers::CONTROL, KeyModifier::Control),
            (CTKeyModifiers::ALT, KeyModifier::Alt),
            (CTKeyModifiers::SUPER, KeyModifier::Super),
            (CTKeyModifiers::HYPER, KeyModifier::Hyper),
            (CTKeyModifiers::META, KeyModifier::Meta),
        ];
        let mut modifiers = Self::NONE;
        for modifier in ct_modifiers {
            if value.contains(modifier.1) {
                modifiers.insert(modifier.0);
            }
        }
        modifiers
    }
}

impl From<CTKeyCode> for KeyCode {
    fn from(value: CTKeyCode) -> Self {
        match value {
            CTKeyCode::Backspace => Self::Backspace,
            CTKeyCode::Enter => Self::Enter,
            CTKeyCode::Left => Self::Left,
            CTKeyCode::Right => Self::Right,
            CTKeyCode::Up => Self::Up,
            CTKeyCode::Down => Self::Down,
            CTKeyCode::Home => Self::Home,
            CTKeyCode::End => Self::End,
            CTKeyCode::PageUp => Self::PageUp,
            CTKeyCode::PageDown => Self::PageDown,
            CTKeyCode::Tab => Self::Tab,
            CTKeyCode::BackTab => Self::BackTab,
            CTKeyCode::Delete => Self::Delete,
            CTKeyCode::Insert => Self::Insert,
            CTKeyCode::F(f) => Self::F(f),
            CTKeyCode::Char(c) => Self::Char(c),
            CTKeyCode::Null => Self::Null,
            CTKeyCode::Esc => Self::Esc,
            _ => Self::Unknown,
        }
    }
}

impl From<KeyCode> for CTKeyCode {
    fn from(value: KeyCode) -> Self {
        match value {
            KeyCode::Backspace => Self::Backspace,
            KeyCode::Enter => Self::Enter,
            KeyCode::Left => Self::Left,
            KeyCode::Right => Self::Right,
            KeyCode::Up => Self::Up,
            KeyCode::Down => Self::Down,
            KeyCode::Home => Self::Home,
            KeyCode::End => Self::End,
            KeyCode::PageUp => Self::PageUp,
            KeyCode::PageDown => Self::PageDown,
            KeyCode::Tab => Self::Tab,
            KeyCode::BackTab => Self::BackTab,
            KeyCode::Delete => Self::Delete,
            KeyCode::Insert => Self::Insert,
            KeyCode::F(f) => Self::F(f),
            KeyCode::Char(c) => Self::Char(c),
            KeyCode::Null => Self::Null,
            KeyCode::Esc => Self::Esc,
            KeyCode::Unknown => Self::Null,
        }
    }
}
