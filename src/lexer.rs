use crate::token::Token;

pub struct Lexer<'a> {
    text: &'a str,
    idx: usize,
    len: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Lexer<'a> {
        if !text.is_ascii() {
            panic!("Lexer: can only lex ascii chars")
        }

        Lexer {
            text: text,
            idx: 0,
            len: text.len(),
        }
    }

    pub fn safe_new(text: &'a str) -> Result<Lexer<'a>, ()> {
        if !text.is_ascii() {
            Err(())
        } else {
            Ok(Lexer {
                text: text,
                idx: 0,
                len: text.len(),
            })
        }
    }

    /// UNCHECKED
    fn curr(&self) -> char {
        self.text.as_bytes()[self.idx] as char
    }

    fn skip_whitespace(&mut self) {
        while self.idx < self.len {
            let curr = self.curr();
            match curr {
                ' ' | '\t' | '\n' => self.idx += 1,
                _ => return,
            }
        }
    }

    fn lex_num(&mut self) -> Token {
        let start = self.idx;
        let mut is_float = false;
        while self.idx < self.len {
            let curr = self.curr();
            match curr {
                '0'..='9' => self.idx += 1,
                '.' if !is_float => {
                    self.idx += 1;
                    is_float = true;
                }
                _ => break,
            };
        }
        if is_float {
            Token::Float(start, self.idx)
        } else {
            Token::Int(start, self.idx)
        }
    }

    fn lex_keyword(&self, start: usize) -> Token {
        let end = self.idx;
        match &self.text[start..end] {
            "if" => Token::If(start),
            _ => Token::Id(start, end),
        }
    }

    pub fn lex_id(&mut self) -> Token {
        let start = self.idx;
        while self.idx < self.len {
            let curr = self.curr();
            match curr {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => self.idx += 1,
                _ => break,
            }
        }

        let tok = self.lex_keyword(start);

        return tok;
    }

    pub fn lex(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if self.idx >= self.len {
            return None;
        }

        let curr = self.curr();

        self.idx += 1;

        match curr {
            '+' => Some(Token::Plus(self.idx)),
            '-' => Some(Token::Minus(self.idx)),
            '*' => Some(Token::Star(self.idx)),
            '/' => Some(Token::Slash(self.idx)),
            '0'..='9' => {
                self.idx -= 1;
                Some(self.lex_num())
            }
            'A'..='Z' | 'a'..='z' => {
                self.idx -= 1;
                Some(self.lex_id())
            }
            _ => Some(Token::Error(self.idx)),
        }
    }
}
