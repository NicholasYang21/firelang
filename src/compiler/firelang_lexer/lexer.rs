use std::str::Chars;
use crate::compiler::firelang_lexer::lexer::TokenKind::{BlockComment, Illegal, LineComment, Slash, Space};

/// Lexer Struct
/// Parse the whole language sourcefile
pub struct Lexer<'a> {
    source: Chars<'a>,
    line: usize,
    column : usize,
}

impl Default for Lexer<'_> {
    fn default() -> Self {
        Self::new("")
    }
}

impl Lexer {
    pub fn new(src: String) -> Self {
        Lexer { source : src, line: 1, position: 0 }
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.source.next()?;

        self.column += 1;

        if c == '\n' {
            self.line += 1;
            self.column = 0;
        }

        Some(c)
    }

    fn lookahead(&self) -> char {
        self.source.clone().next().unwrap_or('\0')
    }

    pub fn advance_token(&mut self) -> Token {
        let first = self.next_char().unwrap();
        match first {
            c if c.is_whitespace() => {
                self.whitespace()
            }

            '/' => {
                match self.lookahead() {
                    '/' => self.line_comment(),
                    '*' => self.block_comment(),
                    _   => Token { kind: Slash, content: "/".to_string() },
                }
            },



            _ => Token { kind: Illegal, content: "unexpected".to_string()},
        }
    }

    fn eat_while(&mut self, mut f: impl FnMut(char) -> bool) {
        while !f(self.lookahead()) && !self.source.as_str().is_empty() {
            self.next_char();
        }
    }

    fn whitespace(&mut self) -> Token {
        self.eat_while(|x| !x.is_whitespace());
        Token { kind: Space, content: " ".to_string() }
    }

    fn line_comment(&mut self) -> Token {
        self.eat_while(|x| x == '\n');
        Token { kind: LineComment, content: " ".to_string() }
    }

    fn block_comment(&mut self) -> Token {
        let mut d = 1usize;

        while let Some(x) = self.next_char() {
            match x {
                '/' if self.lookahead() == '*' => {
                    self.next_char();
                    d += 1;
                },
                '*' if self.lookahead() == '/' => {
                    self.next_char();
                    d -= 1;
                },
                _ => (),
            }
        }

        Token {
            kind:
                BlockComment { unexpected: d == 0 },
            content: " ".to_string()
        }
    }
}

pub struct Token {
    kind: TokenKind,
    content: String,
}

/// All kinds of tokens in Fire.
enum TokenKind {
    /// Illegal token
    Illegal,
    /// Whitespace character
    Space,
    /// "// comment"
    LineComment,
    /// "/* Comment */"
    BlockComment { unexpected: bool },
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
    /// r'\'
    /// 0xFFC66D
    /// 0b1001010
    /// 0o1234567
    /// r"A raw string\ *_\/+_)(*&^%$#@!"
    /// ```
    Literal { kind: LiteralKind, prefix_size : usize },
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

pub enum LiteralKind {
    Int { base: NumBase },
    Float { base: NumBase },
    Char,
    Str,
    Boolean,
    RawChar,
    RawStr,
}

pub enum NumBase {
    Hex, Oct, Bin, Dec
}