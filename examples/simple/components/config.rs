#[derive(Debug, Clone, Copy)]
pub struct Config {}

impl Config {
    pub fn new() -> Config {
        Config {}
    }

    pub fn start(&self) {
        println!("Starting config component ...")
    }

    pub fn stop(&self) {
        println!("Stopping config component ...")
    }
}
