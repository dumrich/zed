// Some traits that interfaces should implement
use crate::{cli::Cli, error::Error};
use zui_core::key::KeyIterator;
use zui_widgets::{backend::ZuiBackend, Terminal};

// Create
mod dashboard;
//mod statusline;
//mod finder;

type ZedError = Result<(), Error>;

pub trait Component {
    type Widget;

    // Create new Component
    fn new() -> Self::Widget;

    // Destroy element
    fn destroy(&mut self, term: &mut Terminal<ZuiBackend>) -> ZedError;

    // Draw the user interface here
    fn view(&mut self, term: &mut Terminal<ZuiBackend>) -> ZedError;

    // (Private) How to handle a refresh when the state is changed
    fn update(&mut self) -> ZedError;

    // Component Keybindings
    fn handle_key(&mut self, term: &mut Terminal<ZuiBackend>, keys: &mut KeyIterator) -> ZedError;
}

pub fn render(
    term: &mut Terminal<ZuiBackend<'_>>,
    c: &mut impl Component,
    keys: &mut KeyIterator,
) -> ZedError {
    c.view(term).unwrap();

    Ok(c.handle_key(term, keys)?)
}

pub fn render_ui(
    cli: &Cli,
    term: &mut Terminal<ZuiBackend<'_>>,
    keys: &mut KeyIterator,
) -> ZedError {
    // Manage the User Interface
    let mut dashboard = dashboard::Dashboard::new();
    render(term, &mut dashboard, keys).unwrap();
    Ok(())
}
