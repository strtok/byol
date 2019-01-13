use regex::Regex;
use std::rc::Rc;
use std::cell::RefCell;

pub enum ParseResult<'a> {
    Value { value: ParseValue, remaining_input: &'a str},
    Error { text: String }
}

impl<'a> ParseResult<'a> {
    pub fn is_value(&self) -> bool {
        match *self {
            ParseResult::Value {..} => true,
            _ => false
        }
    }

    pub fn is_empty(&self) -> bool {
        match *self {
            ParseResult::Value {value: ParseValue::Empty, remaining_input: _} => true,
            _ => false
        }
    }

    pub fn is_error(&self) -> bool {
        match *self {
            ParseResult::Error {..} => true,
            _ => false
        }
    }
}

pub enum ParseValue {
    String(String),
    List(Vec<ParseValue>),
    Empty
}

impl ParseValue {
    pub fn string(&self) -> &str {
        if let ParseValue::String(s) = self {
            return &s;
        } else {
            panic!("unexpected type");
        }
    }
}

pub fn succeed() -> impl Fn(&str) -> ParseResult {
    |input: &str| {
        ParseResult::Value{value: ParseValue::Empty, remaining_input: input}
    }
}

pub fn satisfy(predicate: impl Fn(char) -> bool) -> impl Fn(&str) -> ParseResult {
    move |input: &str| {
        if input.is_empty() {
            return ParseResult::Error {text: "satisfy not satisfied".to_string()}
        }
        let c = &input[0..1];
        if predicate(c.chars().next().unwrap()) {
            ParseResult::Value {
                value: ParseValue::String(c.to_string()),
                remaining_input: &input[1..]
            }
        } else {
            ParseResult::Error {text: "satisfy not satisfied".to_string()}
        }
    }
}

pub fn sym(c: char) -> impl Fn(&str) -> ParseResult {
    satisfy(move |_c| { c == _c })
}

pub fn regex(regex: &str) -> impl Fn(&str) -> ParseResult {
    let re = Regex::new(regex).unwrap();
    satisfy(move |c: char| {
        return re.is_match(&c.to_string());
    })
}

pub fn digit() -> impl Fn(&str) -> ParseResult {
    satisfy(|c: char| {
        c.is_digit(10)
    })
}

pub fn alphanumeric() -> impl Fn(&str) -> ParseResult {
    satisfy(|c: char| {
        c.is_alphanumeric()
    })
}

pub fn alphabetic() -> impl Fn(&str) -> ParseResult {
    satisfy(|c: char| {
        c.is_alphabetic()
    })
}

pub fn repeat(parser: impl Fn(&str) -> ParseResult) -> impl Fn(&str) -> ParseResult {
    move |input: &str| {
        let mut remaining = input;
        let mut values: Vec<ParseValue> = Vec::new();
        loop {
            match parser(remaining) {
                ParseResult::Value{value, remaining_input} => {
                    values.push(value);
                    remaining = remaining_input;
                },
                ParseResult::Error{text} => {
                    if (values.is_empty()) {
                        return ParseResult::Value{value: ParseValue::Empty, remaining_input: remaining}
                    } else {
                        return ParseResult::Value{value: ParseValue::List(values), remaining_input: remaining}
                    }
                }
                result => {
                    return result;
                }
            }
        }
    }
}

pub fn repeat1(parser: impl Fn(&str) -> ParseResult) -> impl Fn(&str) -> ParseResult {
    let repeat_parser = repeat(parser);
    move |input: &str| {
        match repeat_parser(input) {
            ParseResult::Value{value: ParseValue::Empty, remaining_input: _} => {
                ParseResult::Error{text: "expected at least one value".to_string()}
            },
            result => result
        }
    }
}

pub fn one_of(parsers: Vec<Box<dyn Fn(&str) -> ParseResult>>) -> impl Fn(&str) -> ParseResult {
    move |input: &str| {
        for parser in &parsers {
            match parser(input) {
                ParseResult::Value{value, remaining_input} => {
                    return ParseResult::Value{value, remaining_input}
                }
                _ => continue
            }
        }
        ParseResult::Error{text: "expected at least one or() value".to_string()}
    }
}

#[macro_export]
macro_rules! one_of {
    ( $( $x:expr ),* ) => {
        {
            let mut v: Vec<Box<dyn Fn(&str) -> parser::ParseResult>> = Vec::new();
            $(
                v.push(Box::new($x));
            )*
            parser::one_of(v)
        }
    };
}

pub fn seq(parsers: Vec<Box<dyn Fn(&str) -> ParseResult>>) -> impl Fn(&str) -> ParseResult {
    move |input: &str| {
        let mut remaining = input;
        let mut values: Vec<ParseValue> = Vec::new();
        for parser in &parsers {
            match parser(remaining) {
                ParseResult::Value{value, remaining_input} => {
                    values.push(value);
                    remaining = remaining_input;
                },
                result => return result
            }
        }
        return ParseResult::Value {value: ParseValue::List(values), remaining_input: remaining };
    }
}

