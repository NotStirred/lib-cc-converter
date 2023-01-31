pub mod convert;
pub mod converter;
pub mod io;
pub mod util;

#[cfg(test)]
mod tests {
    use crate::convert::anvil2cc::Anvil2CCConverter;
    use crate::convert::data::anvil_chunk_data::AnvilChunkData;
    use crate::convert::entry_location::EntryLocation3d;
    use crate::io::anvil_region_reader::AnvilRegionReader;
    use crate::io::region_writer::CachingRegionWriter;
    use crate::util::positions::RegionPos2d;
    use crate::util::positions::RegionPos3d;
    use crate::util::test_utils;

    #[test]
    fn simple_region_test() {
        let test_resources_path = test_utils::test_resources_path();

        let reader = AnvilRegionReader::new(&test_resources_path.join("simple_region_test"));
        let region_pos = RegionPos2d::new(-1, 0);
        let mut region_data = match reader.read(region_pos) {
            Ok(data) => data,
            Err(err) => {
                panic!("{}, {}", region_pos, err);
            }
        };

        let data = &mut region_data.data;
        let indices = &region_data.chunk_indices;

        let converter = Anvil2CCConverter::new(true);

        let mut writer: CachingRegionWriter<RegionPos3d> =
            CachingRegionWriter::new(&test_resources_path.join("simple_region_test/out"), 512, 64)
                .expect("Failed to create CubicRegionWriter");
        for x in 0..32 {
            for z in 0..32 {
                let i = x + z * RegionPos2d::DIAMETER_IN_CHUNKS;
                if let Some((start, end)) = indices[i] {
                    let cube_data_array = match converter.convert(AnvilChunkData {
                        data: &data[start..end],
                        position: region_pos.to_minecraft_chunk_location_offset(x as i32, z as i32),
                    }) {
                        Ok(data) => data,
                        Err(err) => {
                            panic!("{}", err);
                        }
                    };

                    for cube_data in cube_data_array.iter() {
                        for (y, data) in cube_data.cube_data.iter() {
                            let entry_location = EntryLocation3d::new(cube_data.position.x, *y, cube_data.position.z);

                            match writer.write(entry_location, data) {
                                Ok(_) => {}
                                Err(err) => {
                                    panic!("{}", err);
                                }
                            };
                        }
                    }
                }
            }
        }
        writer.flush().unwrap();
    }
}
