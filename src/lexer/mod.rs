pub mod error;

use std::iter::Enumerate;
use std::str::Chars;
use multipeek::{IteratorExt, MultiPeek};
use crate::error::CompilerError;

pub enum Token {
    Integer(isize),
    Float(f64),
    String(String),
    Chungus,
    Rizz,
    HawkTuah,
    FanumTax,
}

struct Tokenizer<'a> {
    chars: MultiPeek<Enumerate<Chars<'a>>>,
}

impl<'a> Tokenizer<'a> {
    fn new(src: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            chars: src.chars().enumerate().multipeek()
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, CompilerError>;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.peek()? {
            _ => todo!()
        }
    }
}