use std::{
    fmt::{Display, Formatter},
    fs,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use crate::{
    convert::{
        converter::{ReadError, Reader},
        entry_location::RegionPos,
    },
    util::errors::error_from,
};

pub struct RegionData {
    pub data: Vec<u8>,
    pub chunk_indices: Vec<Option<(usize, usize)>>,
}

pub struct RegionReader<POS, EXTRACT> {
    region_location: PathBuf,
    extract_chunks_function: EXTRACT,

    marker_pos: PhantomData<POS>,
}

pub enum RegionReadError {
    StdIo(std::io::Error),
    MissingHeader,
}

error_from!(RegionReadError, std::io::Error, Self::StdIo);

impl Display for RegionReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RegionReadError::StdIo(err) => f.write_str(&format!("Error when reading region: {err}")),
            RegionReadError::MissingHeader => f.write_str("Missing header in region"),
        }
    }
}

impl<POS, F, DATA> RegionReader<POS, F>
where
    F: Fn(POS, RegionData) -> Vec<DATA>,
{
    pub const SECTOR_SIZE: usize = 4096;
    pub const SIZE_BITS: u32 = 8;
    pub const SIZE_MASK: u32 = (1 << Self::SIZE_BITS) - 1;

    pub fn new(region_location: &Path, f: F) -> Self {
        Self {
            region_location: region_location.to_path_buf(),
            extract_chunks_function: f,
            marker_pos: PhantomData::default(),
        }
    }

    pub fn read_region(&self, position: &POS) -> Result<RegionData, RegionReadError>
    where
        POS: RegionPos,
    {
        let region_key = position.region_key();
        let region_path = self.region_location.join(region_key);

        let bytes = fs::read(region_path)?;
        if bytes.len() < Self::SECTOR_SIZE {
            return Err(RegionReadError::MissingHeader);
        }

        let mut data_indices = Vec::with_capacity(POS::entries_per_region());
        for chunk_idx in 0..POS::entries_per_region() {
            let sector_offset = chunk_idx * 4;

            let packed = u32::from(bytes[sector_offset + 3])
                | u32::from(bytes[sector_offset + 2]) << 8
                | u32::from(bytes[sector_offset + 1]) << 16
                | u32::from(bytes[sector_offset]) << 24;

            let offset = (packed >> Self::SIZE_BITS) as usize;
            let size = (packed & Self::SIZE_MASK) as usize;

            if offset == 0 || size == 0 {
                data_indices.push(None);
                continue;
            }

            let byte_offset = offset * Self::SECTOR_SIZE;

            let slice_start = byte_offset + 4;
            let slice_end = byte_offset + (size * Self::SECTOR_SIZE);

            // if header entry is invalid, set the entry to None
            if slice_start > bytes.len() || slice_end > bytes.len() {
                data_indices.push(None);
            }

            data_indices.push(Some((slice_start, slice_end)));
        }

        Ok(RegionData {
            data: bytes,
            chunk_indices: data_indices,
        })
    }
}

impl<POS, DATA, EXTRACT> Reader<POS, DATA> for RegionReader<POS, EXTRACT>
where
    POS: RegionPos + Copy + Send,
    EXTRACT: Fn(POS, RegionData) -> Vec<DATA> + Send,
{
    fn load_all_chunks<F>(&mut self, data_consumer: F) -> Result<(), ReadError>
    where
        F: Fn(DATA),
    {
        let paths = std::fs::read_dir(&self.region_location)?;
        paths.for_each(|path| {
            if let Ok(dir_entry) = path {
                let path = dir_entry.path();
                if path.extension().and_then(std::ffi::OsStr::to_str).unwrap_or("") == "mca" {
                    let file_name = match path.file_name().and_then(std::ffi::OsStr::to_str) {
                        Some(name) => name,
                        None => return,
                    };
                    if let Some(region_pos) = POS::from_file_name(file_name) {
                        match self.read_region(&region_pos) {
                            Ok(region_data) => {
                                let chunks_data = (self.extract_chunks_function)(region_pos, region_data);

                                for data in chunks_data {
                                    data_consumer(data);
                                }
                            }
                            Err(err) => println!("Error reading region {file_name}, skipping it.\n{err}"),
                        }
                    }
                }
            }
        });
        Ok(())
    }
}
