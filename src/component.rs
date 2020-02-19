#[derive(Debug)]
pub struct Component {
    pub name: &'static str,
    pub dependencies: Vec<&'static str>,
}

pub enum Status {
    Off,
    Startng,
    Running,
    Restarting,
    Restarted,
    Failing,
    Retrying,
    Recovering,
    Failed,
    Stopping,
    Terminated,
}
