use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    println!("test");
    println!("Hello, world!");

    let data = load_file("test.toml".to_string());
    println!("{}", data);
}

fn load_file(file_name: String) -> String {
    let path = Path::new(&file_name);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldnt open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                           why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    s
}
