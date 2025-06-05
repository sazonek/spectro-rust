use crate::config::CONFIG_FILEPATH;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const DEFAULT_CONFIG: &str = include_str!("default_config");

pub fn init_config() {
    if !Path::new(CONFIG_FILEPATH).exists() {
        let mut file = File::create(CONFIG_FILEPATH).expect("couldn't create file");
        file.write_all(DEFAULT_CONFIG.as_bytes())
            .expect("couldn't write to file");
    }
}
