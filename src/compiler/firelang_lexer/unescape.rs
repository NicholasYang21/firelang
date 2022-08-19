#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub enum EscapeError {

}

#[derive(Eq, PartialEq)]
pub(super) enum Type {
    Char,
    Str
}

pub(super) fn escape(escaping: &mut String, mode: Type) -> (char, Option<EscapeError>) {
    unimplemented!()
}