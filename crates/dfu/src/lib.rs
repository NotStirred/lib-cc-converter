mod schema;

use crate::schema::v99;

use dfu_structures::types::{ConversionFunc, Types};
use dfu_structures::SchemaInfo;
use lazy_static::lazy_static;
use quartz_nbt::NbtCompound;

fn create_types() -> &'static Types<&'static ConversionFunc> {
    let mut compound = NbtCompound::new();
    let map = compound.inner_mut();
    let mut info = SchemaInfo::new(Types::new(), map, 0, 1);

    v99::define(&mut info);

    Box::leak(Box::new(info.references))
}

lazy_static! {
    pub static ref TYPES: &'static Types<&'static ConversionFunc> = create_types();
}
