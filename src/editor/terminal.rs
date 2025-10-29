use std::io::{stdout, Write};
use crossterm::style::Print;
use crossterm::{queue};
use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::terminal::{ClearType, disable_raw_mode, enable_raw_mode, Clear, size};


pub struct Size {
    pub height: u16,
    pub width: u16,
}

pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Terminal {
}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position{ x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }
    pub fn terminate() -> Result<(), std::io::Error> {
        Self::execute()?;
        disable_raw_mode()
    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(ClearType::All))
    }
    pub fn move_cursor_to(pos: Position) -> Result<(), std::io::Error> {
        queue!(stdout(), MoveTo(pos.x, pos.y))?;
        Ok(())
    }
    pub fn size() -> Result<Size, std::io::Error> {
        let (width, height) = size()?;
        Ok(Size{ width: width, height: height })
    }
    pub fn print(string: &str) -> Result<(), std::io::Error> {
        queue!(stdout(), Print(string))
    }
    pub fn show_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Show)
    }
    pub fn hide_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Hide)
    }
    pub fn clear_line() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))
    }
    pub fn execute() -> Result<(), std::io::Error> {
        stdout().flush()
    }
}
