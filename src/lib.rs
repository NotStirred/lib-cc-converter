pub mod convert;
pub mod io;
pub mod util;

#[cfg(test)]
mod tests {

    use crate::convert::anvil2cc::Anvil2CCConverter;

    use crate::convert::converter::run_conversion;
    use crate::io::anvil_region_reader::AnvilRegionReader;
    use crate::io::cubic_chunks_writer::CubicChunksWriter;

    use crate::util::test_utils;

    #[test]
    fn simple_region_test() {
        let src_path = test_utils::test_resources_path().join("simple_region_test");
        let dst_path = test_utils::test_resources_path().join("simple_region_test/out");

        let reader = AnvilRegionReader::new(&src_path);
        let converter = Anvil2CCConverter::new(true);
        let writer = CubicChunksWriter::new(&dst_path, 64).unwrap();

        run_conversion(reader, converter, writer);
    }
}
