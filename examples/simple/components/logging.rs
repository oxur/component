#[derive(Debug, Clone, Copy)]
pub struct Logging {}

impl Logging {
    pub fn new() -> Logging {
        Logging {}
    }

    pub fn start(&self) {
        println!("Starting logging component ...")
    }

    pub fn stop(&self) {
        println!("Stopping logging component ...")
    }
}
