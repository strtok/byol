
enum ParseResult<'a> {
    Value { value: ParseValue, remaining_input: &'a str},
    Empty,
    Error { text: String }
}

impl<'a> ParseResult<'a> {
    pub fn is_ok(&self) -> bool {
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

enum ParseValue {
    String(String),
    Cons(Box<ParseValue>, Box<ParseValue>),
    Nil
}

pub fn satisfy(predicate: impl Fn(char) -> bool) -> impl Fn(&str) -> ParseResult {
    move |s: &str| {
        if s.is_empty() {
            return ParseResult::Empty;
        }
        let c = &s[0..1];
        if predicate(c.chars().next().unwrap()) {
            ParseResult::Value {
                value: ParseValue::String(c.to_string()),
                remaining_input: &s[1..]
            }
        } else {
            ParseResult::Empty
        }
    }
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

#[cfg(test)]
mod tests {
    use crate::parser;

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
        assert!(parser::digit()("1").is_ok());
        assert!(parser::digit()("A").is_empty());
    }
}