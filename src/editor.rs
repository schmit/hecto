use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read};
use std::panic::{set_hook, take_hook};

mod position;
mod terminal;
use terminal::{Terminal, Size};

mod view;
use view::View;

pub struct Editor {
    should_quit: bool,
    view: View,
}

impl Editor {
    pub fn new() -> Result<Self, std::io::Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));

        Terminal::initialize()?;
        let mut view = View::default();

        if let Some(file_name) = Self::get_filename() {
            view.load(&file_name);
        }
        Ok(Self {
            should_quit: false,
            view,
        })
    }

    pub fn run(&mut self) {
        loop {
            self.view.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}")
                    }
                }
            }
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn evaluate_event(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                }
                (
                    KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Home
                    | KeyCode::End
                    | KeyCode::PageUp
                    | KeyCode::PageDown,
                    _,
                ) => {
                    self.view.move_cursor(code);
                }
                _ => (),
            },
            Event::Resize(width_u16, height_u16) => {
                let height = height_u16 as usize;
                let width = width_u16 as usize;
                self.view.resize(Size { width, height });
            }
            _ => {}
        }
    }

    fn get_filename() -> Option<String> {
        let args: Vec<String> = std::env::args().collect();
        if let Some(filename) = args.get(1) {
            return Some(filename.to_string());
        }
        None
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye!\r\n");
        }
    }
}
