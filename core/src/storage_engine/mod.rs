use std::path::Path;
use std::fs;

#[derive(Debug)]
pub struct StorageEngine {
    last_job: i64
}

struct Job {
    id: i64,
    data: String,
}

impl StorageEngine {
    pub  fn from_path(path :String)->StorageEngine{
        let last_job = 0;
        if !Path::new(&path).exists(){
            init_storage(&path)
        }
        StorageEngine{
            last_job : last_job,
        }
    }
}

fn init_storage(path :&str) {
    fs::create_dir_all(path).unwrap()
}


