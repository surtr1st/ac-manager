#[cfg(test)]
mod reader_tests {
    use account_manager_tui::reader::{TOMLReader, TOMLReaderDefault};
    use std::{env, fs::File, path::Path};
    const BASE_DIR: &str = "HOME";
    const FOLDER: &str = ".ac";

    #[test]
    fn default_init() {
        let toml_reader_init = TOMLReader::default();
        assert_eq!(toml_reader_init.directory, BASE_DIR);
        assert_eq!(toml_reader_init.filename, "accounts.toml");
    }

    #[test]
    fn folder_existed() {
        let dir_folder = format!("{}/{}", env::var(&BASE_DIR).unwrap(), &FOLDER);
        let path = Path::new(&dir_folder).exists();
        assert_eq!(path, true);
    }

    #[test]
    fn file_existed() {
        let dir_file = format!(
            "{}/{}/{}",
            env::var(&BASE_DIR).unwrap(),
            &FOLDER,
            "accounts.toml"
        );
        let file = File::open(dir_file).is_ok();
        assert_eq!(file, true);
    }
}
