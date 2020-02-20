use std::process;

#[derive(Debug, Clone)]
pub struct App {
    system: super::components::system::System,
}

impl App {
    pub fn init() -> App {
        App {
            system: super::components::system::System::new(),
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
