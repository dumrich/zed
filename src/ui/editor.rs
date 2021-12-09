use super::colors;
use super::Component;
use crate::backend::buffer;
use crate::backend::buffer::Buffer;
use crate::backend::editor;
use crate::error::Error;
use buffer::Mode;
use std::io;
use std::io::Write;
use zui_core::color::{self, Color};
use zui_core::key::{Key, KeyIterator};
use zui_core::term::Terminal;

fn draw_statusline<T: Write>(
    term: &mut Terminal<T>,
    buf: &Buffer,
    x_size: u16,
) -> Result<(), io::ErrorKind> {
    // This kinda sucks lol

    // Try not to print ANSI in loop
    // Instead, create string and print that

    // Vi-mode type
    let mut subtract_length = buf.lang_str.chars().count() as u16 + 1;
    term.hide_cursor().unwrap();

    match &buf.mode {
        Mode::Insert => {
            term.print(color::bg(Color::RGB(0, 153, 0))).unwrap();
            term.print(" INSERT ").unwrap();
            term.print(color::bg(Color::Reset)).unwrap();

            subtract_length += 7;
        }
        Mode::Normal => {
            term.print(color::bg(Color::RGB(0, 128, 0))).unwrap();
            term.print(" NORMAL ").unwrap();
            term.print(color::bg(Color::Reset)).unwrap();

            subtract_length += 7;
        }
        Mode::Visual => {
            term.print(color::bg(Color::Red)).unwrap();
            term.print(" VISUAL ").unwrap();
            term.print(color::bg(Color::Reset)).unwrap();

            subtract_length += 7;
        }
    }

    // File-name
    if let Some(file_path) = buf.p {
        let file_str = format!(" {:?}", file_path);

        let mut colored_string = String::new();

        colored_string.push_str(format!("{}", color::bg(Color::RGB(61, 61, 41))).as_str());
        term.print(colored_string).unwrap();
        term.print(file_str).unwrap();
        term.print(format!("{}", color::bg(Color::Reset)).as_str())
            .unwrap();

        // Rest of the statusline
        let mut print_string = String::new();

        print_string.push_str(format!("{}", color::bg(Color::RGB(61, 61, 41))).as_str());
        for _ in 0..x_size - subtract_length - file_path.as_os_str().len() as u16 - 4 {
            print_string.push(' ');
        }

        term.print(print_string).unwrap();

        term.print(format!("{}", color::bg(Color::RGB(61, 61, 41))).as_str())
            .unwrap();
        term.print(buf.lang_str).unwrap();
        term.print(" ").unwrap();
        term.print(format!("{}", color::bg(Color::Reset)).as_str())
            .unwrap();
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

                term.set_cursor_to(1, term.rel_size.1).unwrap();
                draw_statusline(term, curr_buf, x).unwrap();

                term.set_cursor_to(1, 1).unwrap();
                for line in 0..y - 2 {
                    term.print(curr_buf.rope.line(line.into())).unwrap();
                    term.set_cursor_to(term.x_pos, term.y_pos + 1).unwrap();
                }
                term.set_cursor_to(1, 1).unwrap();
            }
        }

        term.show_cursor().unwrap();

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
