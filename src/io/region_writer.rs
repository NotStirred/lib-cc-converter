use crate::convert::entry_location::Key;
use crate::io::write_region::{RegionWriteError, WriteRegion};
use crate::EntryLocation3d;
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::path::{Path, PathBuf};

pub struct CachingRegionWriter<R: Eq + Hash> {
    sector_size: usize,
    max_cache_size: usize,

    path: PathBuf,
    region_cache: HashMap<R, WriteRegion>,
}

impl<R: Eq + Hash> CachingRegionWriter<R> {
    pub fn new(path: &Path, sector_size: usize, max_cache_size: usize) -> Result<Self, std::io::Error> {
        match fs::create_dir(path) {
            Err(err) => match err.kind() {
                std::io::ErrorKind::AlreadyExists => {}
                _ => return Err(err),
            },
            _ => {}
        };
        Ok(Self {
            sector_size,
            max_cache_size,
            path: path.to_path_buf(),
            region_cache: HashMap::new(),
        })
    }

    pub fn write<K: Key<R> + Copy>(&mut self, entry_location: K, data: &[u8]) -> Result<(), RegionWriteError> {
        if self.region_cache.len() > self.max_cache_size {
            self.flush()?
        }

        let region = self.region_cache.entry(entry_location.to_region_pos()).or_insert_with(|| {
            WriteRegion::new(
                &self.path.join(entry_location.region_key()),
                self.sector_size,
                EntryLocation3d::ENTRIES_PER_REGION,
            )
        });

        region.write(entry_location, data)?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), RegionWriteError> {
        for (_, mut region) in self.region_cache.drain() {
            region.flush()?;
        }
        Ok(())
    }
}
