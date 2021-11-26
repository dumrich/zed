const MAX_CHECKS: usize = 1024;

// Generic Finder
use crate::error::Error;
use crate::ui::Component;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fmt::{self, Display, Error as FmtError};
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zui_core::key::{Key, KeyIterator};
use zui_core::term::Terminal;
use zui_core::widgets::popup::Popup;
use zui_core::widgets::Widget;

// Generic fuzzy finder
// TODO: Make more generic
fn finder<T: Write, W: Display>(term: &mut Terminal<T>, r: &[W]) {
    // Cur possition before doing shit
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
    results: Vec<FileResult>,
    currently_selected_index: Option<usize>,
    search: String,
    dir: PathBuf,
}

pub struct FileResult {
    path: PathBuf,
    icon: &'static str,
}

impl FileResult {
    pub fn new() -> FileResult {
        FileResult {
            path: PathBuf::new(),
            icon: "",
        }
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
                let path = entry.path().to_owned();
                if path.to_string_lossy().contains(".git/")
                    || path.to_string_lossy().contains("target/debug/")
                    || path.to_string_lossy().contains("target/release/")
                {
                    continue;
                }
                checked_files += 1;

                if dirs_list.len() <= 30 {
                    if path.is_dir() {
                        continue;
                    }
                    if let Some(x) = path.to_str() {
                        if x.to_owned().contains(&self.search) {
                            dirs_list.push(FileResult {
                                path: path.to_path_buf(),
                                icon: derive_file_type(&path),
                            });
                        } else {
                            continue;
                        }
                    }
                } else {
                    return Ok(dirs_list);
                }
            }
        }

        Ok(dirs_list)
    }
}

fn derive_file_type(p: &Path) -> &'static str {
    let mut file_map: HashMap<&OsStr, &str> = HashMap::new();
    file_map.insert(OsStr::new("rs"), "\u{e7a8}");
    file_map.insert(OsStr::new("md"), "\u{e73e}");
    file_map.insert(OsStr::new("py"), "\u{e73c}");
    file_map.insert(OsStr::new("asm"), "\u{e796}");
    file_map.insert(OsStr::new("c"), "\u{e61e}");
    file_map.insert(OsStr::new("cpp"), "\u{e61d}");
    file_map.insert(OsStr::new("h"), "\u{e61e}");
    file_map.insert(OsStr::new("html"), "\u{e736}");
    file_map.insert(OsStr::new("css"), "\u{e749}");
    file_map.insert(OsStr::new("go"), "\u{e626}");
    file_map.insert(OsStr::new("lua"), "\u{e620}");
    file_map.insert(OsStr::new("php"), "\u{e73d}");
    file_map.insert(OsStr::new("pl"), "\u{e769}");
    file_map.insert(OsStr::new("js"), "\u{e718}");
    file_map.insert(OsStr::new("java"), "\u{e718}");
    file_map.insert(OsStr::new("json"), "\u{fb25}");
    file_map.insert(OsStr::new("cs"), "\u{f81a}");

    let ext = p.extension();

    if let Some(x) = ext {
        match file_map.get(x) {
            Some(p) => p,
            None => "\u{f15c}",
        }
    } else {
        "\u{f15c}"
    }
}

impl Component for FileFinder {
    type Widget = FileFinder;

    fn new() -> Self::Widget {
        FileFinder {
            results: vec![FileResult::new()],
            dir: PathBuf::new(),
            search: String::new(),
            currently_selected_index: None,
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

        self.results = self.search_dir(&self.dir).unwrap();
        finder(term, &self.results[..]);
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
                Key::Up => match self.currently_selected_index {
                    Some(p) => {
                        if p >= self.results.len() - 1 {
                            continue;
                        } else {
                            self.currently_selected_index = Some(p + 1);
                            term.set_cursor_to(term.x_pos, term.y_pos - 1).unwrap();
                        }
                        continue;
                    }
                    None => {
                        self.currently_selected_index = Some(0);
                        let go_back = self.search.chars().count();
                        term.set_cursor_to(term.x_pos - go_back as u16 + 3, term.y_pos - 3)
                            .unwrap();
                        continue;
                    }
                },
                Key::Down => match self.currently_selected_index {
                    Some(p) => {
                        if p == 0 {
                            let go_back = self.search.chars().count();
                            term.set_cursor_to(term.x_pos + go_back as u16 - 3, term.y_pos + 3)
                                .unwrap();
                            self.currently_selected_index = None;
                        } else {
                            self.currently_selected_index = Some(p - 1);
                            term.set_cursor_to(term.x_pos, term.y_pos + 1).unwrap();
                        }
                        continue;
                    }
                    None => {
                        continue;
                    }
                },
                Key::Char(x) => {
                    self.search.push(x);

                    term.set_cursor_to(term.x_pos - 2, term.y_pos - 2).unwrap();
                    term.clear_above_cursor().unwrap();
                    term.set_cursor_to(term.x_pos + 2, term.y_pos + 2).unwrap();

                    self.results = self.search_dir(&self.dir).unwrap();
                    finder(term, &self.results[..]);

                    term.print(x.to_string().as_str()).unwrap();
                    term.set_cursor_to(term.x_pos + 1, term.y_pos).unwrap();

                    continue;
                }
                Key::Backspace => {
                    if self.search.len() >= 1 {
                        self.search = self.search[..self.search.len() - 1].to_string();

                        term.set_cursor_to(term.x_pos - 2, term.y_pos - 2).unwrap();
                        term.clear_above_cursor().unwrap();
                        term.set_cursor_to(term.x_pos + 2, term.y_pos + 2).unwrap();

                        self.results = self.search_dir(&self.dir).unwrap();
                        finder(term, &self.results[..]);

                        term.set_cursor_to(term.x_pos - 1, term.y_pos).unwrap();
                        term.print(" ").unwrap();
                        term.set_cursor_to(term.x_pos, term.y_pos).unwrap();
                    }
                    continue;
                } //TODO: Add Arrow Keys
                _ => continue,
            }
        }
        Ok(())
    }
}
