mod rules;
mod macros;

use std::fmt::Debug;
use std::process::Output;
use std::rc::Rc;
use rules::Rule;
use rules::just::JustRule;
use rules::any::AnyRule;
use rules::between::BetweenRule;

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

    #[test]
    fn test_between_rule() {
        let mut d = BetweenRule::new(
            Rc::new(JustRule::new("<", Rc::new(Ast::A))),
            Rc::new(AnyRule::new()),
            Rc::new(JustRule::new(">", Rc::new(Ast::B)))
        );
        assert!(d.check("<>"));
        assert!(!d.check("a>"))
    }
}
