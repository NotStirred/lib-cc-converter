use crate::convert::entry_location::Key;
use crate::util::math_util;
use byteorder::{BigEndian, WriteBytesExt};
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::fs::File;
use std::io::{BufWriter, ErrorKind, Write};

use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum RegionWriteError {
    MissingHeader(PathBuf),
    StdIo(std::io::Error),
}

impl std::error::Error for RegionWriteError {}

impl From<std::io::Error> for RegionWriteError {
    fn from(err: std::io::Error) -> Self {
        RegionWriteError::StdIo(err)
    }
}

impl Display for RegionWriteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RegionWriteError::StdIo(err) => f.write_str(&format!("Error when reading region: {}", err)),
            RegionWriteError::MissingHeader(path) => f.write_str(&format!("Missing header in region {:?}", path)),
        }
    }
}

pub struct WriteRegion {
    sector_size: usize,
    entries_per_region: usize,
    path: PathBuf,
    write_entries: Option<Vec<Option<Vec<u8>>>>,
}

impl WriteRegion {
    pub const SIZE_BITS: u32 = 8;
    pub const SIZE_MASK: u32 = (1 << Self::SIZE_BITS) - 1;

    pub fn new(path: &Path, sector_size: usize, entries_per_region: usize) -> Self {
        Self {
            sector_size,
            entries_per_region,
            path: path.to_path_buf(),
            write_entries: None,
        }
    }

    pub fn write<K, R>(&mut self, key: K, value: &[u8]) -> Result<(), RegionWriteError>
    where
        K: Key<R>,
    {
        if self.write_entries.is_none() {
            self.initialize()?;
        }

        let size = value.len();
        let size_with_info = size + 4;
        let num_sectors = self.get_sector_number(size_with_info);

        let mut data = Vec::with_capacity(num_sectors * self.sector_size);
        data.write_i32::<BigEndian>(size as i32)?;
        data.resize(num_sectors * self.sector_size, 0);
        data[4..4 + value.len()].copy_from_slice(value);

        let id = key.id();
        self.write_entries
            .as_mut()
            .expect("Initialize is complete, this should be unreachable")[id] = Some(data);
        Ok(())
    }

    fn initialize(&mut self) -> Result<(), RegionWriteError> {
        let bytes = match fs::read(&self.path) {
            Ok(b) => b,
            Err(err) => match err.kind() {
                ErrorKind::NotFound => {
                    self.write_entries = Some(vec![None; self.entries_per_region]);
                    return Ok(());
                }
                _ => return Err(RegionWriteError::StdIo(err)),
            },
        };
        if bytes.len() < self.sector_size {
            self.write_entries = Some(vec![None; self.entries_per_region]);
            return Ok(());
        }

        let mut write_entries = vec![None; self.entries_per_region];
        for (cube_idx, entry) in write_entries.iter_mut().enumerate().take(self.entries_per_region) {
            let sector_offset = cube_idx * 4;

            let (size, offset) = Self::unpack_size_offset(&bytes, sector_offset);

            if offset == 0 || size == 0 {
                continue;
            }

            let byte_offset = offset * self.sector_size as u32;

            let slice_start = (byte_offset + 4) as usize;
            let slice_end = (byte_offset + (size * self.sector_size as u32)) as usize;

            entry.replace(bytes[slice_start..slice_end].to_vec());
        }

        self.write_entries = Some(write_entries);
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        let mut header = Vec::new();

        let mut write_pos = math_util::ceil_div_usize(self.entries_per_region * 4, self.sector_size);
        if let Some(entries) = &mut self.write_entries {
            for write_entry in entries.iter() {
                match write_entry {
                    None => {
                        header.write_i32::<BigEndian>(0)?;
                        continue;
                    }
                    Some(write_entry) => {
                        let sector_count = math_util::ceil_div_usize(write_entry.len(), self.sector_size);
                        header.write_i32::<BigEndian>(Self::packed(write_pos, sector_count) as i32)?;
                        write_pos += sector_count;
                    }
                }
            }

            let file = File::create(&self.path).expect("Unable to create file");
            let mut file = BufWriter::new(file);
            file.write_all(&header)?;
            for write_entry in entries.iter_mut().flatten() {
                file.write_all(write_entry)?;
            }
            entries.fill(None);

            return Ok(());
        }
        Ok(())
    }

    fn get_sector_number(&self, bytes: usize) -> usize {
        math_util::ceil_div_usize(bytes, self.sector_size)
    }

    fn packed(offset: usize, size: usize) -> usize {
        size | (offset << Self::SIZE_BITS)
    }

    fn unpack_size_offset(bytes: &[u8], sector_offset: usize) -> (u32, u32) {
        let packed = (bytes[sector_offset + 3] as u32)
            | (bytes[sector_offset + 2] as u32) << 8
            | (bytes[sector_offset + 1] as u32) << 16
            | (bytes[sector_offset] as u32) << 24;

        let size = packed & Self::SIZE_MASK;
        let offset = packed >> Self::SIZE_BITS;
        (size, offset)
    }
}
