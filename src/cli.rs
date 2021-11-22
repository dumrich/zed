// Resolve file/directory name
use crate::error;
use crate::ui;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use zui_widgets::backend::ZuiBackend;
use zui_widgets::Terminal;

#[derive(Debug)]
pub enum Target {
    File(PathBuf),
    Dir(PathBuf),
    Empty,
}

#[derive(Debug)]
pub struct Cli {
    // Is target file or dir or none
    target: Target,

    // Should backup file on save
    backup: bool,

    // Should save on exit
    save_on_exit: bool,

    // Location of (custom) config file
    pub config: Option<PathBuf>,
}

impl Cli {
    pub fn from_args() -> Cli {
        let config = PathBuf::from("~/.config/zed/config.ron");

        Cli {
            target: Target::Empty,
            backup: false,
            save_on_exit: false,
            config: Some(config),
        }
    }

    pub fn parse_args(&mut self, args: &[String]) -> Result<(), error::Error> {
        // TODO: Allow args like `-bc` or `-cb`
        for (i, arg) in args.iter().enumerate().skip(1) {
            if arg == &String::from("-b") || arg == &String::from("--backup") {
                self.backup = true;
            } else if arg == &String::from("-c") || arg == &String::from("--config") {
                let next_arg = args.get(i + 1);
                match &next_arg {
                    Some(x) => {
                        let custom_config = PathBuf::from(*x);
                        if custom_config.is_file() {
                            self.config = Some(custom_config);
                        } else {
                            return Err(error::Error::ConfigNotFound);
                        }
                    }
                    None => return Err(error::Error::ConfigNotFound),
                }
            } else if arg == &String::from("-h") || arg == &String::from("--help") {
                println!("Usage:\n\tzed [options] [file(s)]\nOptions:\n\t-b\t\tStore backup of file\n\t-c\t\tSpecify custom config\n\t-h, --help\tShow this message");
            } else {
                if arg.starts_with("-") {
                    eprintln!("Invalid Option\nTry zed --help for more information");
                    ::std::process::exit(1);
                } else {
                    // Treats as file argument
                    self.target = resolve_path(arg);
                }
            }
        }
        Ok(())
    }

    pub fn run(&self) -> () {
        // Entry point to editor
        let mut stdout = stdout();
        let backend = ZuiBackend::new(&mut stdout);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.enter_raw_mode().unwrap();
        let mut keys = terminal.keys(stdin());

        ui::render_ui(self, &mut terminal, &mut keys).unwrap()
    }
}

fn resolve_path(path: &str) -> Target {
    let target_path = PathBuf::from(path);

    if target_path.is_file() {
        Target::File(target_path)
    } else if target_path.is_dir() {
        Target::Dir(target_path)
    } else if !target_path.is_file() {
        // Create File
        print!("File does not exist, do you want to create one: ");
        let mut answer = String::new();
        stdout().flush().unwrap();
        stdin().read_line(&mut answer).unwrap();
        let _f;
        if answer.trim().to_lowercase() == "yes".to_string() {
            _f = File::create(path);
        }
        Target::File(PathBuf::from(path))
    } else {
        Target::Dir(PathBuf::from("."))
    }
}
