mod types;

use std::num::{IntErrorKind, ParseIntError};

use crate::backend::types::*;
pub use types::*;

type Byte = u8;
type LexerResult<T> = Result<T, Box<dyn std::error::Error>>; // FIXME: errors

macro_rules! token {
    ($pos:expr, $variant:ident) => {
        Token::new(TokenVariant::$variant, $pos)
    };

    ($pos:expr, $variant:ident, $($params:tt)*) => {
        Token::new(TokenVariant::$variant($($params)*), $pos)
    };

    ($pos:expr, $variant:ident, $param:literal) => {
        Token::new(TokenVariant::$variant($param), $pos)
    }
}

pub struct Lexer {
    data: Vec<Byte>,
    data_len: usize,
    mainptr: usize,
    peekptr: usize,
    inc_by: usize,
}

impl Lexer {
    pub fn new(data: String) -> Self {
        return Self {
            data: data.as_bytes().to_vec(),
            data_len: data.len(),
            mainptr: 0,
            peekptr: 0,
            inc_by: 0,
        };
    }

    fn at(&self) -> Byte {
        self.data[self.mainptr]
    }

    fn peek(&self) -> Byte {
        self.data[self.peekptr]
    }

    fn skip_comments(&mut self) -> bool {
        let s = std::str::from_utf8(&self.data[self.mainptr..=self.mainptr + 2]).unwrap();

        // return if three characters ahead of the curr pos
        // is out of range
        if s == ";;;" {
            self.peekptr = self.mainptr + 2;
            while self.peekptr < self.data_len - 2 && {
                let slice = std::str::from_utf8(&self.data[self.peekptr..=self.peekptr + 2]);
                slice.unwrap()
            } != ";;;"
            {
                self.peekptr += 1;
            }

            // skip over end ;;; sequence
            self.mainptr = self.peekptr + 3;
            return true;
        };

        let s = std::str::from_utf8(&self.data[self.mainptr..=self.mainptr + 1]).unwrap();
        if s == ";;" {
            self.peekptr = self.mainptr + 1;
            while self.peekptr < self.data_len && self.peek() != b'\n' {
                self.peekptr += 1;
            }

            self.mainptr = self.peekptr;
            return true;
        }

        false
    }

    fn next_doublechar_token(&self, s: &str) -> Option<Token> {
        use Operator::*;
        match s {
            "<-" => Some(token!(self.mainptr, Operator, LAssign)),
            "->" => Some(token!(self.mainptr, Operator, RAssign)),
            "<>" => Some(token!(self.mainptr, Operator, Neq)),
            "<<" => Some(token!(self.mainptr, Operator, Shl)),
            ">>" => Some(token!(self.mainptr, Operator, Shr)),
            ">=" => Some(token!(self.mainptr, Operator, Geq)),
            "<=" => Some(token!(self.mainptr, Operator, Leq)),
            "**" => Some(token!(self.mainptr, Operator, Pow)),
            "//" => Some(token!(self.mainptr, Operator, FloorDiv)),
            _ => None, // keep it the same
        }
    }

    fn next_singlechar_token(&self, ch: u8) -> Option<Token> {
        use Operator::*;
        match ch {
            b'+' => Some(token!(self.mainptr, Operator, Add)),
            b'-' => Some(token!(self.mainptr, Operator, Sub)),
            b'*' => Some(token!(self.mainptr, Operator, Mul)),
            b'/' => Some(token!(self.mainptr, Operator, Div)),
            b'%' => Some(token!(self.mainptr, Operator, Mod)),
            b'=' => Some(token!(self.mainptr, Operator, Eq)),
            b'>' => Some(token!(self.mainptr, Operator, Gt)),
            b'<' => Some(token!(self.mainptr, Operator, Gt)),
            _ => None,
        }
    }

