use super::colors;
use super::Component;
use super::ZedError;
use crate::backend::buffer;
use crate::backend::buffer::Buffer;
use crate::backend::editor;
use crate::error::Error;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use zui_core::color::{self, Color};
use zui_core::key::{Key, KeyIterator};
use zui_core::term::Terminal;

fn draw_statusline<T: Write>(term: &mut Terminal<T>, buf: &Buffer) -> Result<(), io::ErrorKind> {
    let (x, y) = term.get_size(); // TODO: Fix this
    term.set_cursor_to(1, y - 1).unwrap();

    // Try not to print ANSI in loop
    // Instead, create string and print that

    // Vi-mode type
    let mut subtract_length = 0;
    term.hide_cursor().unwrap();
    match &buf.mode {
        Insert => {
            term.print(color::bg(Color::GreenLight)).unwrap();
            term.print(" INSERT |").unwrap();
            term.print(color::bg(Color::Reset)).unwrap();

            subtract_length = 9;
        }
        Normal => {
            term.print(color::bg(Color::RGB(1, 200, 2))).unwrap();
            term.print(" NORMAL |").unwrap();
            term.print(color::bg(Color::Reset)).unwrap();

            subtract_length = 9;
        }
        Visual => {
            term.print(color::bg(Color::Red)).unwrap();
            term.print(" VISUAL |").unwrap();
            term.print(color::bg(Color::Reset)).unwrap();

            subtract_length = 9;
        }
    }

    // File-name
    if let Some(file_path) = buf.p {
        let file_str = format!(" {:?}", file_path);
        term.print(file_str).unwrap();

        // Rest of the statusline
        let mut print_string = String::new();
        for index in 0..x - subtract_length - file_path.as_os_str().len() as u16 {
            print_string.push(' ');
        }
        term.print(print_string).unwrap();
    }

    term.show_cursor().unwrap();

    Ok(())
}

pub struct Editor<'a> {
    pub editor: Option<editor::Editor<'a>>,
}

impl<'a> Editor<'a> {
    pub fn set_editor(mut self, e: editor::Editor<'a>) -> Editor<'a> {
        self.editor = Some(e);
        self
    }
}

impl<'a> Component for Editor<'a> {
    type Widget = Editor<'a>;

    // WidgetReturn
    type WidgetReturn = ();

    fn new() -> Self::Widget {
        Editor { editor: None }
    }

    fn destroy<T: Write>(&mut self, term: &mut Terminal<T>) -> super::ZedError {
        Ok(())
    }

    fn view<T: Write>(&mut self, term: &mut Terminal<T>) -> super::ZedError {
        // Inital values
        let (x, y) = term.get_size(); // TODO: Fix this
        term.clear_screen().unwrap();

        // Render Lines
        if let Some(e) = &mut self.editor {
            if let Some(b) = &mut e.buffers {
                // Currently selected buffer
                let curr_buf = &b[e.num_buf];
                draw_statusline(term, curr_buf).unwrap();
                term.set_cursor_to(1, 2).unwrap();
                for line in 0..y - 2 {
                    term.print(curr_buf.rope.line(line.into())).unwrap();
                    term.set_cursor_to(term.x_pos, term.y_pos + 1).unwrap();
                }
                term.set_cursor_to(1, 1).unwrap();
            }
        }

        Ok(())
    }

    fn handle_key<T: Write>(
        &mut self,
        term: &mut Terminal<T>,
        keys: KeyIterator,
    ) -> Result<Self::WidgetReturn, Error> {
        for key in keys.clone() {
            match key {
                Key::Ctrl('q') => return Ok(()),
                _ => continue,
            }
        }
        Ok(())
    }

    fn render<T: Write>(
        &mut self,
        term: &mut Terminal<T>,
        keys: KeyIterator,
    ) -> Result<Self::WidgetReturn, Error> {
        self.view(term).unwrap();

        match self.handle_key(term, keys.clone()) {
            Ok(s) => Ok(()),
            Err(s) => Err(s),
        }
    }
}
