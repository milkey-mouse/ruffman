use memmap::Mmap;

use std::{
    fs::File,
    io::{Read, Result},
    ops::Deref,
    path::Path,
};

pub enum FileBuf {
    Vec(Vec<u8>),
    Mmap(Mmap),
}

impl Deref for FileBuf {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Vec(v) => v,
            Self::Mmap(m) => m,
        }
    }
}

impl AsRef<[u8]> for FileBuf {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Vec(v) => v.as_ref(),
            Self::Mmap(m) => m.as_ref(),
        }
    }
}

impl FileBuf {
    pub fn new<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let mut file = File::open(filename)?;
        unsafe { Mmap::map(&file) }.map_or_else(
            |_| {
                let capacity = Self::initial_buffer_size(&file);
                let mut bytes = Vec::with_capacity(capacity);
                file.read_to_end(&mut bytes)?;
                Ok(Self::Vec(bytes))
            },
            |m| Ok(Self::Mmap(m)),
        )
    }

    pub fn read_to_end<R: Read>(mut reader: R) -> Result<Self> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;
        Ok(Self::Vec(bytes))
    }

    fn initial_buffer_size(file: &File) -> usize {
        // https://doc.rust-lang.org/src/std/fs.rs.html#266-274
        file.metadata().map(|m| m.len() as usize + 1).unwrap_or(0)
    }
}
