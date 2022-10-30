use crate::convert::data_fix::schema::{convert, convert_all, Schema, Types};
use quartz_nbt::{Map, NbtCompound, NbtList, NbtTag};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

const VERSION_99: usize = 0;

#[derive(Debug)]
struct AnError {}

impl Display for AnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for AnError {}

fn asd123(
    types: &mut Types<&'static Schema>,
    data: &mut Map<NbtTag>,
    from_version: usize,
    to_version: usize,
) -> Result<(), Box<dyn Error>> {
    if let Some(inventory) = data.get_mut("Inventory") {
        let inventory: &mut NbtList = inventory.try_into()?;
        convert_all(&types.item_stack, inventory, from_version, to_version);
    }
    if let Some(ender_items) = data.get_mut("EnderItems") {
        let ender_items: &mut NbtList = ender_items.try_into()?;
        convert_all(&types.item_stack, ender_items, from_version, to_version);
    }

    Ok(())
}

fn register(types: &'static mut Types<&'static Schema>) {
    let player = |data: &mut Map<NbtTag>, from_version, to_version| {
        if let Some(inventory) = data.get_mut("Inventory") {
            let inventory: &mut NbtList = inventory.try_into()?;
            convert_all(&types.item_stack, inventory, from_version, to_version);
        }
        if let Some(ender_items) = data.get_mut("EnderItems") {
            let ender_items: &mut NbtList = ender_items.try_into()?;
            convert_all(&types.item_stack, ender_items, from_version, to_version);
        }

        Ok(())
    };
    types.player.insert(VERSION_99, Box::leak(Box::new(player)));

    let chunk = |data: &mut Map<NbtTag>, from_version, to_version| {
        let level: &mut NbtCompound = data.get_mut("Level").ok_or_else(|| Box::new(AnError {}))?.try_into()?;
        let level = level.inner_mut();

        if let Some(entities) = level.get_mut("Entities") {
            convert_all(&types.entity, entities.try_into()?, from_version, to_version);
        }
        if let Some(tile_entities) = level.get_mut("TileEntities") {
            convert_all(&types.tile_entity, tile_entities.try_into()?, from_version, to_version);
        }

        if let Some(tile_ticks) = level.get_mut("TileTicks") {
            let tile_ticks: &mut NbtList = tile_ticks.try_into()?;
            for i in 0..tile_ticks.len() {
                let tile_tick = &mut tile_ticks[i];
                convert(&types.block_name, tile_tick, from_version, to_version);
            }
        }
        Ok::<(), Box<dyn Error>>(())
    };
    types.chunk.insert(VERSION_99, Box::leak(Box::new(chunk)));
}
