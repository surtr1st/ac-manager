#[cfg(test)]
mod reader_tests {
    use account_manager_tui::reader::TOMLReader;
    use std::{env, fs::File, path::Path};
    const BASE_DIR: &str = "HOME";
    const FOLDER: &str = ".ac";
    const FILENAME: &str = "accounts.toml";

    #[test]
    fn default_init() {
        let reader: TOMLReader = TOMLReader::new();
        let default_dir = format!("{}/{}", env::var(BASE_DIR).unwrap(), &FOLDER);
        assert_eq!(reader.directory, default_dir);
        assert_eq!(reader.filename, FILENAME);
    }

    #[test]
    fn folder_existed() {
        let dir_folder = format!("{}/{}", env::var(&BASE_DIR).unwrap(), &FOLDER);
        let path = Path::new(&dir_folder).exists();
        assert_eq!(path, true);
    }

    #[test]
    fn file_existed() {
        let dir_file = format!("{}/{}/{}", env::var(&BASE_DIR).unwrap(), &FOLDER, &FILENAME);
        let file = File::open(dir_file).is_ok();
        assert_eq!(file, true);
    }

    #[test]
    fn read_file_content() {
        let reader: TOMLReader = TOMLReader::new();
        let data = reader.read_from_file();
        let account = &data["Twitter"];
        let username = account["username"].as_str().unwrap();
        let password = account["password"].as_str().unwrap();
        assert_eq!(username, "adudarkwa");
        assert_eq!(password, "123456");
    }
}
