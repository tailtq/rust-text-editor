use super::terminal::Terminal;
use super::buffer::Buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");


pub struct View {
    buffer: Buffer
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer {
                content: vec![
                    ("Hello, World!").to_string(),
                    ("I'm Tai from Workday!").to_string()
                ]
            },
        }
    }
}

impl View {
    pub fn render(&self) -> Result<(), std::io::Error> {
        let height = Terminal::size()?.height;
        Terminal::clear_line()?;

        for current_row in 0..height {
            Terminal::clear_line()?;

            if let Some(line) = self.buffer.content.get(current_row as usize) {
                Terminal::print(&line)?;
                Terminal::print("\r\n")?;
                continue;
            }
            if current_row == (height as f32 / 2.5) as u16 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }
    pub fn draw_welcome_message() -> Result<(), std::io::Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width as usize;
        let len = welcome_message.len();
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding - 1);
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(&welcome_message)?;
        Ok(())
    }
    pub fn draw_empty_row() -> Result<(), std::io::Error> {
        Terminal::print("~")
    }
}
