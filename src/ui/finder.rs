// Generic Finder
use super::ZedError;
use std::io::Write;
use zui_core::term::Terminal;
use zui_core::widgets::popup::Popup;
use zui_core::widgets::{Position, Widget};

pub fn finder<T: Write>(term: &mut Terminal<T>, x: impl Iterator) {
    // Generic Finder
    let x = Popup::new(term)
        .title("Find")
        .width(60)
        .height(2)
        .y_offset(14);
    x.render(term).unwrap();

    let p = Popup::new(term).title("").width(60).height(25);
    p.render(term).unwrap();
}
