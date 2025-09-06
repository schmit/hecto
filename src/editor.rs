use crossterm::event::{Event, KeyEvent, KeyEventKind, read};
use std::panic::{set_hook, take_hook};

mod editorcommand;
mod position;
mod terminal;
mod view;
use terminal::{Size, Terminal};

use editorcommand::EditorCommand;
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
        let size: Size = Terminal::size().unwrap_or_default();
        let mut view = View::new(size);

        if let Some(file_name) = Self::get_filename() {
            view.load(&file_name);
        }
        Ok(Self {
            should_quit: false,
            view,
        })
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        eprintln!("Could not read event: {err:?}");
                    }
                }
            }
        }
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        self.view.render()?;
        Terminal::move_cursor_to(self.view.get_cursor_position())?;
        Terminal::show_cursor()?;
        Terminal::flush()
    }

    #[allow(clippy::needless_pass_by_value)]
    fn evaluate_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(..) => true,
            _ => false,
        };

        if !should_process {
            return;
        }

        match EditorCommand::try_from(event) {
            Ok(command) => {
                if matches!(command, EditorCommand::Quit) {
                    self.should_quit = true;
                } else {
                    self.view.handle_command(command);
                }
            }
            Err(err) => {
                #[cfg(debug_assertions)]
                eprintln!("Ignoring input: {err}");
            }
        }
    }

    fn get_filename() -> Option<String> {
        let mut args = std::env::args();
        let _program = args.next();
        args.next()
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
