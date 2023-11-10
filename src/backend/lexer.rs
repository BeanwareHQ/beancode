use super::Token;

type Byte = u8;
type LexerResult<T> = Result<T, Box<dyn std::error::Error>>; // FIXME: errors

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

    fn peek(&self) -> Byte {
        return self.data[self.peekptr];
    }

    fn tokenize_byte(&mut self, byte: Byte) -> LexerResult<Token> {
        match byte {
            b'-' => {
                self.peekptr = self.mainptr + 1;
                if self.peek() == b'-' {
                    self.mainptr = self.peekptr;
                }

                Ok(Token::Dummy) // FIXME
            }
            _ => Ok(Token::Dummy),
        }
    }

    pub fn tokenize(&mut self) -> LexerResult<Vec<Token>> {
        let mut res = Vec::new();
        while self.mainptr < self.data_len {
            let tok = self.tokenize_byte(self.data[self.mainptr])?;
            res.push(tok);
        }
        res.push(Token::Eof);
        Ok(res)
    }
}
