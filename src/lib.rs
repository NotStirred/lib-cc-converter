pub mod convert;
pub mod io;
pub mod util;

#[cfg(test)]
mod tests {

    use crate::convert::anvil2cc::Anvil2CCConverter;

    use crate::convert::converter::run_conversion;
    use crate::io::anvil_region_reader::create_anvil_region_reader;
    use crate::io::cubic_region_writer::CubicRegionWriter;

    use crate::util::test_utils;

    #[test]
    fn anvil2cc_test() {
        let src_path = test_utils::test_resources_path().join("anvil2cc");
        let dst_path = test_utils::test_resources_path().join("anvil2cc/out");

        let reader = create_anvil_region_reader(&src_path);
        let converter = Anvil2CCConverter::new(true);
        let writer = CubicRegionWriter::new(&dst_path, 64).unwrap();

        run_conversion(reader, converter, writer);
    }
}
