use std::env::args;
use zed::cli::Cli;
use zed::error;

fn main() {
    let args: Vec<String> = args().collect();
    let mut app = Cli::from_args();

    match app.parse_args(&args[..]) {
        Ok(_) => app.run(),
        Err(c) => handle_error(c),
    }
}

fn handle_error(error: error::Error) {
    eprintln!("Failed at: {}", error);
    // exit
    ::std::process::exit(1);
}
