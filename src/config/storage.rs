#[derive(Debug)]
pub struct Config{
    path: String
}

impl Config {
    pub fn new(path: String)->Config{
        Config{
            path
        }
    }
}