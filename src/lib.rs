mod rules;

use std::fmt::Debug;
use std::process::Output;
use std::rc::Rc;
use rules::Rule;
use rules::just::JustRule;
use rules::any::AnyRule;

#[derive(Debug, PartialEq, Clone)]
enum Ast {
    A, B, C
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_just() {
        let mut d = JustRule::new(">", Rc::new(Ast::A));
        assert_eq!(d.parse(">"), Some(Rc::new(Ast::A)));
    }

    #[test]
    fn test_function_any() {
        let mut d = AnyRule::new();
        <AnyRule as Rule<()>>::parse(&mut d, "hhh");
        assert_eq!(d.input(), Some(String::from("hhh")))
    }
}
