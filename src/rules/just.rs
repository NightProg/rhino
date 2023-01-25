use std::rc::Rc;
use super::Rule;

pub struct JustRule<'a, T> {
    prompt: &'a str,
    to: T
}

impl<'a, T> JustRule<'a, T> {
    pub fn new(str_to_respect: &'a str, to: T) -> JustRule<'a, T> {
        JustRule {
            prompt: str_to_respect,
            to
        }
    }
}

impl<'a, T: Clone> Rule<T> for JustRule<'a, T> {

    fn check(&self, input: &str) -> bool {
        input == self.prompt
    }

    fn parse(&mut self, input: &str) -> Option<T> {
        if input == self.prompt {
            Some(self.to.clone())
        } else {
            None
        }
    }
}