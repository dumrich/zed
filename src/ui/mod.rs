use std::io::Write;

// Some traits that interfaces should implement
use crate::{
    cli::{Cli, Target},
    error::Error,
};
use zui_core::key::KeyIterator;
use zui_core::term::Terminal;

// Create
mod dashboard;
//mod statusline;
mod finder;

type ZedError = Result<(), Error>;

pub trait Component {
    type Widget;

    // Create new Component
    fn new() -> Self::Widget;

    // Destroy element
    fn destroy<T: Write>(&mut self, term: &mut Terminal<T>) -> ZedError;

    // Draw the user interface here
    fn view<T: Write>(&mut self, term: &mut Terminal<T>) -> ZedError;

    // Component Keybindings
    fn handle_key<T: Write>(&mut self, term: &mut Terminal<T>, keys: KeyIterator) -> ZedError;
}

pub fn render<T: Write>(
    term: &mut Terminal<T>,
    c: &mut impl Component,
    keys: KeyIterator,
) -> ZedError {
    c.view(term).unwrap();

    Ok(c.handle_key(term, keys)?)
}

pub fn render_ui<T: Write>(cli: &Cli, term: &mut Terminal<T>, keys: KeyIterator) -> ZedError {
    // Manage the User Interface
    match &cli.target {
        Target::Dir(x) => {
            term.switch_screen().unwrap();
            let mut dashboard = dashboard::Dashboard::new().set_dir(x.to_path_buf());
            render(term, &mut dashboard, keys.clone()).unwrap();

            term.switch_main().unwrap();
        }
        Target::File(_x) => (),
        Target::Empty => (),
    }
    Ok(())
}
