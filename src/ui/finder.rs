const MAX_CHECKS: usize = 1024;

// Generic Finder
use super::ZedError;
use crate::error::Error;
use crate::ui::Component;
use std::fmt::{self, Display, Error as FmtError};
use std::fs::{self, DirEntry};
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zui_core::key::{Key, KeyIterator};
use zui_core::term::Terminal;
use zui_core::widgets::popup::Popup;
use zui_core::widgets::{Position, Widget};

fn finder<T: Write, W: Display>(term: &mut Terminal<T>, r: &[W]) {
    // Generic Finder

    let curr_pos = term.get_cursor().unwrap();
    term.hide_cursor().unwrap();
    let p = Popup::new(term).title("").width(60).height(25);
    let p_deets = p.render(term).unwrap();

    term.set_cursor_to(p_deets.starting_pos.0 + 2, p_deets.ending_pos.1 - 1)
        .unwrap();

    let mut max_val = (p.height - 1) as usize;

    if max_val > r.len() {
        max_val = r.len();
    }

    for l in &r[..max_val] {
        term.print(l).unwrap();
        term.set_cursor_to(term.x_pos, term.y_pos - 1).unwrap();
    }

    term.set_cursor_to(curr_pos.0, curr_pos.1).unwrap();
    term.show_cursor().unwrap()
}

pub struct FileFinder {
    search: String,
    dir: PathBuf,
    show_icon: bool,
}

pub struct FileResult {
    path: PathBuf,
    icon: &'static str,
}

impl FileResult {
    pub fn to_string(&self) -> Option<&str> {
        self.path.to_str()
    }
}

impl Display for FileResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.path.to_str() {
            Some(x) => write!(f, "{}  {}", self.icon, x),
            None => Err(FmtError),
        }
    }
}

impl FileFinder {
    pub fn set_dir(mut self, dir: PathBuf) -> FileFinder {
        self.dir = dir.to_path_buf();
        self
    }

    fn search_dir(&self, p: &PathBuf) -> io::Result<Vec<FileResult>> {
        let mut dirs_list = Vec::new();
        let mut checked_files: usize = 0;
        if p.is_dir() {
            for entry in WalkDir::new(p)
                .min_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if checked_files < MAX_CHECKS {
                    let path = entry.path().to_owned();
                    checked_files += 1;

                    if dirs_list.len() <= 30 {
                        if path.is_dir() {
                            continue;
                        }
                        if let Some(x) = path.to_str() {
                            if x.to_owned().contains(&self.search) {
                                dirs_list.push(FileResult {
                                    path: path.to_path_buf(),
                                    icon: "\u{e7a8}",
                                });
                            } else {
                                continue;
                            }
                        }
                    } else {
                        return Ok(dirs_list);
                    }
                } else {
                    return Ok(dirs_list);
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

        term.set_cursor_to(x_deets.starting_pos.0 + 2, x_deets.starting_pos.1 + 1)
            .unwrap();

        finder(term, &self.search_dir(&self.dir).unwrap()[..]);
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
                    term.set_cursor_to(term.x_pos - 2, term.y_pos - 2).unwrap();
                    term.clear_above_cursor().unwrap();
                    term.set_cursor_to(term.x_pos + 2, term.y_pos + 2).unwrap();
                    finder(term, &self.search_dir(&self.dir).unwrap()[..]);
                    term.print(x.to_string().as_str()).unwrap();
                    term.set_cursor_to(term.x_pos + 1, term.y_pos).unwrap();
                    term.show_cursor().unwrap();

                    continue;
                }
                Key::Backspace => {
                    if self.search.len() >= 1 {
                        self.search = self.search[..self.search.len() - 1].to_string();
                        term.set_cursor_to(term.x_pos - 2, term.y_pos - 2).unwrap();
                        term.clear_above_cursor().unwrap();
                        term.set_cursor_to(term.x_pos + 2, term.y_pos + 2).unwrap();
                        finder(term, &self.search_dir(&self.dir).unwrap()[..]);
                        term.set_cursor_to(term.x_pos - 1, term.y_pos).unwrap();
                        term.print(" ").unwrap();
                        term.set_cursor_to(term.x_pos, term.y_pos).unwrap();
                        term.show_cursor().unwrap();
                    }
                    continue;
                } //TODO: Add Arrow Keys
                _ => continue,
            }
        }
        Ok(())
    }
}
