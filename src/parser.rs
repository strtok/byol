
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

pub fn satisfy(_predicate: impl Fn(char) -> bool) -> impl Fn(&str) -> Result<ParseResult, ParseError> {
    |s: &str| {
        match s.len() {
            0 => Err(ParseError{text: String::from("no remaining input")}),
            _ => Ok(ParseResult {
                value: ParseValue::Char(s.chars().next().unwrap()),
                remaining_input: &s[1..]
            })

        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser;

    #[test]
    fn satisfy() {
        let f = parser::satisfy(|_c: char| {true});
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

}