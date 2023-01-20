mod cli;
use std::path::Path;
fn main() {
    let args = cli::args();
    let path = &args[1];

    println!("Path: {}", &path);
    println!("Output Path: output");

    assert_eq!(Path::new("data").try_exists(), false);
    match Path::new(&path).try_exists() {
        Ok(exist) =>  if exist {
            println!("Path exist")
        } else {
            println!("Path doesn't exist")
        },
        Err(_) => println!("Path error!")
    }
}
