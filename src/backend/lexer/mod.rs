mod types;
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

    #[allow(unused_assignments)]
    fn next_token(&mut self) -> Option<Token> {
        // ignore whitespaces
        if self.at().is_ascii_whitespace() {
            return None;
        }

        let default_tok = token!(self.mainptr, Unrecognized);
        let mut result = default_tok;

        // three character long tokens
        {
            if (self.mainptr + 2) > (self.data_len - 1) {
                return None;
            }

            let s = std::str::from_utf8(&self.data[self.mainptr..=self.mainptr + 2]).unwrap();

            //eprint!("({}) ", s.replace("\n", r"\n"));

            if s == ";;;" {
                self.peekptr = self.mainptr + 2;
                while {
                    let slice = std::str::from_utf8(&self.data[self.peekptr..=self.peekptr + 2]);
                    slice.unwrap()
                } != ";;;"
                {
                    self.peekptr += 1;
                }

                // skip over ;;; sequence
                self.mainptr = self.peekptr + 3;
                self.inc_by = 1;
                return None;
            };

            if result.variant != default_tok.variant {
                self.inc_by = 3; // skip over the entire token
                return Some(result);
            }
        }

        // two-character-long tokens
        {
            if (self.mainptr + 1) > (self.data_len - 1) {
                return None;
            }

            let s = std::str::from_utf8(&self.data[self.mainptr..=(self.mainptr + 1)]).unwrap();

            //eprint!("({}) ", s.replace('\n', r"\n"));

            if s == ";;" {
                self.peekptr = self.mainptr + 1;
                while self.peek() != b'\n' {
                    self.peekptr += 1;
                }

                self.mainptr = self.peekptr;
                self.inc_by = 1;
                return None;
            }

            use Operator::*;
            result = match s {
                "<-" => token!(self.mainptr, Operator, LAssign),
                "->" => token!(self.mainptr, Operator, RAssign),
                "<>" => token!(self.mainptr, Operator, Neq),
                "<<" => token!(self.mainptr, Operator, Shl),
                ">>" => token!(self.mainptr, Operator, Shr),
                ">=" => token!(self.mainptr, Operator, Geq),
                "<=" => token!(self.mainptr, Operator, Leq),
                "**" => token!(self.mainptr, Operator, Pow),
                "//" => token!(self.mainptr, Operator, FloorDiv),
                _ => default_tok,
            };

            if result.variant != default_tok.variant {
                self.inc_by = 2; // skip over the entire token
                return Some(result);
            }
        }

        // Single character tokens
        {
            use Operator::*;
            result = match self.at() {
                b'+' => token!(self.mainptr, Operator, Add),
                b'-' => token!(self.mainptr, Operator, Sub),
                b'*' => token!(self.mainptr, Operator, Mul),
                b'/' => token!(self.mainptr, Operator, Div),
                b'%' => token!(self.mainptr, Operator, Mod),
                b'=' => token!(self.mainptr, Operator, Eq),
                _ => default_tok,
            };

            if result.variant != default_tok.variant {
                return Some(result);
            }
        }

        // Idents and keywords
        {
            let mut s = String::new();
            self.peekptr = self.mainptr;

            while !self.peek().is_ascii_whitespace() {
                s.push(self.peek().into());
                if (self.peekptr + 1) < (self.data_len) {
                    self.peekptr += 1;
                } else {
                    break;
                }
            }

            use Keyword::*;
            result = match s.as_str() {
                "DECLARE" => token!(self.mainptr, Keyword, Declare),
                "IF" => token!(self.mainptr, Keyword, If),
                "ELSE" => token!(self.mainptr, Keyword, Else),
                "ENDIF" => token!(self.mainptr, Keyword, Endif),
                "THEN" => token!(self.mainptr, Keyword, Then),
                "CASE" => token!(self.mainptr, Keyword, Case),
                "OF" => token!(self.mainptr, Keyword, Of),
                "FOR" => token!(self.mainptr, Keyword, For),
                "NEXT" => token!(self.mainptr, Keyword, Next),
                "TO" => token!(self.mainptr, Keyword, To),
                "REPEAT" => token!(self.mainptr, Keyword, Repeat),
                "UNTIL" => token!(self.mainptr, Keyword, Until),
                "WHILE" => token!(self.mainptr, Keyword, While),
                "ENDWHILE" => token!(self.mainptr, Keyword, Endwhile),
                "FUNCTION" => token!(self.mainptr, Keyword, Function),
                "ENDFUNCTION" => token!(self.mainptr, Keyword, Endfunction),
                "RETURN" => token!(self.mainptr, Keyword, Return),
                "OUTPUT" => token!(self.mainptr, Keyword, Output),
                "INPUT" => token!(self.mainptr, Keyword, Input),
                "CALL" => token!(self.mainptr, Keyword, Call),
                _ => default_tok,
            };

            if result.variant != default_tok.variant {
                self.inc_by = self.peekptr - self.mainptr;
                println!("{}", self.inc_by);
                return Some(result);
            }
        }

        Some(result)
    }

    pub fn lex(&mut self) -> LexerResult<Vec<Token>> {
        let mut res = Vec::new();
        while self.mainptr < self.data_len {
            self.inc_by = 1;

            if let Some(tok) = self.next_token() {
                res.push(tok);
            }

            self.mainptr += self.inc_by;
        }
        res.push(token!(self.data_len - 1, Eof));
        Ok(res)
    }
}
