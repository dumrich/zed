// Generic Finder
use super::ZedError;
use crate::error::Error;
use crate::ui::Component;
use std::fs::{self, DirEntry};
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use zui_core::key::{Key, KeyIterator};
use zui_core::term::Terminal;
use zui_core::widgets::popup::Popup;
use zui_core::widgets::{Position, Widget};

fn finder<T: Write>(term: &mut Terminal<T>, r: Vec<PathBuf>) {
    // Generic Finder

    let p = Popup::new(term).title("").width(60).height(25);
    let p_deets = p.render(term).unwrap();

    term.set_cursor_to(p_deets.starting_pos.0 + 2, p_deets.starting_pos.1 + 1)
        .unwrap();
    println!("{:?}asd", r);
}

pub struct FileFinder {
    search: String,
    dir: PathBuf,
    show_icon: bool,
}

impl FileFinder {
    pub fn set_dir(mut self, dir: PathBuf) -> FileFinder {
        self.dir = dir.to_path_buf();
        self
    }

    fn search_dir(&self, p: &PathBuf) -> io::Result<Vec<PathBuf>> {
        let mut dirs_list = Vec::new();
        if p.is_dir() {
            for entry in fs::read_dir(p)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    dirs_list.append(&mut self.search_dir(p)?);
                } else {
                    if self.search != "" {
                        if let Some(x) = path.to_str() {
                            if x.contains(&self.search) {
                                dirs_list.push(path.to_owned());
                            }
                        }
                    } else if self.search == "" {
                        dirs_list.push(path.to_owned());
                    }
                }
            }
        }

        Ok(dirs_list)
    }
}

impl Component for FileFinder {
    type Widget = FileFinder;

    // Don't use this. Use create instead
    fn new() -> Self::Widget {
        FileFinder {
            // If you change the banner, make sure to edit cursor (view method)
            // No spaces on lines
            show_icon: true,
            dir: PathBuf::new(),
            search: String::new(),
        }
    }

    fn destroy<T: Write>(&mut self, term: &mut Terminal<T>) -> super::ZedError {
        match term.clear_screen() {
            Ok(_) => Ok(()),
            Err(_e) => Err(Error::CouldNotRender),
        }
    }

    // Fix all these unwraps
    fn view<T: Write>(&mut self, term: &mut Terminal<T>) -> super::ZedError {
        // Inital values
        let x = Popup::new(term)
            .title("Find")
            .width(60)
            .height(2)
            .y_offset(14);
        let x_deets = x.render(term).unwrap();
        finder(term, self.search_dir(&self.dir).unwrap());

        term.set_cursor_to(x_deets.starting_pos.0 + 2, x_deets.starting_pos.1 + 1)
            .unwrap();

        Ok(())
    }

    fn handle_key<T: Write>(
        &mut self,
        term: &mut Terminal<T>,
        keys: KeyIterator,
    ) -> super::ZedError {
        for key in keys.clone() {
            match key {
                Key::Esc => {
                    self.destroy(term).unwrap();
                    return Ok(());
                }
                Key::Char(x) => {
                    self.search.push(x);
                    term.print(x.to_string().as_str()).unwrap();
                    term.set_cursor_to(term.x_pos + 1, term.y_pos).unwrap();
                    term.show_cursor().unwrap();
                }
                Key::Backspace => {
                    if self.search.len() >= 1 {
                        self.search = self.search[..self.search.len() - 1].to_string();
                        term.set_cursor_to(term.x_pos - 1, term.y_pos).unwrap();
                        term.print(" ").unwrap();
                        term.set_cursor_to(term.x_pos, term.y_pos).unwrap();
                        term.show_cursor().unwrap();
                    }
                } //TODO: Add Arrow Keys
                _ => continue,
            }
        }
        Ok(())
    }
}
