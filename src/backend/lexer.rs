use super::{Token, TokenVariant};

type Byte = u8;
type LexerResult<T> = Result<T, Box<dyn std::error::Error>>; // FIXME: errors

macro_rules! token {
    ($pos:expr, $variant:ident) => {
        super::Token::new(super::TokenVariant::$variant, $pos)
    };

    ($pos:expr, $variant:ident, $($params:tt)*) => {
        super::Token::new(super::TokenVariant::$variant($($params)*), $pos)
    };
}

pub struct Lexer {
    data: Vec<Byte>,
    data_len: usize,
    mainptr: usize,
    peekptr: usize,
}

impl Lexer {
    pub fn new(data: String) -> Self {
        return Self {
            data: data.as_bytes().to_vec(),
            data_len: data.len(),
            mainptr: 0,
            peekptr: 0,
        };
    }

    fn at(&self) -> Byte {
        return self.data[self.mainptr];
    }

    fn peek(&self) -> Byte {
        return self.data[self.peekptr];
    }

    fn tokenize_at_ptr(&mut self) -> Option<LexerResult<Token>> {
        // three-character-long tokens
        {
            if (self.mainptr + 2) > (self.data_len - 1) {
                return None;
            }

            let s = std::str::from_utf8(&self.data[self.mainptr..=self.mainptr + 2]);
            let s = s.unwrap(); // TODO: remove unwra

            // eprint!("({}) ", s);

            match s {
                ";;;" => {
                    self.peekptr = self.mainptr + 2;
                    while {
                        let slice =
                            std::str::from_utf8(&self.data[self.peekptr..=self.peekptr + 2]);
                        slice.unwrap() // TODO: remove unwrap
                    } != ";;;"
                    {
                        self.peekptr += 1;
                    }

                    // skip over ;;; sequence
                    self.mainptr = self.peekptr + 3;
                    return None;
                }

                _ => {}
            }
        };

        // two-character-long tokens
        {
            if (self.mainptr + 1) > (self.data_len - 1) {
                return None;
            }

            let s = std::str::from_utf8(&self.data[self.mainptr..=(self.mainptr + 1)]);
            let s = s.unwrap(); // TODO: remove unwrap

            // eprint!("({}) ", s);

            match s {
                ";;" => {
                    self.peekptr = self.mainptr + 1;
                    while self.peek() != b'\n' {
                        self.peekptr += 1;
                    }

                    self.mainptr = self.peekptr + 1;
                    return None;
                }
                "->" => {
                    use super::Operator as O;
                    let tok = token!(self.mainptr, Operator, O::Assign);
                    return Some(Ok(tok));
                }

                _ => {}
            }
        };

        Some(Ok(token!(self.mainptr, NotImplemented)))
    }

    pub fn tokenize(&mut self) -> LexerResult<Vec<Token>> {
        let mut res = Vec::new();
        while self.mainptr < self.data_len {
            match self.tokenize_at_ptr() {
                Some(tok) => res.push(tok?),
                None => {}
            }

            self.mainptr += 1;
        }
        res.push(token!(self.data_len - 1, Eof));
        Ok(res)
    }
}
