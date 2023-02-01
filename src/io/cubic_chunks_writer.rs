use std::path::Path;

use crate::{
    convert::{
        converter::{WriteError, Writer},
        data::cubic_chunks_1_12::CubicChunks112Data,
        entry_location::EntryLocation3d,
    },
    util::positions::RegionPos3d,
};

use super::region_writer::CachingRegionWriter;

pub struct CubicChunksWriter {
    inner: CachingRegionWriter<RegionPos3d>,
}

impl CubicChunksWriter {
    pub fn new(path: &Path, max_cache_size: usize) -> Result<Self, std::io::Error> {
        Ok(Self {
            inner: CachingRegionWriter::new(
                path,
                EntryLocation3d::SECTOR_SIZE,
                EntryLocation3d::ENTRIES_PER_REGION,
                max_cache_size,
            )?,
        })
    }
}

impl Writer<CubicChunks112Data> for CubicChunksWriter {
    fn write(&mut self, out_data: CubicChunks112Data) -> Result<(), WriteError> {
        for (y, data) in &out_data.cube_data {
            let pos = EntryLocation3d::new(out_data.position.x, *y, out_data.position.z);
            self.inner.write(pos, &*data).unwrap();
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), WriteError> {
        match self.inner.flush() {
            Ok(val) => Ok(val),
            Err(err) => Err(WriteError::Custom(Box::new(err))),
        }
    }
}
