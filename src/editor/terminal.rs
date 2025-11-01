use std::io::{stdout, Write};
use crossterm::style::Print;
use crossterm::{queue};
use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::terminal::{ClearType, disable_raw_mode, enable_raw_mode, Clear, size};


#[derive(Debug, Default)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

pub struct Terminal {
}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_caret_to(Position::default())?;
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
    pub fn move_caret_to(pos: Position) -> Result<(), std::io::Error> {
        queue!(stdout(), MoveTo(pos.col as u16, pos.row as u16))?;
        Ok(())
    }
    pub fn size() -> Result<Size, std::io::Error> {
        let (width, height) = size()?;
        Ok(Size{ width: width as usize, height: height as usize })
    }
    pub fn print(string: &str) -> Result<(), std::io::Error> {
        queue!(stdout(), Print(string))
    }
    pub fn show_caret() -> Result<(), std::io::Error> {
        queue!(stdout(), Show)
    }
    pub fn hide_caret() -> Result<(), std::io::Error> {
        queue!(stdout(), Hide)
    }
    pub fn clear_line() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))
    }
    pub fn execute() -> Result<(), std::io::Error> {
        stdout().flush()
    }
}
