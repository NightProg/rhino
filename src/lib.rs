use std::fmt::Debug;
use std::rc::Rc;

#[derive(Clone)]
pub struct Rule<'a, T> {
    check: Rc<dyn Fn(&str) -> bool>,
    parse: Option<Rc<dyn Fn(&str) -> Option<T>>>,
    saved_data: Vec<&'a str>
}

impl<'a, T: Clone + 'static> Rule<'a, T> {
    fn or(self, rhs: Self) -> Rule<'a, T> {
        Rule {
            check: Rc::new(move |e| {
                (rhs.check)(e) || (self.check)(e)
            }),
            parse: None,
            saved_data: self.saved_data,
        }
    }
    fn and(self, rhs: Self) -> Rule<'a, T> {
        Rule {
            check: Rc::new(move |e| {
                (rhs.check)(e) && (self.check)(e)
            }),
            parse: None,
            saved_data: self.saved_data,
        }
    }

    fn to(mut self, t: T) -> Rule<'a, T> {
        Rule {
            check: self.clone().check,
            parse: Some(
                Rc::new(move |e| {
                    if ! (self.check)(e) {
                        None
                    } else {
                        Some(t.clone())
                    }

                })
            ),
            saved_data: self.saved_data,
        }
    }


}

struct Prompt<'a, T> {
    rule: Rule<'a, T>,
    prompt: String
}

impl<'a, T> Prompt<'a, T> {
    fn new(rule: Rule<'a, T>) -> Self {
        Prompt {
            rule,
            prompt: String::new()
        }
    }
}

pub struct Group<'a, T>(Vec<Rule<'a, T>>);

impl<'a, T> Group<'a, T> {

}


#[derive(Clone, PartialEq, Debug)]
pub enum Ast {
    A(i32),
    B(String),
    C
}

pub fn just<'a, T>(s: String) -> Rule<'a, T> {

    Rule {
        check: Rc::new(move |e| {
            e == s.as_str()
        }),
        parse: None,
        saved_data: vec![]
    }
}



pub fn any<'a, T>() -> Rule<'a, T> {

    Rule {
        check: Rc::new(|e| {
            true
        }),
        parse: None,
        saved_data: vec![]
    }
}

pub fn check<T>(r: Rule<T>, s: &str) -> bool {
    (r.check)(s)
}

pub fn parse<T>(r: Rule<T>, s: &str) -> Option<T> {
    if let Some(e) = r.parse {
        (e)(s)
    } else {
        None
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_just() {
        let result = just::<Ast>("<".to_string());
        assert_eq!(check(result, "<"), true);
    }

    #[test]
    fn test_function_parse() {
        let result = just("<".to_string()).to(Ast::C);
        assert_eq!(parse(result, "<"), Some(Ast::C))
    }
}
