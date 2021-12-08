use super::Component;
use super::ZedError;
use crate::backend::buffer;
use crate::backend::editor;
use crate::error::Error;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use zui_core::color::{self, Color};
use zui_core::key::{Key, KeyIterator};
use zui_core::term::Terminal;

fn draw_statusline<T: Write>(term: &mut Terminal<T>) -> Result<(), io::ErrorKind> {
    let (x, y) = term.get_size(); // TODO: Fix this
    term.set_cursor_to(x_pos, y_pos)
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
                term.set_cursor_to(1, 2).unwrap();
                for line in 0..y - 1 {
                    term.print(b[e.num_buf].rope.line(line.into())).unwrap();
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
