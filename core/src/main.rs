extern crate serde;
#[macro_use]
extern crate serde_derive;

mod config;
mod storage_engine;


fn main() {
    const DEFAULT_CONFIG_PATH :&str = "/tmp/config";
    
    let path = String::from(DEFAULT_CONFIG_PATH);
    let cnf = config::Config::from_path(path);
    let storage_engine = storage_engine::StorageEngine::from_path(cnf.storage.path);
    println!("{:?}", storage_engine);
}