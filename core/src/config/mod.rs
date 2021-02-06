extern crate toml;

use std::io::prelude::*;
use std::fs::File;



#[derive(Debug, Deserialize)]
pub struct Storage{
    pub path: String
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub storage:  Storage
}

impl Config {
  
    pub fn from_path(path: String) ->Config{
        let mut cf = File::open(path).unwrap();
        let mut cdata = String::new();
        cf.read_to_string(&mut cdata).unwrap();
        let config: Config = toml::from_str(&*cdata).unwrap();
        config
    }
}

