use crate::convert::entry_location::MinecraftChunkLocation;

pub struct AnvilChunkData<'a> {
    pub position: MinecraftChunkLocation,
    pub data: &'a [u8],
}
