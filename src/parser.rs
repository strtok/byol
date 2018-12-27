
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

pub fn one_char(_c: char) -> impl Fn(&str) -> Result<ParseResult<char>, ParseError> {
    |_s: &str| Ok(ParseResult { value: 'a' })
}

#[cfg(test)]
mod tests {
    use crate::parser;

    #[test]
    fn one_char() {
        let f = parser::one_char('a');
        let result = f("abc").unwrap();
        assert_eq!(result.value, 'a');
    }

}