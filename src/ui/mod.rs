use std::cell::RefCell;
use std::io::Write;
use std::path::PathBuf;

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

fn render_editor<T: Write>(file_path: PathBuf, term: &mut Terminal<T>, k: KeyIterator) -> ZedError {
    let b = Buffer::new().set_path(&file_path);
    let e = Editor::new();

    // Should probably rename this to something else
    let mut editor = editor::Editor::new().set_editor(e);
    editor.push_buf(&b);

    term.switch_screen().unwrap();
    editor.render(term, k).unwrap();
    editor.destroy(term)
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
                        render_editor(m, term, keys).unwrap();
                    }
                    _ => (),
                },
                Err(e) => {
                    eprintln!("Something went wrong at {}", e);
                }
            }
        }
        Target::File(x) => render_editor(x.to_path_buf(), term, keys).unwrap(),
        Target::Empty => (),
    }

    term.clear_screen().unwrap();
    term.switch_main().unwrap();
    Ok(())
}
