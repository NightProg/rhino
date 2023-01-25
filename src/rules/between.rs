use super::Rule;
use std::rc::Rc;

struct BetweenRule<T> {
    start_by: Rc<dyn Rule<T>>,
    end_by: Rc<dyn Rule<T>>,
    body: Rc<dyn Rule<T>>,
    input: Vec<T>
}

impl<T> BetweenRule<T> {
    fn new(start_by: Rc<dyn Rule<T>>, body: Rc<dyn Rule<T>>, end_by: Rc<dyn Rule<T>>) -> BetweenRule<T> {
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
        todo!()
    }

    fn parse(&mut self, input: &str) -> Option<T> {
        todo!()
    }
}