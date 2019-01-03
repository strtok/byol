use regex::Regex;

pub enum ParseResult<'a> {
    Value { value: ParseValue, remaining_input: &'a str},
    Empty,
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
            ParseResult::Empty => true,
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
    Nil
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
    |_: &str| {
        ParseResult::Empty
    }
}

pub fn satisfy(predicate: impl Fn(char) -> bool) -> impl Fn(&str) -> ParseResult {
    move |input: &str| {
        if input.is_empty() {
            return ParseResult::Empty;
        }
        let c = &input[0..1];
        if predicate(c.chars().next().unwrap()) {
            ParseResult::Value {
                value: ParseValue::String(c.to_string()),
                remaining_input: &input[1..]
            }
        } else {
            ParseResult::Empty
        }
    }
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
                ParseResult::Empty => {
                    if values.is_empty() {
                        return ParseResult::Empty;
                    }
                    return ParseResult::Value {value: ParseValue::List(values), remaining_input: remaining };
                },
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
            ParseResult::Empty => {
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
                ParseResult::Empty => continue,
                result => return result
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
                ParseResult::Empty => return ParseResult::Error{text: "unexpected empty result in sequence".to_string()},
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

#[cfg(test)]
mod tests {
    use crate::parser;

    #[test]
    fn succeed() {
        assert!(parser::succeed()("123").is_empty());
    }

    #[test]
    fn satisfy() {
        let f = parser::satisfy(|_c| { true });

        if let parser::ParseResult::Value { value, remaining_input } = f("abc") {
            if let parser::ParseValue::String(str) = value {
                assert_eq!(str, "a");
                assert_eq!(remaining_input, "bc");
                return;
            }
        }

        panic!("fail");
    }

    #[test]
    fn satisfy_no_input() {
        let f = parser::satisfy(|_c: char| {true});
        assert!(f("").is_empty());
    }


    #[test]
    fn digit() {
        assert!(parser::digit()("1").is_value());
        assert!(parser::digit()("A").is_empty());
    }

    #[test]
    fn regex() {
        assert!(parser::regex("[a-z]")("123").is_empty());
        assert!(parser::regex("[a-z]")("a23").is_value());
        assert!(parser::regex("[\\s]")("\t").is_value());
    }

    #[test]
    fn repeat() {
        let f = parser::repeat(parser::digit());
        let result = f("12345abc");

        if let parser::ParseResult::Value{value, remaining_input} = result {
            assert_eq!(remaining_input, "abc");
            if let parser::ParseValue::List(list) = value {
                assert_eq!(list.iter().map(|x| x.string().to_string() ).collect::<Vec<String>>(),
                           vec!("1", "2", "3", "4", "5"));
            } else {
                panic!();
            }
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

        if let parser::ParseResult::Value{value, remaining_input} = result {
            assert_eq!(remaining_input, "abc");
            if let parser::ParseValue::List(list) = value {
                assert_eq!(list.iter().map(|x| x.string().to_string() ).collect::<Vec<String>>(),
                           vec!("1", "2", "3", "4", "5"));
            } else {
                panic!();
            }
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

        if let parser::ParseResult::Value{value, remaining_input} = result {
            assert_eq!(remaining_input, "abc");
            if let parser::ParseValue::List(list) = value {
                assert_eq!(list.iter().map(|x| x.string().to_string() ).collect::<Vec<String>>(),
                           vec!("1", "f", "2"));
            } else {
                panic!();
            }
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
}