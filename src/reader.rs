use std::env;
use std::fs::{create_dir, File};
use std::path::Path;

const BASE_FILENAME: &str = "accounts.toml";

pub struct TOMLReader {
    pub directory: String,
    pub filename: String,
}

pub trait TOMLReaderDefault {
    fn default() -> Self;
}

impl TOMLReaderDefault for TOMLReader {
    fn default() -> Self {
        let root = env::var("HOME").unwrap();
        let store_folder = ".ac";
        let default_path = format!("{}/{}", &root, &store_folder);
        let default_file = format!("{}/{}", &default_path, &BASE_FILENAME);
        let folder_existed = Path::new(&store_folder).exists();
        let file_existed = File::open(&default_file).is_ok();
        if !folder_existed {
            create_dir(&default_path).unwrap();
        }
        if !file_existed {
            File::create(&default_file).unwrap();
        }
        TOMLReader {
            directory: default_path,
            filename: BASE_FILENAME.to_string(),
        }
    }
}
