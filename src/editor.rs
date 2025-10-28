use std::io::stdout;
use crossterm::execute;
use crossterm::terminal::{ClearType, disable_raw_mode, enable_raw_mode, Clear};
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};


pub struct Editor {
    should_quit: bool
}

impl Editor {
    pub fn default() -> Self {
        Editor {
            should_quit: false
        }
    }

    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let repl = self.repl();
        Self::terminate().unwrap();
        repl.unwrap();
    }
    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }
    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }
    // The function will return nothing if everything went well, and returns an error if something we couldn't recover from happened.
    fn repl(&mut self) -> Result<(), std::io::Error> {
        // unwraps the Result of enable_raw_mode.
        // If it's an error, it returns the error immediately. If not, it continues.
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }
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
            print!("Code: {code},  modifiers: {modifiers}\r\n");

            // implicit dereference for code and dereference for modifiers
            match code {
                Char('x') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                _ => (),
            }
        }
    }
    fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Self::clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }
}
