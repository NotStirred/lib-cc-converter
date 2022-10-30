use crate::types::{ConversionFunc, Types};
use quartz_nbt::{Map, NbtTag};

pub mod types;

pub struct SchemaInfo<'d> {
    pub references: Types<&'static ConversionFunc>,
    pub data: &'d mut Map<NbtTag>,
    pub from_version: usize,
    pub to_version: usize,
}

impl<'d> SchemaInfo<'d> {
    pub fn new(references: Types<&'static ConversionFunc>, data: &'d mut Map<NbtTag>, from_version: usize, to_version: usize) -> Self {
        Self {
            references,
            data,
            from_version,
            to_version,
        }
    }
}
