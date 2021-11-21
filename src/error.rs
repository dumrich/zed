use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not resolve config file"))]
    ConfigNotFound,
}
