use std::io::Read;
use anyhow::*;

/// Lexer Struct
/// Parse the whole language sourcefile
pub struct Lexer {
    source: String,
    line: usize,
    position : usize
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new("".into())
    }
}

impl Lexer {
    pub fn new(src: String) -> Self {
        Lexer { source : src, line: 1, position: 0 }
    }

    fn next_char(&mut self) -> Result<char> {
        if self.position == (self.source.len() - 1) {
            bail!("Error at line {} : unexpected EOF", self.line);
        }

        self.position += 1;
        let res = self.source.get(self.position..).ok_or_else( ||
            anyhow!("Error at line {} : failed to read", self.line)
        )?.chars().next().ok_or_else( ||
            anyhow!("Error at line {} : failed to read", self.line)
        )?;

        if res == '\n' {
            self.line += 1;
        }

        Ok(res)
    }

    pub fn advance_token(&mut self) -> Result<String> {

            let token = (self.source.get(self.position..).ok_or_else( ||
                anyhow!("Error at line {} , failed to read", self.line)
            )?).split_whitespace().next().ok_or_else( ||
                anyhow!("Error at line {} , failed to read",self.line)
            )?;
            self.position += (token.len() + 1);// add the length of whitespace
            if token == "\n" {
                self.line += 1;
            }
            Ok(token.into())
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
    /// Comment: "// comment" or "/* Comment */"
    Comment,
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

enum LiteralKind {
    Int { base: NumBase },
    Float { base: NumBase },

}

enum NumBase {
    Hex, Oct, Bin, Dec
}