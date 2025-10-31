mod terminal;
mod view;
mod buffer;
use std::cmp::min;

use terminal::{Terminal, Size};
use view::View;
use crossterm::event::{Event::{self, Key}, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read};

use crate::editor::terminal::Position;


#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    caret_position: Position,
    view: View,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let repl = self.repl();
        Terminal::terminate().unwrap();
        repl.unwrap();
    }
    // The function will return nothing if everything went well, and returns an error if something we couldn't recover from happened.
    fn repl(&mut self) -> Result<(), std::io::Error> {
        // unwraps the Result of enable_raw_mode.
        // If it's an error, it returns the error immediately. If not, it continues.
        loop {
            self.refresh_screen()?;
            View::draw_welcome_message()?;
            if self.should_quit {
                let _ = self.view.render();
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) -> Result<(), std::io::Error> {
        // This syntax is shorter compared to match with only one option
        // (using match requires catching the other cases _ => ())
        // event is an enum and the if block checks if it's Key
        if let Key(KeyEvent {
            code, modifiers, kind: KeyEventKind::Press, ..
        }) = event {
            // print!("Code: {code},  modifiers: {modifiers}\r\n");
            // implicit dereference for code and dereference for modifiers
            match code {
                KeyCode::Char('x') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                KeyCode::Up |
                KeyCode::Down |
                KeyCode::Left |
                KeyCode::Right |
                KeyCode::PageUp |
                KeyCode::Home |
                KeyCode::PageDown |
                KeyCode::End => {
                    self.move_point(code)?;
                },
                _ => (),
            }
            
        }
        Ok(())
    }
    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;

        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            self.view.render()?;
            Terminal::move_caret_to(Position{
                x: self.caret_position.x,
                y: self.caret_position.y,
            })?;
        }

        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }
    
    fn move_point(&mut self, key_code: &KeyCode) -> Result<(), std::io::Error> {
        let Position { mut x, mut y } = self.caret_position;
        let Size { width, height } = Terminal::size()?;

        match *key_code {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => y = min(height.saturating_sub(1), y.saturating_add(1)),
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => x = min(width.saturating_sub(1), x.saturating_add(1)),
            KeyCode::Home => x = 0,
            KeyCode::PageUp => y = 0,
            KeyCode::End => x = width.saturating_sub(1),
            KeyCode::PageDown => y = height.saturating_sub(1),
            _ => (),
        }
        self.caret_position.x = x;
        self.caret_position.y = y;
        Ok(())
    }
}
