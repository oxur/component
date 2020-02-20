use super::components::system::System;
use std::process;

#[derive(Debug, Clone)]
pub struct App {
    system: System,
}

impl App {
    pub fn init() -> App {
        App {
            system: System::new(),
        }
    }

    pub fn start(&self) {
        self.system.start()
    }

    pub fn stop(&self, exit: i32) {
        self.system.stop();
        process::exit(exit);
    }
}
