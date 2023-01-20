mod cli;
mod file;
use std::path::Path;


fn main() {
    let args = cli::args();
    let path = &args[1];

    match Path::new(&path).try_exists() {
        Ok(exist) =>  if exist {
            file::process(&path)
        } else {
            println!("Path doesn't exist")
        },
        Err(_) => println!("Path error!")
    }
}
  