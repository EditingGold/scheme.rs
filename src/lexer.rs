use std::iter::Peekable;
use std::str::Chars;
use std::string::ParseError;
use std::cmp::Ordering;

use utils::GentleIterator;
use utils::AndOr;
use utils::fraction::Fraction;

// TODO: string.parse::<Token>();

struct Dummy;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LParen,
    RParen,
    Symbol(String),
    Integer(i64),
    Fraction(Fraction),
    Float(f64),
    Boolean(bool),
    Chr(char),
    Str(String),
    Dot,
    Ellipsis,
    Quote,
    QuasiQuote,
    UnQuote,
    UnQuoteSplicing
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Token) -> Option<Ordering> {
        use self::Token::*;
        match (self, other) {
            (Integer(x), Integer(y)) => x.partial_cmp(y),
            (Float(x), Float(y)) => x.partial_cmp(y),
            (Fraction(x), Fraction(y)) => x.partial_cmp(y),
            (Integer(x), Float(y)) => (*x as f64).partial_cmp(y),
            (Float(x), Integer(y)) => x.partial_cmp(&(*y as f64)),
            (Integer(x), Fraction(y)) => (*x as f64).partial_cmp(&(*y).into()),
            (Fraction(x), Integer(y)) => f64::from(*x).partial_cmp(&(*y as f64)),
            (Float(x), Fraction(y)) => x.partial_cmp(&(*y).into()),
            (Fraction(x), Float(y)) => f64::from(*x).partial_cmp(y),

            (Str(x), Str(y)) => x.partial_cmp(y),
            (Chr(x), Chr(y)) => x.partial_cmp(y),
            (Boolean(x), Boolean(y)) => x.partial_cmp(y),
            (Symbol(x), Symbol(y)) => x.partial_cmp(y),

            _ => None
        }
    }
}


impl Token {
    fn get(chr: char) -> Token {
        match chr {
            '(' | '['  => Token::LParen,
            ')' | ']'  => Token::RParen,
            '\'' => Token::Quote,
            '`'  => Token::QuasiQuote,
            ','  => Token::UnQuote,
            '@'  => Token::UnQuoteSplicing,
            '.'  => Token::Dot,
            x    => Token::Chr(x),
        }
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let iter = &mut input.chars().peekable();

    loop {
        if parse_whitespace(iter) || parse_comment(iter) {
            continue
        }

        // or() is eagerly evaluated
        // thats why I used or_else
        let token = parse_lparen(iter)
            .or_else(|| parse_quote(iter))
            .or_else(|| parse_unquote(iter))
            .or_else(|| parse_quasiquote(iter))
            .or_else(|| parse_rparen(iter))
            .or_else(|| parse_string(iter))
            .or_else(|| parse_hash(iter))
            .or_else(|| parse_symbol(iter));

        if let Some(x) = token {
            tokens.push(x)
        } else {
            break;
        }
    };

    tokens
}

//
// Parsers
//
fn parse_whitespace(iter: &mut Peekable<Chars>) -> bool {
    if check_chr(iter, ' ') || check_chr(iter, '\n') {
        iter.next();
        true
    } else {
        false
    }
}

fn parse_comment(iter: &mut Peekable<Chars>) -> bool {
    if check_chr(iter, ';') {
        iter.take_until(|c| *c != '\n');
        true
    } else {
        false
    }
}

fn parse_quote(iter: &mut Peekable<Chars>) -> Option<Token> {
    parse_single(iter, '\'')
}

fn parse_unquote(iter: &mut Peekable<Chars>) -> Option<Token> {
    parse_single(iter, ',')
        .and_or(parse_single(iter, '@'))
}

fn parse_quasiquote(iter: &mut Peekable<Chars>) -> Option<Token> {
    parse_single(iter, '`')
}

fn parse_lparen(iter: &mut Peekable<Chars>) -> Option<Token> {
    parse_single(iter, '(')
        .or_else(|| parse_single(iter, '['))
}

fn parse_rparen(iter: &mut Peekable<Chars>) -> Option<Token> {
    parse_single(iter, ')')
        .or_else(|| parse_single(iter, ']'))
}

fn parse_string(iter: &mut Peekable<Chars>) -> Option<Token> {
    // FIXME: check escape chars
    if !check_chr(iter, '"') {
        return None
    }

    iter.next(); // Consume the opening "
    let value = iter
        .take_until(|c| *c != '"')
        .collect();
    iter.next(); // Consume the closing "
    Some(Token::Str(value))
}

fn parse_hash(iter: &mut Peekable<Chars>) -> Option<Token> {
    if !check_chr(iter, '#') {
        return None
    }

    iter.next(); // Consume #
    match iter.next() {
        Some('t') => Some(Token::Boolean(true)),  // #t means true
        Some('f') => Some(Token::Boolean(false)), // #f means false
        Some('\\') => {
            // #\a represents char 'a'
            // #\b represents char 'b'
            // ...
            let value = iter.next()
                .expect("Expected a char, got nothing.");
            Some(Token::Chr(value))
        },
        Some('(') => {
            // Return Token::VectorOpener ?
            panic!("Not yet implemented.")
        }
        Some(c) => {
            panic!("Expected #t, #f, #(...) or #\\<char> got: #{}", c)
        },
        None => {
            panic!("Expected something , got nothing: ....")
        }
    }
}

fn parse_symbol(iter: &mut Peekable<Chars>) -> Option<Token> {
    // Check if iter is empty or not
    if !check(iter, |_| true) {
        return None
    }

    let value: String = iter
        .take_until(|c| *c != ' ' && *c != ')' && *c != ']' && *c != '\n')
        .collect();

    value.parse::<i64>().map(Token::Integer)
        .or_else(|_| value.parse::<f64>().map(Token::Float))
        .or_else(|_| value.parse::<Fraction>().map(|f| {
            if f.is_int() { Token::Integer(f.n)}
            else { Token::Fraction(f) }
        }))
        .or_else(|_| if value == "..." { Ok(Token::Ellipsis) } else { Err(Dummy) })
        .or_else(|_| if value == "." { Ok(Token::Dot) } else { Err(Dummy) })
        .or_else::<Dummy,_>(|_| Ok(Token::Symbol(value)))
        .ok()
}

/// Parse a single char and return the corresponding Token
fn parse_single(iter: &mut Peekable<Chars>, chr: char) -> Option<Token> {
    if !check_chr(iter, chr) {
        return None
    }

    iter.next();
    Some(Token::get(chr))
}

//
// Helper functions
//
fn check<F>(iter: &mut Peekable<Chars>, fun: F) -> bool
where F: Fn(char) -> bool {
    if let Some(&x) = iter.peek() {
        fun(x)
    } else {
        false
    }
}

fn check_chr(iter: &mut Peekable<Chars>, chr: char) -> bool {
    check(iter, |x| x == chr)
}
