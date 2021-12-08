use super::buffer::Buffer;
use std::path::PathBuf;

// Overall editor mechanism

pub struct Editor<'a> {
    pub dir: Option<PathBuf>,
    pub anon: bool,
    pub num_buf: usize,
    pub buffers: Option<Vec<Buffer<'a>>>,
}

impl<'a> Editor<'a> {
    pub fn new() -> Editor<'a> {
        Editor {
            dir: None,
            anon: false,
            num_buf: 0,
            buffers: None,
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

    pub fn push_buf(mut self, buf: Buffer<'a>) -> Editor<'a> {
        match &mut self.buffers {
            Some(x) => {
                x.push(buf);
                self
            }
            None => {
                let mut buf_vec = Vec::new();
                buf_vec.push(buf);
                self.buffers = Some(buf_vec);
                self
            }
        }
    }
}
