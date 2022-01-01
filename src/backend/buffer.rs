use ropey::Rope;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::{ffi::OsStr, path::PathBuf};

#[derive(Copy, Clone, Debug)]
pub enum Language {
    Python,
    Rust,
    // There is only an AT&T syntax.
    // This is a statement.
    Asm,
    C,
    Cpp,
    Lua,
    Php,
    Html,
    Css,
    Javascript,
    Java,
    Txt,
    Json,
    Csharp,
    Perl,
    Haskell,
    Cobol,
    Markdown,
    Go,
}

// Vi-Editing modes
#[derive(Copy, Debug, Clone)]
pub enum Mode {
    Insert,
    Visual,
    Normal,
}

// Individual Buffer Struct
pub struct Buffer<'a> {
    pub p: Option<&'a Path>,
    pub name: Option<&'a OsStr>,
    pub lang: Language,
    pub lang_str: &'static str,
    pub line_count: usize,
    pub rope: Rope,
    pub mode: Mode,
}

impl<'a> Buffer<'a> {
    pub fn new() -> Buffer<'a> {
        Buffer {
            p: None,
            name: None,
            lang: Language::Txt,
            lang_str: "",
            line_count: 0,
            rope: Rope::new(),
            mode: Mode::Normal,
        }
    }

    pub fn set_mode(mut self, m: Mode) -> Buffer<'a> {
        self.mode = m;
        self
    }

    pub fn set_path(mut self, p: &'a Path) -> Buffer<'a> {
        self.name = p.file_name();
        self.lang = derive_file_type(p);
        self.lang_str = derive_file_str(p);
        self.rope = Rope::from_reader(BufReader::new(File::open(&p).unwrap())).unwrap();
        self.line_count = self.rope.len_lines();
        self.p = Some(p);
        self
    }
}

fn derive_file_str(p: &Path) -> &'static str {
    let mut file_map: HashMap<&OsStr, &'static str> = HashMap::new();
    file_map.insert(OsStr::new("rs"), "\u{e7a8} Rust");
    file_map.insert(OsStr::new("md"), "\u{e73e} Markdown");
    file_map.insert(OsStr::new("py"), "\u{e73c} Python");
    file_map.insert(OsStr::new("asm"), "Assembly");
    file_map.insert(OsStr::new("c"), "\u{e61e} C");
    file_map.insert(OsStr::new("cpp"), "\u{e61d} C++");
    file_map.insert(OsStr::new("h"), "Header");
    file_map.insert(OsStr::new("html"), "\u{e736} HTML");
    file_map.insert(OsStr::new("css"), "\u{e749} CSS");
    file_map.insert(OsStr::new("go"), "\u{e626} Go");
    file_map.insert(OsStr::new("lua"), "\u{e620} Lua");
    file_map.insert(OsStr::new("php"), "\u{e73d} PHP");
    file_map.insert(OsStr::new("pl"), "\u{e769} Perl");
    file_map.insert(OsStr::new("js"), "\u{e74e} Javascript");
    file_map.insert(OsStr::new("java"), "\u{e738} Java");
    file_map.insert(OsStr::new("json"), "\u{fb25} Json");
    file_map.insert(OsStr::new("cs"), "\u{f81a} C#");

    let ext = p.extension();

    if let Some(x) = ext {
        match file_map.get(x) {
            Some(&p) => p,
            None => "TXT",
        }
    } else {
        "TXT"
    }
}

fn derive_file_type(p: &Path) -> Language {
    let mut file_map: HashMap<&OsStr, Language> = HashMap::new();
    file_map.insert(OsStr::new("rs"), Language::Rust);
    file_map.insert(OsStr::new("md"), Language::Markdown);
    file_map.insert(OsStr::new("py"), Language::Python);
    file_map.insert(OsStr::new("asm"), Language::Asm);
    file_map.insert(OsStr::new("c"), Language::C);
    file_map.insert(OsStr::new("cpp"), Language::Cpp);
    file_map.insert(OsStr::new("h"), Language::C);
    file_map.insert(OsStr::new("html"), Language::Html);
    file_map.insert(OsStr::new("css"), Language::Css);
    file_map.insert(OsStr::new("go"), Language::Go);
    file_map.insert(OsStr::new("lua"), Language::Lua);
    file_map.insert(OsStr::new("php"), Language::Php);
    file_map.insert(OsStr::new("pl"), Language::Perl);
    file_map.insert(OsStr::new("js"), Language::Javascript);
    file_map.insert(OsStr::new("java"), Language::Java);
    file_map.insert(OsStr::new("json"), Language::Json);
    file_map.insert(OsStr::new("cs"), Language::Csharp);

    let ext = p.extension();

    if let Some(x) = ext {
        match file_map.get(x) {
            Some(&p) => p,
            None => Language::Txt,
        }
    } else {
        Language::Txt
    }
}
