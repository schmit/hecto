use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use super::terminal::Size;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

pub enum Direction {
    PageUp,
    PageDown,
    Home,
    End,
    Up,
    Down,
    Left,
    Right,
}

pub enum EditorCommand {
    Move(Direction),
    Resize(Size),
    Quit,
}

#[derive(Debug)]
pub enum CommandError {
    UnsupportedEvent,
    UnsupportedKey(KeyCode),
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::UnsupportedEvent => write!(f, "unsupported event"),
            CommandError::UnsupportedKey(code) => write!(f, "unsupported key: {code:?}"),
        }
    }
}

impl std::error::Error for CommandError {}

impl TryFrom<Event> for EditorCommand {
    type Error = CommandError;
    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                (KeyCode::Up, _) => Ok(Self::Move(Direction::Up)),
                (KeyCode::Down, _) => Ok(Self::Move(Direction::Down)),
                (KeyCode::Left, _) => Ok(Self::Move(Direction::Left)),
                (KeyCode::Right, _) => Ok(Self::Move(Direction::Right)),
                (KeyCode::Home, _) => Ok(Self::Move(Direction::Home)),
                (KeyCode::End, _) => Ok(Self::Move(Direction::End)),
                (KeyCode::PageUp, _) => Ok(Self::Move(Direction::PageUp)),
                (KeyCode::PageDown, _) => Ok(Self::Move(Direction::PageDown)),
                _ => Err(CommandError::UnsupportedKey(code)),
            },
            Event::Resize(width_u16, height_u16) => {
                let height = usize::from(height_u16);
                let width = usize::from(width_u16);
                Ok(Self::Resize(Size { width, height }))
            }
            _ => Err(CommandError::UnsupportedEvent),
        }
    }
}
