use std::path::Path;

use crate::{
    convert::{
        converter::{WriteError, Writer},
        data::anvil::Data,
        entry_location::EntryLocation2d,
    },
    io::region_writer::CachingRegionWriter,
    util::positions::RegionPos2d,
};

pub struct AnvilRegionWriter {
    inner: CachingRegionWriter<RegionPos2d>,
}

impl AnvilRegionWriter {
    pub fn new(path: &Path, max_cache_size: usize) -> Result<Self, std::io::Error> {
        Ok(Self {
            inner: CachingRegionWriter::new(
                path,
                EntryLocation2d::SECTOR_SIZE,
                EntryLocation2d::ENTRIES_PER_REGION,
                max_cache_size,
            )?,
        })
    }
}

impl Writer<Data> for AnvilRegionWriter {
    fn write(&mut self, out_data: Data) -> Result<(), WriteError> {
        let pos = EntryLocation2d::new(out_data.position.x, out_data.position.z);
        if let Err(err) = self.inner.write(pos, &out_data.data) {
            return Err(WriteError::Custom(Box::new(err)));
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
