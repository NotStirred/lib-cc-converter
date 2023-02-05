use crate::convert::entry_location::MinecraftChunkLocation;

#[derive(Clone)]
pub struct Data {
    pub position: MinecraftChunkLocation,
    pub data: Vec<u8>,
}
