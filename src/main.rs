mod cli;
mod lib;
use std::path::Path;
use std::fs;

fn main() {
  let args = cli::args();
  let path = &args[1];


  // check if output directory is present
  // if not create
  match Path::new("output").try_exists() {
      Ok(exist) => if !exist {
        fs::create_dir("output");
        fs::create_dir("output/uncompressed");
        fs::create_dir("output/compressed");
      }
      Err(_) => ()
  }

  match Path::new(&path).try_exists() {
      Ok(exist) =>  if exist {
          lib::file::process(&path);
      } else {
          println!("Path doesn't exist")
      },
      Err(_) => println!("Path error!")
  }
}
  