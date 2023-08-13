use crate::token;
use crate::TemplateError;
use crate::Token;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::hash::Hash;
use winnow::stream::Offset;
use winnow::stream::Stream;
use winnow::*;

pub trait Template {
    fn substitute<'input, K, V, S>(
        &'input self,
        map: &HashMap<K, V, S>,
    ) -> Result<String, TemplateError<'input>>
    where
        K: Borrow<str> + Hash + PartialEq + Eq,
        V: AsRef<str>,
        S: BuildHasher;
    fn safe_substitute<'input, K, V, S>(
        &'input self,
        map: &HashMap<K, V, S>,
    ) -> Result<String, TemplateError<'input>>
    where
        K: Borrow<str> + Hash + PartialEq + Eq,
        V: AsRef<str>,
        S: BuildHasher;
}

impl Template for str {
    fn substitute<'input, K, V, S>(
        &'input self,
        map: &HashMap<K, V, S>,
    ) -> Result<String, TemplateError<'input>>
    where
        K: Borrow<str> + Hash + PartialEq + Eq,
        V: AsRef<str>,
        S: BuildHasher,
    {
        // Work with bytes as it is more performant. We only ask for str to ensure everything is UTF-8 encoded
        let mut input = self.as_bytes();
        let mut buff = Vec::with_capacity(input.len() * 2);

        let checkpoint = input.checkpoint();
        loop {
            if input.is_empty() {
                break;
            }
            let Ok(token) = token::<()>.parse_next(&mut input) else {
            let position = input.offset_from(&checkpoint);
            return Err(TemplateError::ParserError{ position })
        };
            write_token(token, map, &mut buff)?;
        }

        let s = unsafe { String::from_utf8_unchecked(buff) };
        Ok(s)
    }

    fn safe_substitute<'input, K, V, S>(
        &'input self,
        map: &HashMap<K, V, S>,
    ) -> Result<String, TemplateError<'input>>
    where
        K: Borrow<str> + Hash + PartialEq + Eq,
        V: AsRef<str>,
        S: BuildHasher,
    {
        // Work with bytes as it is more performant. We only ask for str to ensure everything is UTF-8 encoded
        let mut input = self.as_bytes();
        let mut buff = Vec::with_capacity(input.len() * 2);

        let checkpoint = input.checkpoint();
        loop {
            if input.is_empty() {
                break;
            }
            let Ok(token) = token::<()>.parse_next(&mut input) else {
            let position = input.offset_from(&checkpoint);
            return Err(TemplateError::ParserError{ position })
        };
            safe_write_token(token, map, &mut buff)?;
        }

        let s = unsafe { String::from_utf8_unchecked(buff) };
        Ok(s)
    }
}

fn write_token<'input, K, V, S>(
    token: Token<'input>,
    map: &HashMap<K, V, S>,
    buff: &mut Vec<u8>,
) -> Result<(), TemplateError<'input>>
where
    K: Borrow<str> + Hash + PartialEq + Eq,
    V: AsRef<str>,
    S: BuildHasher,
{
    match token {
        Token::Escape => buff.push(b'$'),
        Token::SimplePlaceholder(identifier) | Token::BracketPlaceholder(identifier) => {
            // SAFETY: identifiers are always valid strings
            let identifier = unsafe { std::str::from_utf8_unchecked(identifier) };
            let Some(value) = map.get(identifier) else {
                    return Err(TemplateError::KeyError(identifier))
                };
            let value = value.as_ref().as_bytes();
            buff.extend_from_slice(value);
        }
        Token::Content(content) => buff.extend_from_slice(content),
    };
    Ok(())
}

fn safe_write_token<'input, K, V, S>(
    token: Token<'input>,
    map: &HashMap<K, V, S>,
    buff: &mut Vec<u8>,
) -> Result<(), TemplateError<'input>>
where
    K: Borrow<str> + Hash + PartialEq + Eq,
    V: AsRef<str>,
    S: BuildHasher,
{
    match token {
        Token::Escape => buff.push(b'$'),
        Token::SimplePlaceholder(identifier) => {
            // SAFETY: identifiers are always valid strings
            let identifier_str = unsafe { std::str::from_utf8_unchecked(identifier) };
            if let Some(value) = map.get(identifier_str) {
                let value = value.as_ref().as_bytes();
                buff.extend_from_slice(value);
            } else {
                buff.push(b'$');
                buff.extend_from_slice(identifier);
            }
        }
        Token::BracketPlaceholder(identifier) => {
            // SAFETY: identifiers are always valid strings
            let identifier_str = unsafe { std::str::from_utf8_unchecked(identifier) };
            if let Some(value) = map.get(identifier_str) {
                let value = value.as_ref().as_bytes();
                buff.extend_from_slice(value);
            } else {
                buff.extend_from_slice(b"${");
                buff.extend_from_slice(identifier);
                buff.push(b'}');
            }
        }
        Token::Content(content) => buff.extend_from_slice(content),
    };
    Ok(())
}
