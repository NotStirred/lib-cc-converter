use dfu_schema_define::define_schema;
use dfu_structures::SchemaInfo;

pub fn define(info: &mut SchemaInfo) {
    define_schema!(info, {
        register player {
            opt("Inventory", list(item_stack)),
            opt("EnderItems", list(item_stack))
        }
    });
}
