use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};

use crate::convert::converter::{ReadError, Reader};
use crate::convert::data::anvil_chunk_data::AnvilChunkData;
use crate::convert::entry_location::MinecraftChunkLocation;
use crate::util::positions::RegionPos2d;

pub struct AnvilRegionData {
    pub data: Vec<u8>,
    pub chunk_indices: Vec<Option<(usize, usize)>>,
}

pub struct AnvilRegionReader {
    region_location: PathBuf,
}

pub enum AnvilReadError {
    StdIo(std::io::Error),
    MissingHeader(RegionPos2d),
}

impl From<std::io::Error> for AnvilReadError {
    fn from(err: std::io::Error) -> Self {
        AnvilReadError::StdIo(err)
    }
}

impl Debug for AnvilReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AnvilReadError::StdIo(err) => f.write_str(&format!("Error when reading anvil region: {}", err)),
            AnvilReadError::MissingHeader(region_pos) => f.write_str(&format!("Missing header in region {}", region_pos)),
        }
    }
}

impl Display for AnvilReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AnvilReadError::StdIo(err) => f.write_str(&format!("Error when reading anvil region: {}", err)),
            AnvilReadError::MissingHeader(region_pos) => f.write_str(&format!("Missing header in region {}", region_pos)),
        }
    }
}

impl AnvilRegionReader {
    pub const SECTOR_SIZE: usize = 4096;
    pub const SIZE_BITS: u32 = 8;
    pub const SIZE_MASK: u32 = (1 << Self::SIZE_BITS) - 1;

    pub fn new(region_location: &Path) -> Self {
        Self {
            region_location: region_location.to_path_buf(),
        }
    }

    pub fn read_region(&self, region_pos: RegionPos2d) -> Result<AnvilRegionData, AnvilReadError> {
        let region_key = region_pos.region_key();
        let region_path = self.region_location.join(region_key);

        let bytes = fs::read(region_path)?;
        if bytes.len() < Self::SECTOR_SIZE {
            return Err(AnvilReadError::MissingHeader(region_pos));
        }

        let mut data_indices = Vec::with_capacity(RegionPos2d::CHUNKS_COUNT);
        for chunk_idx in 0..RegionPos2d::CHUNKS_COUNT {
            let sector_offset = chunk_idx * 4;

            let packed = (bytes[sector_offset + 3] as u32)
                | (bytes[sector_offset + 2] as u32) << 8
                | (bytes[sector_offset + 1] as u32) << 16
                | (bytes[sector_offset] as u32) << 24;

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

        Ok(AnvilRegionData {
            data: bytes,
            chunk_indices: data_indices,
        })
    }
}

impl Reader<MinecraftChunkLocation, AnvilChunkData> for AnvilRegionReader {
    fn load_all_chunks<F>(&mut self, data_consumer: F) -> Result<(), ReadError>
    where
        F: Fn(AnvilChunkData) -> (),
    {
        let paths = std::fs::read_dir(&self.region_location)?;
        for path in paths {
            if let Ok(dir_entry) = path {
                let path = dir_entry.path();
                if path.extension().and_then(std::ffi::OsStr::to_str).unwrap_or("") == "mca" {
                    let file_name = match path.file_name().and_then(std::ffi::OsStr::to_str) {
                        Some(name) => name,
                        None => continue,
                    };
                    if let Some(region_pos) = RegionPos2d::from(file_name) {
                        match self.read_region(region_pos) {
                            Ok(region_data) => {
                                let data = &region_data.data;
                                let indices = &region_data.chunk_indices;

                                for x in 0..32 {
                                    for z in 0..32 {
                                        let i = x + z * RegionPos2d::DIAMETER_IN_CHUNKS;
                                        if let Some((start, end)) = indices[i] {
                                            data_consumer(AnvilChunkData {
                                                data: (&data[start..end]).to_vec(),
                                                position: region_pos.to_minecraft_chunk_location_offset(x as i32, z as i32),
                                            });
                                        }
                                    }
                                }
                            }
                            Err(err) => println!("Error reading region {}, skipping it.\n{}", file_name, err),
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
