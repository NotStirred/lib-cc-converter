use quartz_nbt::{NbtCompound, NbtTag};
use std::collections::HashMap;
use std::error::Error;

pub type ConversionFunc = dyn Fn(&mut NbtTag, usize, usize) -> Result<(), Box<dyn Error>> + Sync;

pub struct Types<F> {
    pub level: HashMap<usize, F>,
    pub player: HashMap<usize, F>,
    pub chunk: HashMap<usize, F>,
    pub hotbar: HashMap<usize, F>,
    pub options: HashMap<usize, F>,
    pub structure: HashMap<usize, F>,
    pub stats: HashMap<usize, F>,
    pub saved_data: HashMap<usize, F>,
    pub advancements: HashMap<usize, F>,
    pub poi_chunk: HashMap<usize, F>,
    pub entity_chunk: HashMap<usize, F>,
    pub tile_entity: HashMap<usize, F>,
    pub item_stack: HashMap<usize, F>,
    pub block_state: HashMap<usize, F>,
    pub entity_name: HashMap<usize, F>,
    pub entity: HashMap<usize, F>,
    pub block_name: HashMap<usize, F>,
    pub item_name: HashMap<usize, F>,
    pub untagged_spawner: HashMap<usize, F>,
    pub structure_feature: HashMap<usize, F>,
    pub objective: HashMap<usize, F>,
    pub team: HashMap<usize, F>,
    pub recipe: HashMap<usize, F>,
    pub biome: HashMap<usize, F>,
    pub world_gen_settings: HashMap<usize, F>,
}

impl<F> Types<F> {
    pub fn new() -> Self {
        Self {
            level: HashMap::new(),
            player: HashMap::new(),
            chunk: HashMap::new(),
            hotbar: HashMap::new(),
            options: HashMap::new(),
            structure: HashMap::new(),
            stats: HashMap::new(),
            saved_data: HashMap::new(),
            advancements: HashMap::new(),
            poi_chunk: HashMap::new(),
            entity_chunk: HashMap::new(),
            tile_entity: HashMap::new(),
            item_stack: HashMap::new(),
            block_state: HashMap::new(),
            entity_name: HashMap::new(),
            entity: HashMap::new(),
            block_name: HashMap::new(),
            item_name: HashMap::new(),
            untagged_spawner: HashMap::new(),
            structure_feature: HashMap::new(),
            objective: HashMap::new(),
            team: HashMap::new(),
            recipe: HashMap::new(),
            biome: HashMap::new(),
            world_gen_settings: HashMap::new(),
        }
    }
}

pub fn convert<F: Fn(&mut NbtTag, usize, usize) -> Result<(), Box<dyn Error>>>(
    _t: &HashMap<usize, F>,
    _data: &mut NbtCompound,
    _from: usize,
    _to: usize,
) {
}
