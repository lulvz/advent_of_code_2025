use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub mod utils {
    use super::*;

    pub fn read_input(filename: &str) -> io::Result<String> {
        let path = Path::new("./inputs/").join(filename);
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        Ok(contents.to_string()) 
    }
}