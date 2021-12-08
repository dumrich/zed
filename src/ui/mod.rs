use std::io::Write;
use std::thread;
use std::time;
use std::time::Duration;

use crate::backend::buffer::Buffer;
use crate::backend::editor::Editor;
// Some traits that components should implement
use crate::{
    cli::{Cli, Target},
    error::Error,
};
use zui_core::key::KeyIterator;
use zui_core::term::Terminal;

// Create
mod colors;
mod dashboard;
mod editor;
mod finder;

type ZedError = Result<(), Error>;

pub trait Component {
    // Widget Type
    type Widget;

    // HandleKey Types
    type WidgetReturn;

    // Create new Component
    fn new() -> Self::Widget;

    // Destroy element
    fn destroy<T: Write>(&mut self, term: &mut Terminal<T>) -> ZedError;

    // Draw the user interface here
    fn view<T: Write>(&mut self, term: &mut Terminal<T>) -> ZedError;

    // Component Keybindings
    fn handle_key<T: Write>(
        &mut self,
        term: &mut Terminal<T>,
        keys: KeyIterator,
    ) -> Result<Self::WidgetReturn, Error>;

    fn render<T: Write>(
        &mut self,
        term: &mut Terminal<T>,
        keys: KeyIterator,
    ) -> Result<Self::WidgetReturn, Error>;
}

pub fn render_ui<T: Write>(cli: &Cli, term: &mut Terminal<T>, keys: KeyIterator) -> ZedError {
    // Manage the User Interface
    match &cli.target {
        Target::Dir(x) => {
            term.switch_screen().unwrap();
            let mut dashboard = dashboard::Dashboard::new().set_dir(x.to_path_buf());

            match dashboard.render(term, keys.clone()) {
                Ok(t) => match t {
                    Target::File(m) => {
                        let mut b = Buffer::new().set_path(&m);
                        let mut e = Editor::new().push_buf(b);

                        // Should probably rename this to something else
                        let mut editor = editor::Editor::new().set_editor(e);
                        editor.render(term, keys.clone()).unwrap();
                    }
                    _ => (),
                },
                Err(e) => {
                    eprintln!("Something went wrong at {}", e);
                }
            }

            term.switch_main().unwrap();
        }
        Target::File(_x) => (),
        Target::Empty => (),
    }
    Ok(())
}
