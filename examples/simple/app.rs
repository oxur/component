// use super::components::system;

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

    pub fn stop(&self) {
        self.system.stop()
    }
}
