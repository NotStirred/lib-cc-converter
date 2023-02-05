use crate::convert::entry_location::Key;
use crate::io::write_region::{RegionWriteError, WriteRegion};
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::path::{Path, PathBuf};

pub struct CachingRegionWriter<REGION> {
    sector_size: usize,
    entries_per_region: usize,
    max_cache_size: usize,

    path: PathBuf,
    region_cache: HashMap<REGION, WriteRegion>,
}

impl<REGION> CachingRegionWriter<REGION> {
    pub fn new(path: &Path, sector_size: usize, entries_per_region: usize, max_cache_size: usize) -> Result<Self, std::io::Error> {
        if let Err(err) = fs::create_dir(path) {
            match err.kind() {
                std::io::ErrorKind::AlreadyExists => {}
                _ => return Err(err),
            }
        };
        Ok(Self {
            sector_size,
            entries_per_region,
            max_cache_size,
            path: path.to_path_buf(),
            region_cache: HashMap::new(),
        })
    }

    pub fn write<KEY>(&mut self, entry_location: KEY, data: &[u8]) -> Result<(), RegionWriteError>
    where
        REGION: Eq + Hash,
        KEY: Key<REGION> + Copy,
    {
        if self.region_cache.len() > self.max_cache_size {
            self.flush()?;
        }

        let region = self.region_cache.entry(entry_location.to_region_pos()).or_insert_with(|| {
            WriteRegion::new(
                &self.path.join(entry_location.region_key()),
                self.sector_size,
                self.entries_per_region,
            )
        });

        region.write(&entry_location, data)?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), RegionWriteError> {
        for (_, mut region) in self.region_cache.drain() {
            region.flush()?;
        }
        Ok(())
    }
}
