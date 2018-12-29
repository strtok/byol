
struct ParseResult<'a> {
    value: ParseValue,
    remaining_input: &'a str
}

#[derive(Debug)]
struct ParseError {
    text: String
}

enum ParseValue {
    String(String),
    Char(char),
    Cons(Box<ParseValue>, Box<ParseValue>),
    Nil
}

pub fn satisfy(predicate: impl Fn(char) -> bool) -> impl Fn(&str) -> Result<ParseResult, ParseError> {
    move |s: &str| {
        if s.is_empty() {
            return Err(ParseError{text: String::from("no remaining input")});
        }
        let c = s.chars().next().unwrap();
        if predicate(c) {
            Ok(ParseResult {
                value: ParseValue::Char(c),
                remaining_input: &s[1..]
            })
        } else {
            Err(ParseError{text: format!("'{}' did not match predicate", c)})
        }
    }
}

pub fn digit() -> impl Fn(&str) -> Result<ParseResult, ParseError> {
    satisfy(|c: char| {
        c.is_digit(10)
    })
}

pub fn alphanumeric() -> impl Fn(&str) -> Result<ParseResult, ParseError> {
    satisfy(|c: char| {
        c.is_alphanumeric()
    })
}

pub fn alphabetic() -> impl Fn(&str) -> Result<ParseResult, ParseError> {
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
        let result = f("abc").unwrap();
        match result.value {
            parser::ParseValue::Char(c) => {
                assert_eq!(c, 'a');
                assert_eq!(result.remaining_input, "bc");
            },
            _ => panic!("fail")
        }
    }

    #[test]
    fn satisfy_no_input() {
        let f = parser::satisfy(|_c: char| {true});
        assert!(f("").is_err());
    }


    #[test]
    fn is_digit() {
        assert!(parser::digit()("1").is_ok());
        assert!(parser::digit()("A").is_err());
    }

}