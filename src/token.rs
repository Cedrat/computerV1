use std::fmt;

pub const MULTIPLY:& str = "MULTIPLY";
pub const SIGN:& str = "SIGN";
pub const EQUALITY:& str = "EQUALITY";
pub const NUMBER:& str = "NUMBER";
pub const POWER:& str = "POWER";

pub struct Token {
    pub arg: String,
    pub type_arg: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.arg, self.type_arg)
    }
}
