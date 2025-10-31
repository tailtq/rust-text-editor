#[derive(Default, Clone)]
pub struct Buffer {
    pub content: Vec<String>
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.content.len() == 0
    }
}
