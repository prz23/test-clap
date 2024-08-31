mod config;
mod types;


fn main() {
    let config = config::Config::from_args();
    println!("config {config:?}");
}
