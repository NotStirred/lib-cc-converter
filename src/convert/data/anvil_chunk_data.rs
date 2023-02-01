use crate::convert::entry_location::MinecraftChunkLocation;

#[derive(Clone)]
pub struct AnvilChunkData {
    pub position: MinecraftChunkLocation,
    pub data: Vec<u8>,
}
