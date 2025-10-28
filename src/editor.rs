mod terminal;
use terminal::Terminal;
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};


pub struct Editor {
    should_quit: bool
}

impl Editor {
    pub const fn default() -> Self {
        Self {
            should_quit: false
        }
    }

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
            if self.should_quit {
                let _ = Self::draw_rows();
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) {
        // This syntax is shorter compared to match with only one option
        // (using match requires catching the other cases _ => ())
        // event is an enum and the if block checks if it's Key
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event {
            // print!("Code: {code},  modifiers: {modifiers}\r\n");
            // implicit dereference for code and dereference for modifiers
            match code {
                Char('x') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                _ => (),
            }
        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye.\r\n");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(0, 0)?;
        }
        Ok(())
    }
    fn draw_rows() -> Result<(), std::io::Error> {
        let height = Terminal::size()?.1;

        for current_row in 0..height {
            Terminal::move_cursor_to(0, current_row)?;
            print!("~");

            if current_row + 1 < height {
                print!("\r\n");
            }
        }

        Ok(())
    }
}
