pub mod graph;
pub mod utils;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    Wildcard,
    String(String),
}

#[derive(Debug, Clone)]
pub struct Value(usize);

pub trait Analytics {
    // the space that the algorithm uses
    fn space_occupied(&self) -> usize;
}

pub trait Storage<const N: usize> {
    fn increment(&mut self, key: [String; N]) -> anyhow::Result<()>;
    fn get(&self, key: [Key; N]) -> anyhow::Result<Value>;
}
