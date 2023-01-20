use std::path::Path;
use crate::lib::png_convert::convert_to_png;
use std::path::PathBuf;
use std::sync::mpsc;
use image_compressor::FolderCompressor;
use image_compressor::Factor;


struct FileMetaData {
  name: String,
  directory: bool,
  path: String
}

impl FileMetaData {
  fn new(path: &str) -> FileMetaData {
    FileMetaData {
      name: FileMetaData::get_file_name(&path).to_string(),
      directory: FileMetaData::is_directory(&path),
      path: path.to_string(),
    }
  }

  fn get_file_name(path: &str) -> &str {
    let file_name_os_str = Path::new(path).file_stem().unwrap();
    return file_name_os_str.to_str().unwrap()
  }

  fn is_directory(path: &str) -> bool {
    Path::new(&path).is_dir() 
  }
}

pub fn process(path: &str) -> () {
  let metadata = FileMetaData::new(&path);
  convert_to_png(metadata.path, metadata.name).expect("Can't convert to PNG");
  compress_image_folder();
}

fn compress_image_folder() -> () {
  let origin = PathBuf::from("output/uncompressed");
  let dest = PathBuf::from("output/compressed");
  let thread_count: u32= 4;
  let (tx, _tr) = mpsc::channel();

  let mut comp = FolderCompressor::new(origin, dest);
  comp.set_factor(Factor::new(75., 0.7));
  comp.set_thread_count(thread_count);
  comp.set_sender(tx);

  match comp.compress(){
      Ok(_) => {},
      Err(e) => println!("Cannot compress the folder!: {}", e),
  }
}