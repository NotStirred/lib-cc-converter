use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use crate::util::positions::RegionPos2d;

pub struct AnvilRegionData {
    pub data: Vec<u8>,
    pub chunk_indices: Vec<Option<(usize, usize)>>,
}

pub struct AnvilRegionReader {
    region_location: PathBuf,
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

    pub fn read(&self, region_pos: RegionPos2d) -> Result<AnvilRegionData, Box<dyn Error>> {
        let region_key = region_pos.region_key();
        let region_path = self.region_location.join(region_key);

        let bytes = fs::read(region_path)?;
        let mut data_indices = Vec::with_capacity(RegionPos2d::CHUNKS_COUNT);
        for chunk_idx in 0..RegionPos2d::CHUNKS_COUNT {
            let sector_offset = chunk_idx * 4;

            let packed = (bytes[sector_offset + 3] as u32)
                | (bytes[sector_offset + 2] as u32) << 8
                | (bytes[sector_offset + 1] as u32) << 16
                | (bytes[sector_offset + 0] as u32) << 24;

            let offset = (packed >> Self::SIZE_BITS) as usize;
            let size = (packed & Self::SIZE_MASK) as usize;

            if offset == 0 || size == 0 {
                data_indices.push(None);
                continue;
            }

            let byte_offset = offset * Self::SECTOR_SIZE;

            let slice_start = byte_offset + 4;
            let slice_end = byte_offset + (size * Self::SECTOR_SIZE);

            data_indices.push(Some((slice_start, slice_end)));
        }

        Ok(AnvilRegionData {
            data: bytes,
            chunk_indices: data_indices,
        })
    }
}
