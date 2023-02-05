use std::path::Path;

use crate::{convert::data::anvil::Data, util::positions::RegionPos2d};

use super::region_reader::{RegionData, RegionReader};

pub type AnvilRegionReader = RegionReader<RegionPos2d, fn(RegionPos2d, RegionData) -> Vec<Data>>;

pub fn create_anvil_region_reader(path: &Path) -> AnvilRegionReader {
    RegionReader::new(path, |region_pos, region_data| {
        let mut data_out = Vec::with_capacity(RegionPos2d::CHUNKS_COUNT);
        let data = &region_data.data;
        let indices = &region_data.chunk_indices;

        for x in 0..32 {
            for z in 0..32 {
                let i = x + z * RegionPos2d::DIAMETER_IN_CHUNKS;
                if let Some((start, end)) = indices[i] {
                    data_out.push(Data {
                        position: region_pos.to_minecraft_chunk_location_offset(x as i32, z as i32),
                        data: data[start..end].to_vec(),
                    });
                }
            }
        }
        data_out
    })
}
