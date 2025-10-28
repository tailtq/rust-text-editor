use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};


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
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        print!("Goodbye.\r\n");
    }
    // The function will return nothing if everything went well, and returns an error if something we couldn't recover from happened.
    fn repl(&mut self) -> Result<(), std::io::Error> {
        // unwraps the Result of enable_raw_mode.
        // If it's an error, it returns the error immediately. If not, it continues.
        enable_raw_mode()?;

        loop {
            if let Key(KeyEvent {
                code, modifiers, kind, state
            }) = read()? {
                println!("Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r");
                
                match code {
                    Char('x') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                    },
                    _ => (),
                }
            }
            if self.should_quit {
                break;
            }
        }

        disable_raw_mode()?;
        Ok(())
    }
}
