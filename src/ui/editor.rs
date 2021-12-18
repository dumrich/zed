use super::Component;
use crate::backend::buffer;
use crate::backend::buffer::Buffer;
use crate::backend::editor;
use crate::error::Error;
use buffer::Mode;
use ropey::RopeSlice;
use std::io;
use std::io::Write;
use zui_core::color::{self, fg, Color};
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
    pub current_line: usize,
    pub current_index: usize,
}

impl<'a> Editor<'a> {
    pub fn set_editor(mut self, e: editor::Editor<'a>) -> Editor<'a> {
        self.editor = Some(e);
        self
    }

    pub fn push_buf(&mut self, buf: &'a Buffer<'a>) {
        if let Some(e) = &mut self.editor {
            e.buffers.push(buf);
            e.cur_buf = Some(e.buffers[e.buffers.len() - 1]);
        }
    }

    // Movement methods
    pub fn move_up<T: Write>(&mut self, term: &mut Terminal<T>) -> Result<(), Error> {
        if self.current_line > 1 {
            self.current_line -= 1;
            term.set_cursor_to(term.x_pos, term.y_pos - 1).unwrap();
            return Ok(());
        }
        Err(Error::CouldNotMove)
    }

    pub fn move_down<T: Write>(
        &mut self,
        term: &mut Terminal<T>,
        buf: &Buffer<'a>,
    ) -> Result<(), Error> {
        if buf.line_count - 1 > self.current_line {
            self.current_line += 1;
            term.set_cursor_to(term.x_pos, term.y_pos + 1).unwrap();
            return Ok(());
        }
        Err(Error::CouldNotMove)
    }
}

fn draw_line<T: Write>(
    term: &mut Terminal<T>,
    line: &RopeSlice,
    x_size: u16,
) -> Result<(), io::ErrorKind> {
    term.print(line).unwrap();
    term.set_cursor_to(term.x_pos, term.y_pos + 1).unwrap();
    Ok(())
}

impl<'a> Component for Editor<'a> {
    type Widget = Editor<'a>;

    // WidgetReturn
    type WidgetReturn = ();

    fn new() -> Self::Widget {
        Editor {
            editor: None,
            current_line: 1,
            current_index: 1,
        }
    }

    fn destroy<T: Write>(&mut self, _term: &mut Terminal<T>) -> super::ZedError {
        Ok(())
    }

    fn view<T: Write>(&mut self, term: &mut Terminal<T>) -> super::ZedError {
        // Inital values
        let (x, y) = term.get_size(); // TODO: Fix this
        term.clear_screen().unwrap();

        // Render Lines
        if let Some(e) = &mut self.editor {
            if let Some(cur_buf) = e.cur_buf {
                // Currently selected buffer
                term.set_cursor_to(1, term.rel_size.1).unwrap();
                draw_statusline(term, cur_buf, x).unwrap();

                term.set_cursor_to(1, 1).unwrap();
                let buff_lc = cur_buf.line_count;

                let mut buf_offset = 0;
                match buff_lc {
                    0..=9 => {
                        buf_offset = 1;
                    }
                    10..=99 => {
                        buf_offset = 2;
                    }
                    100..=999 => {
                        buf_offset = 3;
                    }
                    1000..=9999 => {
                        buf_offset = 4;
                    }
                    _ => {
                        buf_offset = 5;
                    }
                }

                for line in 0..y - 1 {
                    if buff_lc > line.into() {
                        // Line numbers + offset
                        term.print(fg(Color::RGB(153, 153, 102))).unwrap();

                        let mut o_str = String::new();
                        let mut o_count = buf_offset;
                        if (0..9).contains(&line) {
                            o_count -= 1;
                        } else if (9..99).contains(&line) {
                            o_count -= 2;
                        } else if (99..999).contains(&line) {
                            o_count -= 3;
                        } else if (999..9999).contains(&line) {
                            o_count -= 4;
                        }

                        for _ in 0..o_count {
                            o_str.push(' ');
                        }
                        term.print(o_str).unwrap();
                        term.print(line + 1).unwrap();
                        term.print(fg(Color::Reset)).unwrap();
                        term.print(" ").unwrap();

                        // Render text
                        let curr_line = cur_buf.rope.line(line.into());
                        draw_line(term, &curr_line, x).unwrap();
                    } else {
                        term.print("~").unwrap();
                        term.set_cursor_to(term.x_pos, term.y_pos + 1).unwrap();
                    }
                }
                term.set_cursor_to(buf_offset + 2, 1).unwrap();
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
        for key in keys {
            match key {
                Key::Ctrl('q') => return Ok(()),
                Key::Up | Key::Char('j') => {
                    if let Err(e) = self.move_up(term) {
                        continue;
                    }
                }
                Key::Down | Key::Char('k') => {
                    if let Some(e) = &self.editor {
                        if let Some(b) = e.cur_buf {
                            if let Err(i) = self.move_down(term, b) {
                                continue;
                            }
                        }
                    }
                }
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

        match self.handle_key(term, keys) {
            Ok(s) => Ok(()),
            Err(s) => Err(s),
        }
    }
}
