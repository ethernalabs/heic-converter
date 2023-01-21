mod png_convert;
use std::{io, fs};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use image_compressor::{FolderCompressor, Factor};


#[derive(Debug)]
struct FileMetaData {
  name: String,
  directory: bool,
  path: String
}

impl FileMetaData {
  fn new(path: &str) -> Result<FileMetaData, &'static str> {
    match FileMetaData::get_file_name(&path) {
      Ok(name) => Ok(FileMetaData {
                    name: name,
                    directory: Path::new(&path).is_dir(),
                    path: path.to_string(),
                  }),
      Err(e) => Err(e)
    }
  }

  fn get_file_name(path: &str) -> Result<String, &'static str> {
    let file_path = Path::new(path);
    let file_name_os_str = file_path.file_stem().unwrap();

    if file_path.is_dir() || 
       file_path.extension().unwrap().to_str().unwrap() == "heic" { 
      Ok(file_name_os_str.to_str().unwrap().to_owned())
    } else {
      Err("File type not supported")
    }
  }
}

pub fn process(path: &str) -> () {
  let metadata = FileMetaData::new(&path);
  match metadata {
    Ok(m) => if !m.directory {
              png_convert::convert_to_png(m.path, m.name).expect("Can't convert to PNG");
             } else {
              convert_dir_to_png(&path).expect("Can't convert the files inside the directory");
             }
    Err(e) => println!("{}", e)
  }

  compress_image_folder();

}

fn convert_dir_to_png(path: &str) -> io::Result<()> {
  let mut entries = fs::read_dir(&path)?
    .map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, io::Error>>()?;
  
  entries.sort();

  for file in entries {
    let file_path = file.to_str().unwrap();
    match FileMetaData::new(&file_path) {
      Ok(m) => png_convert::convert_to_png(m.path, m.name).expect("Can't convert to PNG"),
      Err(e) => eprintln!("{}: {}", e, file_path)
    }
  }
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