use std::convert::Into;
use std::fmt::{Display, Formatter};
use std::ops::Range;

pub struct FireError {
    messages: Vec<String>,
}

pub enum Type {
    Warning,
    Error,
}

impl Display for FireError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut message: String = "".into();
        for s in self.messages {
            message.push_str((s + "\n").as_str());
        }

        write!(f, "{}", message)
    }
}

impl FireError {
    fn new(ty: Type) -> FireError {
        let mut message: String = "".into();

        match ty {
            Type::Error => message += "Error",
            Type::Warning => message += "Warning",
        }

        FireError {
            messages: vec![message],
        }
    }

    fn stress(line: u32, range: Range<u32>) -> &mut FireError {}
}
