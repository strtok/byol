extern crate pretty_env_logger;
extern crate regex;
#[macro_use] extern crate log;

#[macro_use] mod parser;

use rustyline::error::ReadlineError;
use crate::parser::*;

fn main() {
    pretty_env_logger::init();
    debug!("starting");

    let ws = || {
        discard(repeat1(regex("[\\s]")))
    };

    let opt_ws = || {
        optional(ws())
    };

    let number = || {
        flat_string(seq!(optional(ch('-')), flat_string(repeat1(digit()))))
    };

    let operator = || {
        regex("[+/*-]")
    };

    let mut expr = Parser::new();
    let inner_expr = Box::new(
        one_of!(number(),
                seq!(ch('('),
                     opt_ws(),
                     operator(),
                     opt_ws(),
                     ch(')')),
                seq!(ch('('),
                     opt_ws(),
                     operator(),
                     repeat1(last_of(seq!(ws(), expr.delegate()))),
                     opt_ws(),
                     ch(')')))
    );

    expr.update(inner_expr);

    let parser = expr.delegate();

    let mut rl = rustyline::Editor::<()>::new();
    loop {
        let readline = rl.readline("lisp> ");
        match readline {
            Ok(line) => {
                let result = parser(&line);
                match &result {
                    ParseResult::Value(value, remaining_input) => {
//                        println!("{:#?}", value);
                        println!("{}", eval(&value));
                    }
                    ParseResult::Error(text) => {
                        println!("error: {}", text);
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                break;
            },
            Err(ReadlineError::Eof) => {
                break;
            },
            Err(err) => {
                error!("error: {:?}", err);
                break;
            }
        }
    }
}

fn eval(expr: &ParseValue) -> i64 {
    match expr {
        ParseValue::String(number) => {
            return number.parse::<i64>().unwrap();
        },
        ParseValue::List(list) if 3 == list.len() => {
            let op = list[1].string();
            match op {
                "+" => 0,
                "*" => 1,
                _ => panic!("op requires arguments")
           }
        },
        ParseValue::List(list) => {
            let op = list[1].string();
            let operands = list[2].list();
            let mut x = eval(&operands[0]);
            let mut i = 1;
            while i < operands.len() {
                match op {
                    "+" => {
                        x = x + eval(&operands[i]);
                    },
                    "*" => {
                        x = x * eval(&operands[i]);
                    },
                    "-" => {
                        x = x - eval(&operands[i]);
                    },
                    "/" => {
                        x = x / eval(&operands[i]);
                    },
                    _ => panic!("unknown op")
                }
                i = i + 1;
            }
            x
        },
        result => panic!()
    }
}

#[cfg(test)]
mod main_tests {
    use crate::parser;

    #[test]
    fn negative_numbers() {
        assert_eq!(-11, super::eval(&parser::ParseValue::String("-11".to_string())));
        assert_eq!(-1, super::eval(&parser::ParseValue::String("-1".to_string())));
        assert_eq!(0, super::eval(&parser::ParseValue::String("-0".to_string())));
    }

    #[test]
    fn sum_empty() {
        assert_eq!(0,
            super::eval(
                &parser::ParseValue::List(vec![
                    parser::ParseValue::String("(".to_string()),
                    parser::ParseValue::String("+".to_string()),
                    parser::ParseValue::String(")".to_string()),
                ])
            )
        );
    }

    #[test]
    fn mul_empty() {
        assert_eq!(1,
            super::eval(
                &parser::ParseValue::List(vec![
                    parser::ParseValue::String("(".to_string()),
                    parser::ParseValue::String("*".to_string()),
                    parser::ParseValue::String(")".to_string()),
                ])
            )
        );
    }
}