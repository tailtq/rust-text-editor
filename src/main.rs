mod editor;

use editor::Editor;
// Short-hand for two lines below -> import io module from the "standard library" (std)
// use std::io;
// use std::io::Read;

fn main() {
    // Read byte from the standard input into the variable b.
    // io::stdin().bytes() is an iterator
    let mut editor = Editor::default();
    editor.run();
}
