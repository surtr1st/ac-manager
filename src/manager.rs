use std::{fs, fs::File, io::Write, path::PathBuf};

#[derive(Debug, Clone)]
pub struct FileManager {
    pub directory: String,
    pub files: Vec<PathBuf>,
}

impl FileManager {
    pub fn set_directory(&mut self, dir: String) {
        self.directory = dir;
    }

    pub fn read_directory(&mut self) {
        let paths = fs::read_dir(&self.directory).unwrap();
        for path in paths {
            let file_path = path.unwrap().path();
            self.files.push(file_path);
        }
    }

    pub fn read_content(self, index: i32) -> String {
        let mut selected_file = &self.files[index as usize];
        let content = fs::read_to_string(&mut selected_file).unwrap();
        content.to_string()
    }

    pub fn write(self, file: String, content: String) {
        let mut new_file = File::create(file).unwrap();
        new_file.write_all(content.as_bytes()).unwrap();
    }
}
