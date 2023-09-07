use ron::de::from_reader;
use serde::de::DeserializeOwned;
use std::{fs::File, path::PathBuf};

mod config;

pub use config::*;

/// Opens a file using the project's manifest directory as the root.
pub fn open_local_file(path: &str) -> File {
    let input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path);
    File::open(input_path)
        .expect(&format!("Failed opening file: {:#?}", path)[..])
}

/// Opens and loads a `*.ron` config file into a compatible struct.
pub fn load_config_from_file<T: DeserializeOwned>(path: &str) -> T {
    let f = open_local_file(path);

    match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);

            std::process::exit(1);
        },
    }
}
