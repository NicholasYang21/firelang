use std::str::Chars;
use TokenKind::*;
use NumBase::*;
use LiteralKind::*;

/// Lexer Struct
/// Parse the whole language sourcefile
#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    pub source: Chars<'a>,
    prev: char,
    pub line: usize,
    pub column : usize,
}

/// The end of file.
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
    Literal { kind: LiteralKind, suffix: String },
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
    Int { base: NumBase, dangling: bool },
    Float { base: NumBase, dangling: bool },
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

impl Lexer<'_> {
    pub fn new(src: &str) -> Lexer {
        Lexer { source: src.chars(), prev: EOF, line: 1, column: 0 }
    }

    /// Get next char without modifying the source code.
    pub fn lookahead(&self) -> char {
        self.source.clone().next().unwrap_or(EOF)
    }

    /// Eat the char until the return value of `f` being true.
    pub fn eat_while(&mut self, mut f: impl FnMut(char) -> bool) {
        while !f(self.lookahead()) && !self.source.as_str().is_empty() {
            self.next();
        }
    }

    /// Get the previous char.
    fn before(&self) -> char {
        self.prev
    }

    /// Generate a token.
    pub fn next_token(&mut self) -> Token {
        if self.source.as_str().is_empty() {
            return self.make_token(Eof, "End of file.");
        }

        let first = self.next().unwrap();

        match first {
            EOF => { self.make_token(Eof, "End of file.") }

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

            c @ '0'..='9' => {
                let (kind, content) = self.number(c);
                let suffix = self.get_suffix();
                self.make_token(Literal { kind, suffix }, content.as_str())
            }

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
        self.make_token(Space, " ")
    }

    fn line_comment(&mut self) -> Token {
        let (ln, col) = (self.line - 1, self.column - 1);

        self.eat_while(|x| x == '\n');
        Token {
            kind: LineComment,
            content: "".into(),
            line: ln,
            column: col,
        }
    }

    fn block_comment(&mut self) -> Token {
        self.next();

        let mut d = 1usize;

        while let Some(x) = self.next() {
            match x {
                '/' if self.lookahead() == '*' => {
                    d += 1;
                },

                '*' if self.lookahead() == '/' => {
                    d -= 1;
                    self.next();
                    if d == 0 { break; }
                },

                _ => (),
            }
        }

        self.make_token(BlockComment { expected: d == 0 }, "")
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

    fn number(&mut self, prev: char) -> (LiteralKind, String) {
        let mut base: NumBase = Dec;

        if prev == '0' {
            let (empty, content) = match self.lookahead() {
                'b' => {
                    base = Bin;
                    self.next();
                    self.eat_digit()
                },

                'o' => {
                    base = Oct;
                    self.next();
                    self.eat_digit()
                },

                'x' => {
                    base = Hex;
                    self.next();
                    self.eat_hex()
                }

                '0'..='9' | '.' | 'e' | 'E' => {
                    self.eat_digit()
                }

                _ => {
                    return (Int { base: Dec, dangling: false }, "0".into());
                }
            };

            if empty {
                return (Int { base, dangling: true }, content)
            }
        }

        (Int { base: Dec, dangling: false }, "0".into())
    }

    fn eat_digit(&mut self) -> (bool, String) {
        let mut res = true;
        let mut content: String = "".into();

        while let c @ '0'..='9' = self.lookahead() {
            res = false;
            content.push(c);
            self.next();
        }

        (res, content)
    }

    fn eat_hex(&mut self) -> (bool, String) {
        let mut res = true;
        let mut content: String = "".into();

        while let c @ ('0'..='9' | 'A'..='F' | 'a'..='f') = self.lookahead() {
            res = false;
            content.push(c);
            self.next();
        }

        (res, content)
    }

    fn get_suffix(&mut self) -> String {
        "".into()
    }
}