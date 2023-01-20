mod lib;
use std::path::Path;

struct FileMetaData {
  name: String,
  directory: bool,
}

impl FileMetaData {
  fn new(path: &str) -> FileMetaData {
    FileMetaData {
      name: FileMetaData::get_file_name(&path).to_string(),
      directory: FileMetaData::is_directory(&path),
    }
  }

  fn get_file_name(path: &str) -> &str {
    let file_name_os_str = Path::new(path).file_name().unwrap();
    return file_name_os_str.to_str().unwrap()
  }

  fn is_directory(path: &str) -> bool {
    Path::new(&path).is_dir() 
  }
}

pub fn process(path: &str) {
  let metadata = FileMetaData::new(&path);
  println!("Name: {}", metadata.name);
  println!("Directory: {}", metadata.directory);
}

fn convert_to_png(images: Vec<String>) -> () {

}