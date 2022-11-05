use crate::types::{ConversionFunc, Types};
use quartz_nbt::{Map, NbtTag};
use std::error::Error;

pub mod types;

pub fn call_closure_with<'a, F: Fn(&'a mut NbtTag, usize, usize) -> Result<(), Box<dyn Error>>>(
    f: F,
    value: &'a mut NbtTag,
    from_ver: usize,
    to_ver: usize,
) -> Result<(), Box<dyn Error>> {
    f(value, from_ver, to_ver)
}

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
