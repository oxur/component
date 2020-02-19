pub use self::deps::*;

pub mod deps;

#[derive(Debug)]
pub struct Component {
    pub name: &'static str,
    pub dependencies: Vec<&'static str>,
}
