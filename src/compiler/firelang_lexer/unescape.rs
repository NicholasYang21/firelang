use UnescapeError::*;
use std::collections::VecDeque;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub enum UnescapeError {
    OnlyOneSlashError,
    IllegalEscape,
    EmptyUnicode,
    UnclosedUnicode,
    IllegalUnicode,
    TooLongUnicode,
    ValueOutOfUnicode,
    LoneSurrogate,
    InvalidCharInUnicode,
    TooShortEscape,
    InvalidCharInHex,
    ValueOutOfHex,
}

pub fn unescape(input: &str) -> Result<String, UnescapeError> {
    let mut que = input.chars().collect::<VecDeque<char>>();
    let mut res: String = "".into();

    if input.is_empty() { return Ok(res); }

    while let Some(c) = que.pop_front() {
        if c != '\\' {
            res.push(c);
            continue;
        }

        match que.pop_front() {
            None => return Err(OnlyOneSlashError),
            Some('b') => res.push('\u{0008}'),
            Some('r') => res.push('\r'),
            Some('n') => res.push('\n'),
            Some('t') => res.push('\t'),
            Some('\'') => res.push('\''),
            Some('\\') => res.push('\\'),
            Some('u') => {
                if que.is_empty() {
                    return Err(UnclosedUnicode);
                }

                if que.pop_front().unwrap() != '{' {
                    return Err(IllegalUnicode);
                }

                let mut digits: usize = 0;
                let mut value: u32 = 0;

                loop {
                    match que.pop_front() {
                        None => return Err(UnclosedUnicode),

                        Some(x) if x.is_ascii_hexdigit() => {
                            if digits == 6 {
                                return Err(TooLongUnicode);
                            }

                            digits += 1;

                            value *= 16;
                            value += x.to_digit(16).unwrap();
                        },

                        Some('}') => {
                            if value > 0x10FFFF {
                                return Err(ValueOutOfUnicode);
                            }

                            if char::from_u32(value).is_none() {
                                return Err(LoneSurrogate);
                            }

                            res.push(char::from_u32(value).unwrap());
                            break;
                        },

                        _ => { return Err(InvalidCharInUnicode) },
                    }
                }
            },

            Some('x') => {
                let high = que.pop_front().ok_or(TooShortEscape)?;
                let high = high.to_digit(16).ok_or(InvalidCharInHex)?;

                let low = que.pop_front().ok_or(TooShortEscape)?;
                let low = low.to_digit(16).ok_or(InvalidCharInHex)?;

                let val = high * 16 + low;

                if val > 0x7f {
                    return Err(ValueOutOfHex);
                }

                res.push(val as u8 as char);
            }

            _ => return Err(IllegalEscape),
        }
    }

    Ok(res)
}