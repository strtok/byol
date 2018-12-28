
struct ParseResult<T> {
    value: T
}

#[derive(Debug)]
struct ParseError {
    text: String
}

enum ParseValue {
    String(String),
    Char(char)
}

pub fn satisfy(_predicate: impl Fn(char) -> bool) -> impl Fn(&str) -> Result<ParseResult<ParseValue>, ParseError>
{
    |_s: &str| {
        Ok(ParseResult {value: ParseValue::Char('b')})
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
            parser::ParseValue::Char(c) => assert_eq!(c, 'a'),
            _ => panic!("fail")
        }
    }

}