use std::marker::PhantomData;

use crate::util::positions::{MinecraftRegionPos, RegionPos2d, RegionPos3d};
use crate::util::vec::vec2i::Vec2i;
use crate::util::vec::vec3i::Vec3i;
use crate::util::vec::CoordinateSpace;

pub type RegionKey = String;

pub trait RegionPos {
    fn from_file_name(file_name: &str) -> Option<Self>
    where
        Self: Sized;

    fn diameter_in_chunks() -> usize;
    fn entries_per_region() -> usize;

    fn region_key(&self) -> RegionKey;
}

pub trait Key<R> {
    fn to_region_pos(self) -> R;
    fn region_key(&self) -> RegionKey;
    fn id(&self) -> usize;

    fn entries_per_region() -> usize;
}

pub struct EntryLocation3dSpace {}
impl CoordinateSpace for EntryLocation3dSpace {}

pub type EntryLocation3d = Vec3i<EntryLocation3dSpace>;
impl EntryLocation3d {
    const LOC_BITS: usize = 4;
    const LOC_BITMASK: usize = (1 << Self::LOC_BITS) - 1;
    pub const ENTRIES_PER_REGION: usize = (1 << Self::LOC_BITS) * (1 << Self::LOC_BITS) * (1 << Self::LOC_BITS);
    pub const SECTOR_SIZE: usize = 512;
}

impl Key<RegionPos3d> for EntryLocation3d {
    fn to_region_pos(self) -> RegionPos3d {
        RegionPos3d::new(self.x >> Self::LOC_BITS, self.y >> Self::LOC_BITS, self.z >> Self::LOC_BITS)
    }

    fn region_key(&self) -> RegionKey {
        let reg_x = self.x >> Self::LOC_BITS;
        let reg_y = self.y >> Self::LOC_BITS;
        let reg_z = self.z >> Self::LOC_BITS;

        format!("{reg_x}.{reg_y}.{reg_z}.3dr")
    }

    fn id(&self) -> usize {
        ((self.x as usize & Self::LOC_BITMASK) << (Self::LOC_BITS * 2))
            | ((self.y as usize & Self::LOC_BITMASK) << Self::LOC_BITS)
            | (self.z as usize & Self::LOC_BITMASK)
    }

    fn entries_per_region() -> usize {
        Self::ENTRIES_PER_REGION
    }
}

pub struct EntryLocation2dSpace {}
impl CoordinateSpace for EntryLocation2dSpace {}

pub type EntryLocation2d = Vec2i<EntryLocation2dSpace>;
impl EntryLocation2d {
    const LOC_BITS: usize = 5;
    const LOC_BITMASK: usize = (1 << Self::LOC_BITS) - 1;
    pub const ENTRIES_PER_REGION: usize = (1 << Self::LOC_BITS) * (1 << Self::LOC_BITS);
    pub const SECTOR_SIZE: usize = 512;
}

impl Key<RegionPos2d> for EntryLocation2d {
    fn to_region_pos(self) -> RegionPos2d {
        RegionPos2d::new(self.x >> Self::LOC_BITS, self.z >> Self::LOC_BITS)
    }

    fn region_key(&self) -> RegionKey {
        format!("{}.{}.2dr", self.x >> Self::LOC_BITS, self.z >> Self::LOC_BITS)
    }

    fn id(&self) -> usize {
        ((self.x as usize & Self::LOC_BITMASK) << Self::LOC_BITS) | (self.z as usize & Self::LOC_BITMASK)
    }

    fn entries_per_region() -> usize {
        Self::ENTRIES_PER_REGION
    }
}

pub struct MinecraftChunkSpace {}
impl CoordinateSpace for MinecraftChunkSpace {}

pub type MinecraftChunkLocation = Vec2i<MinecraftChunkSpace>;
impl MinecraftChunkLocation {
    const LOC_BITS: usize = 5;
    const LOC_BITMASK: usize = (1 << Self::LOC_BITS) - 1;
    pub const ENTRIES_PER_REGION: usize = (1 << Self::LOC_BITS) * (1 << Self::LOC_BITS);
    pub const SECTOR_SIZE: usize = 4096;

    pub fn to_entry_location_2d(self) -> EntryLocation2d {
        EntryLocation2d {
            x: self.x,
            z: self.z,
            _phantom: PhantomData::default(),
        }
    }
}

impl MinecraftRegionPos {
    pub fn to_minecraft_chunk_location(self) -> MinecraftChunkLocation {
        self.to_minecraft_chunk_location_offset(0, 0)
    }
    pub fn to_minecraft_chunk_location_offset(self, local_x: i32, local_z: i32) -> MinecraftChunkLocation {
        MinecraftChunkLocation::new((self.x << 5) + local_x, (self.z << 5) + local_z)
    }
}

impl Key<RegionPos2d> for MinecraftChunkLocation {
    fn to_region_pos(self) -> RegionPos2d {
        RegionPos2d::new(self.x >> Self::LOC_BITS, self.z >> Self::LOC_BITS)
    }

    fn region_key(&self) -> RegionKey {
        format!("r.{}.{}.mca", self.x, self.z)
    }

    fn id(&self) -> usize {
        ((self.x as usize & Self::LOC_BITMASK) << Self::LOC_BITS) | (self.z as usize & Self::LOC_BITMASK)
    }

    fn entries_per_region() -> usize {
        Self::ENTRIES_PER_REGION
    }
}
