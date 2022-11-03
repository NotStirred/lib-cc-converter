use dfu_schema_define::define_schema;
use dfu_structures::SchemaInfo;

pub fn define(info: &mut SchemaInfo) {
    define_schema!(info, {
        register player {
            opt("Inventory", list(refer(item_stack))),
            opt("EnderItems", list(refer(item_stack)))
        },
        register chunk {
            req("Level", all(
                opt("Entities", list(refer(entity))),
                opt("TileEntitites", list(refer(tile_entity))),
                opt("TileTicks", list(req("i", refer(block_name))))
            ))
        }
    });
}
