#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Command(String),
    Pipe,
    RedirectOut,
    RedirectAppend,
    RedirectIn,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            self.skip_whitespace();

            match self.peek() {
                Some('|') => {
                    self.next();
                    tokens.push(Token::Pipe);
                }

                Some('>') => {
                    self.next();
                    if self.peek() == Some('>') {
                        self.next();
                        tokens.push(Token::RedirectAppend);
                    } else {
                        tokens.push(Token::RedirectOut);
                    }
                }

                Some('<') => {
                    self.next();
                    tokens.push(Token::RedirectIn);
                }

                Some(_) => {
                    let word = self.read_word();
                    tokens.push(Token::Command(word));
                }

                None => break,
            }
        }

        tokens.push(Token::EOF);

        tokens
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn next(&mut self) -> Option<char> {
        let ch = self.peek();
        self.position += 1;
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }

    fn read_word(&mut self) -> String {
        let mut word = String::new();

        while let Some(ch) = self.peek() {
            match ch {
                ' ' | '\t' | '\n' | '|' | '>' | '<' => break,
                _ => {
                    word.push(ch);
                    self.next();
                }
            }
        }

        word
    }
}
