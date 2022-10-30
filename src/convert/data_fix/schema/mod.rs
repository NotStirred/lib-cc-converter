use lazy_static::lazy_static;
use quartz_nbt::{Map, NbtList, NbtTag};
use std::collections::HashMap;
use std::error::Error;

pub mod v99;

pub type Schema = dyn Fn(&mut Map<NbtTag>, usize, usize) -> Result<(), Box<dyn Error>> + Sync;

pub struct Types<F: Fn(&mut Map<NbtTag>, usize, usize) -> Result<(), Box<dyn Error>> + Sync> {
    level: HashMap<usize, F>,
    player: HashMap<usize, F>,
    chunk: HashMap<usize, F>,
    hotbar: HashMap<usize, F>,
    options: HashMap<usize, F>,
    structure: HashMap<usize, F>,
    stats: HashMap<usize, F>,
    saved_data: HashMap<usize, F>,
    advancements: HashMap<usize, F>,
    poi_chunk: HashMap<usize, F>,
    entity_chunk: HashMap<usize, F>,
    tile_entity: HashMap<usize, F>,
    item_stack: HashMap<usize, F>,
    block_state: HashMap<usize, F>,
    entity_name: HashMap<usize, F>,
    entity_tree: HashMap<usize, F>,
    entity: HashMap<usize, F>,
    block_name: HashMap<usize, F>,
    item_name: HashMap<usize, F>,
    untagged_spawner: HashMap<usize, F>,
    structure_feature: HashMap<usize, F>,
    objective: HashMap<usize, F>,
    team: HashMap<usize, F>,
    recipe: HashMap<usize, F>,
    biome: HashMap<usize, F>,
    world_gen_settings: HashMap<usize, F>,
}

impl<F: Fn(&mut Map<NbtTag>, usize, usize) -> Result<(), Box<dyn Error>> + Sync> Types<F> {
    fn new() -> Self {
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
            entity_tree: HashMap::new(),
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

pub fn convert_all<F: Fn(&mut Map<NbtTag>, usize, usize) -> Result<(), Box<dyn Error>>>(
    t: &HashMap<usize, F>,
    data: &mut NbtList,
    from: usize,
    to: usize,
) {
}

pub fn convert<F: Fn(&mut Map<NbtTag>, usize, usize) -> Result<(), Box<dyn Error>>>(
    t: &HashMap<usize, F>,
    data: &mut NbtTag,
    from: usize,
    to: usize,
) {
}

lazy_static! {
    static ref TYPES: Types<&'static Schema> = Types::new();
}
