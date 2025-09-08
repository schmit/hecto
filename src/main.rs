#![warn(clippy::all, clippy::pedantic)]
mod editor;
use editor::Editor;

fn main() {
    if let Err(err) = Editor::new().and_then(|mut e| e.run()) {
        eprintln!("hecto error: {err}");
    }
}
