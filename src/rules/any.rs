use super::Rule;

pub struct AnyRule {
    input: Option<String>
}

impl AnyRule {
    pub fn new() -> AnyRule {
        AnyRule {
            input: None
        }
    }

    pub fn input(self) -> Option<String> {
        self.input
    }
}

impl<T> Rule<T> for AnyRule {
    fn check(&self, input: &str) -> bool {
        true
    }

    fn parse(&mut self, input: &str) -> Option<T> {
        self.input = Some(input.to_string());
        None
    }
}