use std::fmt::{Formatter};
use std::str::Chars;

use super::unescape::*;

use TokenKind::*;
use NumBase::*;
use LiteralKind::*;
use RawStrError::*;

/// Lexer Struct
/// Parse the whole language sourcefile
#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    pub src: String,
    source: Chars<'a>,
    prev: char,
    pub line: usize,
    pub column : usize,
}

/// The end of file.
pub const EOF: char = '\0';

/// All kinds of tokens in Fire.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
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
    /// Identifier or Keyword: "abc" or "int32"
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
    /// - (Sub)
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

impl std::fmt::Debug for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}

impl ToString for TokenKind {
    fn to_string(&self) -> String {
        match self {
            Eof => "EOF",
            Ident => "<identifier>",
            Space => "<whitespace>",
            Literal { .. } => "<literal>",
            Plus => "'+'",
            Minus => "'-'",
            Star => "'*'",
            Slash => "'/'",
            Percent => "'%'",
            Comma => "','",
            Semicolon => "';'",
            Dot => "'.'",
            Colon => "':'",
            LeftParen => "'('",
            RightParen => "')'",
            LeftBracket => "'['",
            RightBracket => "']'",
            LeftBrace => "'{'",
            RightBrace => "'}'",
            Equal => "'='",
            Exclamation => "'!'",
            Not => "'~'",
            Le => "'<'",
            Ge => "'>'",
            And => "'&'",
            Or => "'|'",
            Caret => "'^'",
            _ => ""
        }.into()
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub content: String,
    pub line: usize,
    pub column: usize
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub enum LiteralKind {
    /// 0xFFC66D, 0o1234567, 0b1010001010
    Int { base: NumBase, dangling: bool },
    /// 0.12345, 1e10, 1e-10, 1e+10
    Float { dangling: bool },
    /// 'a', '\n', '\x1b', '\u{1F600}'
    Char { unclose: bool, err: Option<UnescapeError> },
    /// "a string", "string with \n", "\x1b[33m STRING!", "\u{58a8}\u{6c34}", "中文"
    Str { unclose: bool, err: Option<UnescapeError> },
    RawStr { err: Option<RawStrError> }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub enum RawStrError {
    UncloseString,
    UncloseParen,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
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
    /// Construct a new Lexer
    pub fn new(src: &str) -> Lexer {
        Lexer { src: src.into(), source: src.chars(), prev: EOF, line: 1, column: 0 }
    }

    /// Get next char without modifying the source code.
    fn lookahead(&self) -> char {
        self.source.clone().next().unwrap_or(EOF)
    }

    /// Eat the char until the returning value of `f` being true.
    fn eat_while(&mut self, mut f: impl FnMut(char) -> bool) {
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

            'r' => {
                if self.lookahead() == '"' {
                    self.next();
                    return self.eat_raw_str();
                }
                self.ident()
            }

            c if unicode_xid::UnicodeXID::is_xid_start(c) || c == '_' => {
                self.ident()
            },

            c @ '0'..='9' => {
                let (kind, content) = self.number(c);
                let suffix = self.get_suffix();
                self.make_token(Literal { kind, suffix }, content.as_str())
            },

            '\'' => {
                self.eat_char()
            },

            '"' => {
                self.eat_str()
            },

            ',' => self.make_token(Comma, ","),
            ';' => self.make_token(Semicolon, ";"),
            '.' => self.make_token(Dot, "."),
            ':' => self.make_token(Colon, ":"),
            '(' => self.make_token(LeftParen, "("),
            ')' => self.make_token(RightParen, ")"),
            '{' => self.make_token(LeftBrace, "{"),
            '}' => self.make_token(RightBrace, "}"),
            '[' => self.make_token(LeftBracket, "["),
            ']' => self.make_token(RightBracket, "]"),
            '~' => self.make_token(Not, "~"),
            '^' => self.make_token(Caret, "^"),
            '+' => self.make_token(Plus, "+"),
            '-' => self.make_token(Minus, "-"),
            '*' => self.make_token(Star, "*"),
            '%' => self.make_token(Percent, "%"),
            '=' => self.make_token(Equal, "="),
            '!' => self.make_token(Exclamation, "!"),
            '<' => self.make_token(Le, "<"),
            '>' => self.make_token(Ge, ">"),
            '&' => self.make_token(And, "&"),
            '|' => self.make_token(Or, "|"),

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
        let mut number: String = prev.into();

        if prev == '0' {
            let (empty, content) = match self.lookahead() {
                'b' => {
                    base = Bin;
                    number.push('b');
                    self.next();
                    self.eat_digit()
                },

                'o' => {
                    base = Oct;
                    number.push('o');
                    self.next();
                    self.eat_digit()
                },

                'x' => {
                    base = Hex;
                    number.push('x');
                    self.next();
                    self.eat_hex()
                }

                '0'..='9' | '.' | 'e' | 'E' => {
                    let temp = self.eat_digit().1;
                    (false, temp)
                }

                _ => {
                    return (Int { base: Dec, dangling: false }, "0".into());
                }
            };

            if empty {
                return (Int { base, dangling: true }, content);
            }

            let t = self.lookahead();

            if t != '.' && t != 'e' && t != 'E' {
                return (Int { base, dangling: false }, number + &*content);
            }

        } else {
            number += &*self.eat_digit().1;
        }

        match self.lookahead() {
            '.' => {
                self.next();
                number.push('.');

                if self.lookahead().is_ascii_digit() {
                    number += &*self.eat_digit().1;
                    match self.lookahead() {
                        'e' | 'E' => {
                            number.push(self.next().unwrap());

                            let tuple = self.eat_exponent();

                            let dangling = tuple.0;
                            number += &*tuple.1;

                            (Float { dangling }, number)
                        },
                        _ => { (Float { dangling: false }, number) },
                    }
                } else { (Float { dangling: true }, number) }
            },

            'e' | 'E' => {
                number.push(self.next().unwrap());

                let tuple = self.eat_exponent();

                let dangling = tuple.0;
                number += &*tuple.1;

                (Float { dangling }, number)
            },

            _ => { (Int {base, dangling: false}, number) },
        }
    }

    fn eat_digit(&mut self) -> (bool, String) {
        let mut dangling = true;
        let mut content: String = "".into();

        while let c @ '0'..='9' = self.lookahead() {
            dangling = false;
            content.push(c);
            self.next();
        }

        (dangling, content)
    }

    fn eat_hex(&mut self) -> (bool, String) {
        let mut dangling = true;
        let mut content: String = "".into();
        while let c @ ('0'..='9' | 'A'..='F' | 'a'..='f') = self.lookahead() {
            dangling = false;
            content.push(c);
            self.next();
        }

        (dangling, content)
    }

    fn get_suffix(&mut self) -> String {
        if !unicode_xid::UnicodeXID::is_xid_start(self.lookahead())
            && self.lookahead() != '_' {
            return "".into();
        }

        self.next();

        let mut result: String = self.next().unwrap().into();

        while unicode_xid::UnicodeXID::is_xid_continue(self.lookahead()) {
            result.push(self.next().unwrap());
        }

        result
    }

    fn eat_exponent(&mut self) -> (bool, String) {
        let mut res: String = "".into();

        if self.lookahead() == '+' || self.lookahead() == '-' {
            res.push(self.lookahead());
            self.next();
        }

        let tuple = self.eat_digit();

        let dangling = tuple.0;
        res += &*tuple.1;

        (dangling, res)
    }

    fn eat_char(&mut self) -> Token {
        let mut unclose: bool = false;
        let mut content: String = "".into();

        while self.lookahead() != '\'' {
            if self.lookahead() == EOF {
                unclose = true;
                break;
            }

            content.push(self.next().unwrap_or(EOF));
        }

        if unclose {
            return self.make_token(
                Literal {
                    kind: Char { unclose, err: None },
                    suffix: "".into()
                },
                content.as_str());
        }

        if unescape(content.as_str()).is_err() {
            let err = unescape(content.as_str()).unwrap_err();
            let err = Some(err);

            return self.make_token(
                Literal {
                    kind: Char { unclose, err },
                    suffix: "".into()
                },
                content.as_str()
            )
        }

        self.next();

        self.make_token(
            Literal {
                kind: Char { unclose, err: None },
                suffix: "".into()
            },
            unescape(content.as_str()).unwrap().as_str()
        )
    }

    fn eat_str(&mut self) -> Token {
        let mut unclose: bool = false;
        let mut content: String = "".into();

        while self.lookahead() != '"' {
            if self.lookahead() == EOF {
                unclose = true;
                break;
            }

            content.push(self.next().unwrap_or(EOF));
        }

        if unclose {
            return self.make_token(
                Literal {
                    kind: Str { unclose, err: None },
                    suffix: "".into()
                },
                content.as_str());
        }

        if unescape(content.as_str()).is_err() {
            let err = unescape(content.as_str()).unwrap_err();
            let err = Some(err);

            return self.make_token(
                Literal {
                    kind: Str { unclose, err },
                    suffix: "".into()
                },
                content.as_str()
            )
        }

        self.next();

        self.make_token(
            Literal {
                kind: Str { unclose, err: None },
                suffix: "".into()
            },
            unescape(content.as_str()).unwrap().as_str()
        )
    }

    fn eat_raw_str(&mut self) -> Token {
        if self.lookahead() != '(' {
            let mut res: String = "".into();
            let literal =
              Literal {
                  kind: RawStr { err: Some(UncloseParen) },
                  suffix: "".into()
              };

            res.push(self.lookahead());
            self.next();

            while self.lookahead() != '"' {
                if self.lookahead() == EOF {
                    return self.make_token(literal, res.as_str());
                }

                res.push(self.lookahead());
                self.next();
            }

            self.next();
            return self.make_token(literal, res.as_str());
        }

        let mut res: String = "".into();
        self.next();

        loop {
            match self.lookahead() {
                ')' => {
                    self.next();
                    if self.lookahead() == '"' {
                        self.next();
                        break;
                    } else {
                        res.push(')');
                    }
                },

                '"' => {
                    self.next();
                    return self.make_token(
                        Literal {
                            kind: RawStr { err: Some(UncloseParen) },
                            suffix: "".into()
                        },
                        ""
                    )
                }

                EOF => {
                    return self.make_token(
                        Literal {
                            kind: RawStr { err: Some(UncloseString) },
                            suffix: "".into()
                        },
                        ""
                    )
                },

                x => {
                    res.push(x);
                }
            }

            self.next();
        }

        self.make_token(
            Literal {
                kind: RawStr { err: None },
                suffix: "".into()
            },
            res.as_str()
        )
    }
}