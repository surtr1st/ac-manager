pub mod manager;
pub mod query;

use manager::FileManager;

fn main() {
    let dir = String::from("/home/ishi/Tests/accounts");
    let mut file_manager = FileManager {
        directory: String::from(""),
        files: Vec::new(),
    };

    file_manager.set_directory(dir);
    file_manager.read_directory();
    println!("{:?}", file_manager.read_content(0));
}
