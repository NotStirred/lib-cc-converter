use crate::util::positions::RegionPos2d;
use crate::util::vec::vec2i::Vec2i;
use crate::util::vec::vec3i::Vec3i;
use crate::util::vec::CoordinateSpace;

pub type RegionKey = String;

pub trait Key {
    fn region_key(&self) -> RegionKey;
    fn id(&self) -> u32;
}

pub struct EntryLocation3dSpace {}
impl CoordinateSpace for EntryLocation3dSpace {}

pub type EntryLocation3d = Vec3i<EntryLocation3dSpace>;
impl EntryLocation3d {
    const LOC_BITS: u32 = 4;
    const LOC_BITMASK: u32 = (1 << Self::LOC_BITS) - 1;
    const ENTRIES_PER_REGION: u32 =
        (1 << Self::LOC_BITS) * (1 << Self::LOC_BITS) * (1 << Self::LOC_BITS);
}

impl Key for EntryLocation3d {
    fn region_key(&self) -> RegionKey {
        format!("{}.{}.{}.3dr", self.x, self.y, self.z)
    }

    fn id(&self) -> u32 {
        ((self.x as u32 & Self::LOC_BITMASK) << Self::LOC_BITS * 2)
            | ((self.y as u32 & Self::LOC_BITMASK) << Self::LOC_BITS)
            | (self.z as u32 & Self::LOC_BITMASK)
    }
}

pub struct EntryLocation2dSpace {}
impl CoordinateSpace for EntryLocation2dSpace {}

pub type EntryLocation2d = Vec2i<EntryLocation2dSpace>;
impl EntryLocation2d {
    const LOC_BITS: u32 = 5;
    const LOC_BITMASK: u32 = (1 << Self::LOC_BITS) - 1;
    const ENTRIES_PER_REGION: u32 = (1 << Self::LOC_BITS) * (1 << Self::LOC_BITS);
}

impl Key for EntryLocation2d {
    fn region_key(&self) -> RegionKey {
        format!("{}.{}.2dr", self.x, self.z)
    }

    fn id(&self) -> u32 {
        ((self.x as u32 & Self::LOC_BITMASK) << Self::LOC_BITS)
            | (self.z as u32 & Self::LOC_BITMASK)
    }
}

pub struct MinecraftChunkSpace {}
impl CoordinateSpace for MinecraftChunkSpace {}

pub type MinecraftChunkLocation = Vec2i<MinecraftChunkSpace>;
impl MinecraftChunkLocation {
    const LOC_BITS: u32 = 5;
    const LOC_BITMASK: u32 = (1 << Self::LOC_BITS) - 1;
    const ENTRIES_PER_REGION: u32 = (1 << Self::LOC_BITS) * (1 << Self::LOC_BITS);

    pub fn to_entry_location_2d(self) -> EntryLocation2d {
        EntryLocation2d {
            x: self.x,
            z: self.z,
            _phantom: Default::default(),
        }
    }
}

impl RegionPos2d {
    pub fn to_minecraft_chunk_location(self) -> MinecraftChunkLocation {
        self.to_minecraft_chunk_location_offset(0, 0)
    }
    pub fn to_minecraft_chunk_location_offset(
        self,
        local_x: i32,
        local_z: i32,
    ) -> MinecraftChunkLocation {
        MinecraftChunkLocation::new((self.x << 5) + local_x, (self.z << 5) + local_z)
    }
}

impl Key for MinecraftChunkLocation {
    fn region_key(&self) -> RegionKey {
        format!("r.{}.{}.mca", self.x, self.z)
    }

    fn id(&self) -> u32 {
        ((self.x as u32 & Self::LOC_BITMASK) << Self::LOC_BITS)
            | (self.z as u32 & Self::LOC_BITMASK)
    }
}
