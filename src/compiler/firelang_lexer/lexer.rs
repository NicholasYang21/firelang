use std::str::Chars;
use TokenKind::*;
use NumBase::*;
use crate::compiler::firelang_lexer::lexer::LiteralKind::{Float, Int};

/// Lexer Struct
/// Parse the whole language sourcefile
#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    pub source: Chars<'a>,
    prev: char,
    pub line: usize,
    pub column : usize,
}

pub const EOF: char = '\0';

/// All kinds of tokens in Fire.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum TokenKind {
    /// \0
    Eof,
    /// Illegal token (unknown invalid unicode)
    Illegal,
    /// An unknown prefix in the literal.
    UnknownPrefix,
    /// An invalid literal
    InvalidLiteral,
    /// Whitespace character
    Space,
    /// "// comment"
    LineComment,
    /// "/* Comment */"
    BlockComment { expected: bool },
    /// Identifier / Keyword: "abc" or "int32"
    Ident,
    /// ### Literals
    /// Literals without prefix
    /// ```text
    /// 'a'
    /// 123
    /// "A string"
    /// true
    /// ```
    /// Literals with prefix
    /// ```text
    /// 0xFFC66D
    /// 0b1001010
    /// 0o1234567
    /// ```
    Literal { kind: LiteralKind },
    /// + (Add)
    Plus,
    /// - (Minus)
    Minus,
    /// * (Mul)
    Star,
    /// / (Div)
    Slash,
    /// % (Mod)
    Percent,
    /// ,
    Comma,
    /// ;
    Semicolon,
    /// .
    Dot,
    /// :
    Colon,
    /// (
    LeftParen,
    /// )
    RightParen,
    /// {
    LeftBrace,
    /// }
    RightBrace,
    /// [
    LeftBracket,
    /// ]
    RightBracket,
    /// =
    Equal,
    /// !
    Exclamation,
    /// ~
    Not,
    /// <
    Le,
    /// >
    Ge,
    /// &
    And,
    /// |
    Or,
    /// ^
    Caret,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub content: String,
    pub line: usize,
    pub column: usize
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq,Clone)]
pub enum LiteralKind {
    Int { base: NumBase },
    Float,
    Char,
    Str,
    Boolean,
    RawChar,
    RawStr,
}

#[derive(Debug,Ord, PartialOrd, Eq, PartialEq,Clone)]
pub enum NumBase {
    Hex, Oct, Bin, Dec
}

impl Iterator for Lexer<'_> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let c = self.source.next();

        self.prev = c.unwrap();


        self.column += 1;
        if c == Some('\n') {
            self.line += 1;
            self.column = 0;
        }
        c
    }
}

impl DoubleEndedIterator for Lexer<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let c = self.source.next_back();
        self.column -= 1;
        if c == Some('\n') {
            self.line -= 1;
            self.column = self.source.as_str().lines().nth(self.line - 1).unwrap().len();
        }
        c
    }
}

impl Lexer<'_> {
    pub fn new(src: &str) -> Lexer {
        Lexer { source: src.chars(), prev: EOF, line: 1, column: 0 }
    }

    pub fn lookahead(&self) -> char {
        self.source.clone().next().unwrap_or(EOF)
    }

    pub fn eat_while(&mut self, mut f: impl FnMut(char) -> bool) {
        while !f(self.lookahead()) && !self.source.as_str().is_empty() {
            self.next();
        }
    }

    fn before(&self) -> char {
        self.prev
    }

    pub fn advance_token(&mut self) -> Token {
        if self.source.as_str().is_empty() {
            return self.make_token(Eof, "End of file.");
        }
        let first = self.next().unwrap();
        match first {
            '\0' => { self.make_token(Eof, "End of file.") }

            c if c.is_whitespace() => {
                self.whitespace()
            }

            '/' => {
                match self.lookahead() {
                    '/' => self.line_comment(),
                    '*' => self.block_comment(),
                    _ => self.make_token(Slash, "/")
                }
            },

            c if unicode_xid::UnicodeXID::is_xid_start(c) || c == '_' => {
                self.ident()
            },

            c if c.is_digit(10) => {
                if c == '0' && !self.lookahead().is_numeric() {
                    self.num_with_prefix_or_unknown_prefix()
                } else {
                    self.next_back();
                    self.make_number()
                }
            },

            _ => self.make_token(Illegal, "unexpected"),
        }
    }

    #[inline]
    fn make_token(&self, kind: TokenKind, content: &str) -> Token {
        Token {
            kind, content: content.into(),
            line: self.line, column: self.column
        }
    }

    fn whitespace(&mut self) -> Token {
        self.eat_while(|x| !x.is_whitespace());
        Token {
            kind: Space,
            content: " ".to_string(),
            line: self.line,
            column: self.column
        }
    }

    fn line_comment(&mut self) -> Token {
        self.eat_while(|x| x == '\n');
        Token {
            kind: LineComment,
            content: " ".to_string(),
            line: self.line,
            column: self.column
        }
    }

    fn block_comment(&mut self) -> Token {
        let mut d = 1usize;

        while let Some(x) = self.next() {
            match x {
                '/' if self.lookahead() == '*' => {
                    self.next();
                    d += 1;
                },

                '*' if self.lookahead() == '/' => {
                    self.next(); self.next();
                    d -= 1;
                },

                _ => continue,
            }
        }
        self.make_token(BlockComment { expected: d == 0 }, " ")
    }

    fn ident(&mut self) -> Token {
        let mut c : String = self.before().into();

        while unicode_xid::UnicodeXID::is_xid_continue(self.lookahead()) {
            // unexpected EOF
            c.push(self.next()
                .unwrap_or_else(||
                    panic!("Error : Unexpected EOF at line {} column {}", self.line, self.column)));
        }

        self.make_token(Ident, c.as_str())
    }

    fn num_with_prefix_or_unknown_prefix(&mut self) -> Token {
        let mut content : String = "".into();

        match self.lookahead() {
            'x' => {
            },

            'b' => {
            },

            'o' => {
            },

            '.' => {
            },
            _   => { self.make_token(UnknownPrefix, "unexpected") },
        }
    }

    fn make_number(&mut self) -> Token {
        let mut res : String = "".into();
        let mut dots: bool = false;
        let mut kind: TokenKind = Literal { kind: Int { base: Dec } };

        loop {
            match self.next() {
                Some(c) if c.is_digit(10) => {
                    res.push(c);
                },

                Some('.') => {
                    if !dots {
                        dots = true;
                        kind = Literal { kind: Float };

                        continue;
                    }
                    break;
                },

                None => {
                    break;
                }

                _ => {
                    self.next_back();
                    break;
                }
            }
        }

        self.make_token(kind, res.as_str())
    }
}