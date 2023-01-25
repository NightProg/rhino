pub mod just;
pub mod any;
pub mod between;

pub trait Rule<T> {
    fn check(&self, input: &str) -> bool;
    fn parse(&mut self, input: &str) -> Option<T>;
}