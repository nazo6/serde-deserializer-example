use crate::error::{Error, Result};
use crate::{err, JsonDeserializer};

mod map;
mod seq;

pub struct CommaSeparated<'a, 'de: 'a> {
    de: &'a mut JsonDeserializer<'de>,
    first: bool,
}

impl<'a, 'de> CommaSeparated<'a, 'de> {
    fn new(de: &'a mut JsonDeserializer<'de>) -> Self {
        CommaSeparated { de, first: true }
    }
}

impl<'de> JsonDeserializer<'de> {
    pub fn parse_u64(&mut self) -> u64 {
        let mut num = 0;
        while let Ok('0'..='9') = self.peek_char() {
            num *= 10;
            num += self.next_char().unwrap().to_digit(10).unwrap() as u64;
        }
        num
    }
    pub fn parse_i64(&mut self) -> Result<i64> {
        let mut num: i64 = 0;
        self.next_char().unwrap();
        if let Ok('0'..='9') = self.peek_char() {
            return Err(err!("Expected number"));
        }
        while let Ok('0'..='9') = self.peek_char() {
            num *= 10;
            num += self.next_char().unwrap().to_digit(10).unwrap() as i64;
        }
        Ok(num)
    }

    pub fn parse_string(&mut self) -> Result<&'de str> {
        if self.next_char()? != '"' {
            return Err(err!("Expected string"));
        }
        match self.input.find('"') {
            Some(len) => {
                let s = &self.input[..len];
                self.input = &self.input[len + 1..];
                Ok(s)
            }
            None => Err(err!("Unexpected EOF")),
        }
    }
}
