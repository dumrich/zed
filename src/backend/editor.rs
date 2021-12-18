use super::buffer::Buffer;
use std::path::PathBuf;

// Overall editor mechanism

pub struct Editor<'a> {
    pub dir: Option<PathBuf>,
    pub anon: bool,
    pub num_buf: usize,
    pub cur_buf: Option<&'a Buffer<'a>>,
    pub buffers: Vec<&'a Buffer<'a>>,
}

impl<'a> Editor<'a> {
    pub fn new() -> Editor<'a> {
        Editor {
            dir: None,
            anon: false,
            num_buf: 0,
            cur_buf: None,
            buffers: Vec::new(),
        }
    }

    pub fn set_dir(mut self, dir: PathBuf) -> Editor<'a> {
        self.dir = Some(dir);
        self
    }

    pub fn set_anon(mut self) -> Editor<'a> {
        self.anon = true;
        self
    }
}