#[macro_export]
macro_rules! seq {
    ( $( $x:expr ),* ) => {
        {
            let mut v: Vec<Box<dyn Fn(&str) -> parser::ParseResult>> = Vec::new();
            $(
                v.push(Box::new($x));
            )*
            parser::seq(v)
        }
    };
}

pub fn boxed() -> Rc<RefCell<Box<dyn Fn (&str) -> ParseResult>>> {
    Rc::new(RefCell::new(Box::new(|_input: &str| { ParseResult::Error{text: "uninitialized parser".to_string()} })))
}

#[cfg(test)]
mod tests {
    use crate::parser;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn succeed() {
        assert!(parser::succeed()("123").is_empty());
    }

    #[test]
    fn satisfy() {
        let f = parser::satisfy(|_c| { true });

        if let parser::ParseResult::Value { value: parser::ParseValue::String(str), remaining_input } = f("abc") {
            assert_eq!(str, "a");
            assert_eq!(remaining_input, "bc");
        } else {
            panic!("fail");
        }
    }

    #[test]
    fn satisfy_no_input() {
        let f = parser::satisfy(|_c: char| {true});
        assert!(f("").is_error());
    }

    #[test]
    fn sym() {
        let f = parser::sym('{');
        if let parser::ParseResult::Value { value: parser::ParseValue::String(str), remaining_input } = f("{abc") {
            assert_eq!(str, "{");
            assert_eq!(remaining_input, "abc");
        } else {
            panic!("fail");
        }
    }

    #[test]
    fn digit() {
        assert!(parser::digit()("1").is_value());
        assert!(parser::digit()("A").is_error());
    }

    #[test]
    fn regex() {
        assert!(parser::regex("[a-z]")("123").is_error());
        assert!(parser::regex("[a-z]")("a23").is_value());
        assert!(parser::regex("[\\s]")("\t").is_value());
    }

    #[test]
    fn repeat() {
        let f = parser::repeat(parser::digit());
        let result = f("12345abc");

        if let parser::ParseResult::Value{value: parser::ParseValue::List(list), remaining_input} = result {
            assert_eq!(remaining_input, "abc");
            assert_eq!(list.iter().map(|x| x.string().to_string() ).collect::<Vec<String>>(),
                       vec!("1", "2", "3", "4", "5"));
        } else {
            panic!();
        }
    }

    #[test]
    fn repeat_empty() {
        let f = parser::repeat(parser::digit());
        assert!(f("abc").is_empty());
    }


    #[test]
    fn repeat1() {
        let f = parser::repeat1(parser::digit());
        let result = f("12345abc");

        if let parser::ParseResult::Value{value: parser::ParseValue::List(list), remaining_input} = result {
            assert_eq!(remaining_input, "abc");
            assert_eq!(list.iter().map(|x| x.string().to_string() ).collect::<Vec<String>>(),
                       vec!("1", "2", "3", "4", "5"));
        } else {
            panic!();
        }
    }

    #[test]
    fn repeat1_errors_when_empty() {
        let f = parser::repeat1(parser::digit());
        assert!(f("abc").is_error());
    }

    #[test]
    fn one_of_test() {
        let f = one_of!(parser::digit(), parser::alphabetic());
        assert!(f("123").is_value());
        assert!(f("abc").is_value());
        assert!(f(" abc").is_error());
    }

    #[test]
    fn seq() {
        let f = seq!(parser::digit(),
                     parser::alphabetic(),
                     parser::digit());
        let result = f("1f2abc");

        if let parser::ParseResult::Value{value: parser::ParseValue::List(list), remaining_input} = result {
            assert_eq!(remaining_input, "abc");
            assert_eq!(list.iter().map(|x| x.string().to_string() ).collect::<Vec<String>>(),
                       vec!("1", "f", "2"));
        } else {
            panic!();
        }

        let errored_result = f("abc");
        assert!(f("abc").is_error());
    }

    #[test]
    fn seq_returns_error_if_one_parser_empty() {
        let f = seq!(parser::digit(),
                     parser::alphabetic(),
                     parser::digit());
        assert!(f("abc").is_error());
        assert!(f("").is_error());
    }


    #[test]
    fn boxed_closure() {
        let r = parser::boxed();

        let r_clone = Rc::clone(&r);
        let parser = seq!(move |input: &str| {
                let parser = r_clone.borrow();
                parser(input)
            }
        );

        assert!(parser("abc").is_error());

        let new_closure: Box<dyn Fn (&str) -> parser::ParseResult> = Box::new(|input: &str| { parser::ParseResult::Value{value: parser::ParseValue::String("wat".to_string()), remaining_input: &input[1..]} });
        r.replace(new_closure);
        assert!(parser("abc").is_value());
    }
}