    fn next_keyword_token(&self, s: &str) -> Option<Token> {
        use Keyword::*;
        match s {
            "DECLARE" => Some(token!(self.mainptr, Keyword, Declare)),
            "IF" => Some(token!(self.mainptr, Keyword, If)),
            "ELSE" => Some(token!(self.mainptr, Keyword, Else)),
            "ENDIF" => Some(token!(self.mainptr, Keyword, Endif)),
            "THEN" => Some(token!(self.mainptr, Keyword, Then)),
            "CASE" => Some(token!(self.mainptr, Keyword, Case)),
            "OF" => Some(token!(self.mainptr, Keyword, Of)),
            "FOR" => Some(token!(self.mainptr, Keyword, For)),
            "NEXT" => Some(token!(self.mainptr, Keyword, Next)),
            "TO" => Some(token!(self.mainptr, Keyword, To)),
            "REPEAT" => Some(token!(self.mainptr, Keyword, Repeat)),
            "UNTIL" => Some(token!(self.mainptr, Keyword, Until)),
            "WHILE" => Some(token!(self.mainptr, Keyword, While)),
            "ENDWHILE" => Some(token!(self.mainptr, Keyword, Endwhile)),
            "FUNCTION" => Some(token!(self.mainptr, Keyword, Function)),
            "ENDFUNCTION" => Some(token!(self.mainptr, Keyword, Endfunction)),
            "RETURN" => Some(token!(self.mainptr, Keyword, Return)),
            "OUTPUT" => Some(token!(self.mainptr, Keyword, Output)),
            "INPUT" => Some(token!(self.mainptr, Keyword, Input)),
            "CALL" => Some(token!(self.mainptr, Keyword, Call)),
            "TRUE" => Some(token!(self.mainptr, Keyword, True)),
            "FALSE" => Some(token!(self.mainptr, Keyword, False)),
            _ => None,
        }
    }

    fn next_literal_token(&self, s: &str) -> Option<Token> {
        //let s_slice = std::str::from_utf8(slice).unwrap();
        let bytes = s.as_bytes();

        if s.is_empty() {
            return None;
        }

        // Number literals
        if bytes[0].is_ascii_digit() || bytes[0] == b'-' {
            let mut num_buf = String::new();
            let mut seen_dp = false;

            for chr in bytes {
                if chr == &b'_' {
                    continue;
                }

                if chr == &b'.' {
                    if !seen_dp {
                        seen_dp = true;
                    } else {
                        return Some(token!(self.mainptr, Invalid));
                    }
                }

                num_buf.push(char::from_u32(*chr as u32).unwrap());
            }

            if seen_dp {
                let num = num_buf.parse::<f64>().unwrap();
                let result = BFloat::new(num);
                return Some(token!(self.mainptr, Literal, BObject::Float(result)));
            } else {
                return Some(token!(
                    self.mainptr,
                    Literal,
                    BObject::Integer(BInteger::parse_from_string(&num_buf))
                ));
            }
        };

        // String literals

        None
    }

    fn next_word(&mut self) -> String {
        let mut s = String::new();
        self.peekptr = self.mainptr;

        while {
            let chr = self.peek();
            !chr.is_ascii_whitespace()
        } {
            s.push(self.peek().into());
            if (self.peekptr + 1) < (self.data_len) {
                self.peekptr += 1;
            } else {
                self.peekptr += 1;
                break;
            }
        }

        s
    }

    fn next_token(&mut self) -> Option<Token> {
        // ignore whitespaces
        if self.at().is_ascii_whitespace() {
            return None;
        }

        // Single character tokens
        if let Some(tok) = self.next_singlechar_token(self.at()) {
            return Some(tok);
        }

        // return if 3 chars ahead is out of range
        if (self.mainptr + 3) > (self.data_len - 1) {
            return None;
        }

        if self.skip_comments() {
            return None;
        }

        // return if 2 chars ahead is out of range
        if (self.mainptr + 2) > (self.data_len - 1) {
            return None;
        }

        let s = std::str::from_utf8(&self.data[self.mainptr..=self.mainptr + 1]).unwrap();
        if let Some(tok) = self.next_doublechar_token(s) {
            self.inc_by = 2;
            return Some(tok);
        }

        // Idents, keywords and literals
        let s = self.next_word();
        self.inc_by = self.peekptr - self.mainptr; // skip over the loop after the loop

        if let Some(tok) = self.next_keyword_token(&s) {
            return Some(tok);
        };

        if let Some(tok) = self.next_literal_token(&s) {
            return Some(tok);
        }

        Some(token!(self.mainptr, Ident, s))
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut res = Vec::new();
        while self.mainptr < self.data_len {
            self.inc_by = 1;

            if let Some(tok) = self.next_token() {
                res.push(tok);
            }

            self.mainptr += self.inc_by;
        }
        res.push(token!(self.data_len - 1, Eof));
        res
    }
}
