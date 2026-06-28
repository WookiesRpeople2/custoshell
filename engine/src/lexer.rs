use std::{collections::HashMap, sync::LazyLock};

use crate::tok;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Command(String),
    Pipe,
    HomeSymbol,
    RedirectOut,
    RedirectAppend,
    RedirectIn,
    EOF,
}

type TokenEntry = (Token, Option<(char, Token)>);

static TOKENMAP: LazyLock<HashMap<char, TokenEntry>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert('|', tok!(Token::Pipe));
    map.insert('<', tok!(Token::RedirectIn));
    map.insert('~', tok!(Token::HomeSymbol));
    map.insert('>', tok!(Token::RedirectOut, '>' => Token::RedirectAppend));
    map
});

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
                Some(c) => {
                    if let Some((default_tok, lookahead)) = TOKENMAP.get(&c) {
                        self.next();
                        let token = match lookahead {
                            Some((next_char, alt_tok)) if self.peek() == Some(*next_char) => {
                                self.next();
                                alt_tok.clone()
                            }
                            _ => default_tok.clone(),
                        };
                        tokens.push(token);
                    } else {
                        let word = self.read_word();
                        tokens.push(Token::Command(word));
                    }
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
