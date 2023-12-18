mod error;
mod parse;

use serde::{
    de::{self, Visitor},
    forward_to_deserialize_any, Deserialize,
};

use error::{Error, Result};

pub struct JsonDeserializer<'de> {
    input: &'de str,
}

#[macro_export]
macro_rules! err {
    ($msg:expr) => {
        Error::Message($msg.to_string())
    };
}

impl<'de> JsonDeserializer<'de> {
    // Look at the first character in the input without consuming it.
    fn peek_char(&mut self) -> Result<char> {
        self.input.chars().next().ok_or(err!("Unexpected EOF"))
    }

    // Consume the first character in the input.
    fn next_char(&mut self) -> Result<char> {
        let ch = self.peek_char()?;
        self.input = &self.input[ch.len_utf8()..];
        Ok(ch)
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut JsonDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        while let ' ' | '\n' | '\t' | '\r' = self.peek_char()? {
            self.next_char()?;
        }

        match self.peek_char()? {
            'n' => visitor.visit_unit(), // null (Unit)
            't' => {
                for c in ['t', 'r', 'u', 'e'] {
                    if c != self.next_char()? {
                        return Err(err!("Expected true"));
                    }
                }
                visitor.visit_bool(true)
            }
            'f' => {
                for c in ['f', 'a', 'l', 's', 'e'] {
                    if c != self.next_char()? {
                        return Err(err!("Expected false"));
                    }
                }
                visitor.visit_bool(false)
            }
            // 後で実装します
            '"' => visitor.visit_borrowed_str(self.parse_string()?), // string
            '0'..='9' => visitor.visit_u64(self.parse_u64()),        // unsigned number
            '-' => visitor.visit_u64(self.parse_u64()),              // signed number
            '[' => self.parse_deserialize_seq(visitor),
            '{' => self.parse_deserialize_map(visitor),
            _ => Err(err!("Unexpected character")),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

pub fn from_str<'de, T: Deserialize<'de>>(input: &'de str) -> Result<T> {
    let mut deserializer = JsonDeserializer { input };
    T::deserialize(&mut deserializer)
}
