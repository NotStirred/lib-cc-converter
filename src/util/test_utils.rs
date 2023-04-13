use std::path::PathBuf;

#[cfg(test)]
pub fn test_resources_path() -> PathBuf {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/test");
    d
}
