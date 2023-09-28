use std::collections::HashSet;

use super::Token;

pub struct Tokenizer {
    delimeters: HashSet<char>,
    min_len: usize,
}

impl Tokenizer {
    pub fn new(delimeters: HashSet<char>, min_len: usize) -> Self {
        Tokenizer {
            delimeters,
            min_len,
        }
    }

    pub fn tokenize<'a>(&'a self, text: &'a String) -> Iter<'a> {
        Iter::new(&self.delimeters, text, self.min_len)
    }
}

pub struct Iter<'a> {
    delimeters: &'a HashSet<char>,
    text: &'a str,
    min_len: usize,
    left: usize,
}

impl<'a> Iter<'a> {
    fn new(delimeters: &'a HashSet<char>, text: &'a str, min_len: usize) -> Self {
        Iter {
            delimeters,
            text,
            min_len,
            left: 0,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.left;

        for (pos, char) in self.text[self.left..].char_indices() {
            if !self.delimeters.contains(&char) {
                continue;
            }

            let right = offset + pos;
            let current = &self.text[self.left..right];
            let len = right - self.left;
            let begin = self.left;
            self.left = right + 1;

            if len < self.min_len || current.is_empty() {
                continue;
            }

            return Some(Self::Item::new(current.to_string(), begin));
        }

        None
    }
}
