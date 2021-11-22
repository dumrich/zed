use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not resolve config file"))]
    ConfigNotFound,
    #[snafu(display("Could not render component. Are you on Windows?"))]
    CouldNotRender,
}
