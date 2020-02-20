#[derive(Debug, Clone)]
pub struct System {
    config: super::config::Config,
    logging: super::logging::Logging,
    server: super::server::Server,
}

impl System {
    pub fn new() -> System {
        // XXX set up components
        System {
            config: super::config::Config::new(),
            logging: super::logging::Logging::new(),
            server: super::server::Server::new(),
        }
    }

    pub fn start(&self) {
        self.config.start();
        self.logging.start();
        self.server.start();
    }

    pub fn stop(&self) {
        self.server.stop();
        self.logging.stop();
        self.config.stop();
    }
}
