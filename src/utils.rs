mod png_convert;
use std::{io, fs};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use image_compressor::{FolderCompressor, Factor};



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
    let file_path = Path::new(path);
    let file_name_os_str = file_path.file_stem().unwrap();
    if !FileMetaData::is_directory(path) {
      if file_path.extension().unwrap().to_str().unwrap() != "heic" {
        panic!("File format not supported");
      }
    }
    return file_name_os_str.to_str().unwrap()
  }

  fn is_directory(path: &str) -> bool {
    Path::new(&path).is_dir() 
  }
}

pub fn process(path: &str) -> () {
  let metadata = FileMetaData::new(&path);
  if !metadata.directory {
    png_convert::convert_to_png(metadata.path, metadata.name).expect("Can't convert to PNG");
  } else {
    convert_dir_to_png(&path);
  }

  compress_image_folder();

}

fn convert_dir_to_png(path: &str) -> io::Result<()> {
  let mut entries = fs::read_dir(&path)?
    .map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, io::Error>>()?;
  
  entries.sort();
  Ok(())
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