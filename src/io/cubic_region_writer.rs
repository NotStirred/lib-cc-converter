use std::path::Path;

use crate::{
    convert::{
        converter::{WriteError, Writer},
        data::cc_1_12::CubicChunks112Data,
        entry_location::{EntryLocation2d, EntryLocation3d},
    },
    util::positions::{RegionPos2d, RegionPos3d},
};

use super::region_writer::CachingRegionWriter;

pub struct CubicRegionWriter {
    inner_2d: CachingRegionWriter<RegionPos2d>,
    inner_3d: CachingRegionWriter<RegionPos3d>,
}

impl CubicRegionWriter {
    pub fn new(path: &Path, max_cache_size: usize) -> Result<Self, std::io::Error> {
        std::fs::create_dir_all(path)?;
        Ok(Self {
            inner_2d: CachingRegionWriter::new(
                &path.join("region2d"),
                EntryLocation2d::SECTOR_SIZE,
                EntryLocation2d::ENTRIES_PER_REGION,
                max_cache_size,
            )?,
            inner_3d: CachingRegionWriter::new(
                &path.join("region3d"),
                EntryLocation3d::SECTOR_SIZE,
                EntryLocation3d::ENTRIES_PER_REGION,
                max_cache_size,
            )?,
        })
    }
}

impl Writer<CubicChunks112Data> for CubicRegionWriter {
    fn write(&mut self, out_data: CubicChunks112Data) -> Result<(), WriteError> {
        if let Err(err) = self.inner_2d.write(out_data.position, &out_data.column_data) {
            return Err(WriteError::RegionWrite(err));
        }

        for (y, data) in &out_data.cube_data {
            let pos = EntryLocation3d::new(out_data.position.x, *y, out_data.position.z);
            if let Err(err) = self.inner_3d.write(pos, data) {
                return Err(WriteError::RegionWrite(err));
            }
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), WriteError> {
        if let Err(err) = self.inner_2d.flush() {
            return Err(WriteError::RegionWrite(err));
        };
        if let Err(err) = self.inner_3d.flush() {
            return Err(WriteError::RegionWrite(err));
        };
        Ok(())
    }
}
