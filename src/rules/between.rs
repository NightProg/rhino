use super::Rule;
use std::rc::Rc;

pub struct BetweenRule<T> {
    start_by: Rc<dyn Rule<T>>,
    end_by: Rc<dyn Rule<T>>,
    body: Rc<dyn Rule<T>>,
    input: Vec<T>
}

impl<T> BetweenRule<T> {
    pub fn new(start_by: Rc<dyn Rule<T>>, body: Rc<dyn Rule<T>>, end_by: Rc<dyn Rule<T>>) -> BetweenRule<T> {
        BetweenRule {
            start_by,
            body,
            end_by,
            input: vec![]
        }
    }
}

impl<T> Rule<T> for BetweenRule<T> {
    fn check(&self, input: &str) -> bool {
        self.start_by.check(
            input
                .chars()
                .nth(0)
                .unwrap()
                .to_string()
                .as_str()
        ) &&
            self.end_by.check(input
                .chars()
                .last()
                .unwrap()
                .to_string()
                .as_str()
            ) &&
            self.body.check(&input[1..input.len()-1])

    }

    fn parse(&mut self, input: &str) -> Option<T> {
        todo!()
    }
}