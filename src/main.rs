mod config;

fn main() {
    let path = String::from("path");
    let cnf = config::Config::new(path);
    print!("{:#?}",cnf);
}
