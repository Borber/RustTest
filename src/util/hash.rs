use anyhow::Result;
use std::fs::File;
use std::{cmp, io};
use std::io::prelude::*;
use std::path::Path;

const DERIVE_KEY: &str = "BORBER";

enum Input {
    Mmap(io::Cursor<memmap::Mmap>),
    File(File)
}

impl Input {
    fn open(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        if let Some(mmap) = maybe_memmap_file(&file)? {
            return Ok(Self::Mmap(io::Cursor::new(mmap)));
        }
        Ok(Self::File(file))
    }

    fn hash(&mut self) -> Result<blake3::Hash> {
        let mut hasher = blake3::Hasher::new_derive_key(DERIVE_KEY);
        match self {
            Self::Mmap(cursor) => {
                hasher.update_rayon(cursor.get_ref());
            }
            Self::File(file) => {
                copy_wide(file, &mut hasher)?;
            }
        }
        Ok(hasher.finalize())
    }
}

impl Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::Mmap(cursor) => cursor.read(buf),
            Self::File(file) => file.read(buf)
        }
    }
}


fn maybe_memmap_file(file: &File) -> Result<Option<memmap::Mmap>> {
    let metadata = file.metadata()?;
    let file_size = metadata.len();
    Ok(if !metadata.is_file() {
        None
    } else if file_size > isize::MAX as u64 {
        None
    } else if file_size == 0 {
        None
    } else if file_size < 16 * 1024 {
        None
    } else {
        let map = unsafe {
            memmap::MmapOptions::new()
                .len(file_size as usize)
                .map(&file)?
        };
        Some(map)
    })
}

fn copy_wide(mut reader: impl Read, hasher: &mut blake3::Hasher) -> io::Result<u64> {
    let mut buffer = [0; 65536];
    let mut total = 0;
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => return Ok(total),
            Ok(n) => {
                hasher.update(&buffer[..n]);
                total += n as u64;
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
}

pub fn hash_one(path: &Path) -> Result<String> {
    let mut block = [0; blake3::guts::BLOCK_LEN];
    let mut input = Input::open(path)?;
    let mut output = input.hash()?;
    let s = output.to_string();
    Ok(s)
}