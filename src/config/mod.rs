mod storage;

#[derive(Debug)]
pub struct Config {
    storage:  storage::Config
}

impl Config {
    pub fn new(path: String) ->Config{
        Config{
            storage : storage::Config::new(path)
        }
    }
}