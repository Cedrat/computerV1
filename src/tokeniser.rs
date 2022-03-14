use regex::Regex;
use crate::token::*;

fn type_token(token: &str) -> &str {

    let regex_number: regex::Regex = Regex::new("^[0-9]+.?[0-9]*+$").unwrap();
    let regex_power: regex::Regex = Regex::new("^X\\^[0-9]+$").unwrap();

    if token == "*" {
        MULTIPLY
    }
    else if token == "-" || token == "+" {
        SIGN
    }
    else if token == "=" {
        EQUALITY
    }
    else if regex_number.is_match(token) {
        NUMBER
    }
    else if regex_power.is_match(token) {
        POWER
    }
    else {
        "UNDEFINED"
    }
}

pub fn create_dict_token(tokens: Vec<String>) -> Vec<Token> {
    let mut list_tokens: Vec<Token> = Vec::new();
    for token in tokens {
        let element = Token {
            arg : token.to_string(),
            type_arg:  type_token(&token).to_string(),
        };
        list_tokens.push(element)
    }
    list_tokens
}

pub struct CheckerSyntax {
    is_equality : bool,
}
impl CheckerSyntax {
    pub fn new() -> Self {
        CheckerSyntax {
            is_equality : false,
        }
    }

    fn after_equality<'a>(&mut self, mut tokens_iter: impl Iterator<Item = &'a Token>) -> bool {
        let token = &tokens_iter.next();
        
        if token.is_none() || self.is_equality {
            return false;
        }
        let token = &token.unwrap().type_arg;
        
        self.is_equality = true;

        match token as &str {
            NUMBER=> self.after_number(tokens_iter),
            _ => false,
        }
    }

    fn after_power<'a>(&mut self, mut tokens_iter: impl Iterator<Item = &'a Token>) -> bool {
        let token = &tokens_iter.next();
        
        if token.is_none() {
            return true;
        }
        let token = &token.unwrap().type_arg;
        
        match token as &str {
            NUMBER => self.after_number(tokens_iter),
            EQUALITY => self.after_equality(tokens_iter),
            SIGN => self.after_sign(tokens_iter),
            _ => false,
        }
    }

    fn after_multiply<'a>(&mut self, mut tokens_iter: impl Iterator<Item = &'a Token>) -> bool {
        let token = &tokens_iter.next();
        
        if token.is_none() {
            return false;
        }
        let token = &token.unwrap().type_arg;
        
        match token as &str {
            POWER => self.after_power(tokens_iter),
            _ => false,
        }
    }

    fn after_sign<'a>(&mut self, mut tokens_iter: impl Iterator<Item = &'a Token>) -> bool {
        let token = &tokens_iter.next();
        
        if token.is_none() {
            return false;
        }
        let token = &token.unwrap().type_arg;

        match token as &str {
            NUMBER => self.after_number(tokens_iter),
            _ => false,
        }

    }


    fn after_number<'a>(&mut self, mut tokens_iter: impl Iterator<Item = &'a Token>) -> bool {
        let token = &tokens_iter.next();
        
        if token.is_none() {
            return true;
        }
        let token = &token.unwrap().type_arg;

        match token as &str {
            SIGN => self.after_sign(tokens_iter),
            MULTIPLY => self.after_multiply(tokens_iter),
            EQUALITY => self.after_equality(tokens_iter),
            _ => false,
        }
    }

    pub fn verify_syntax(&mut self, tokens: &[Token]) -> bool {
        let mut tokens_iter = tokens.iter();
        self.is_equality = false;

        let token = &tokens_iter.next().unwrap().type_arg;

        match token as &str { 
            NUMBER => self.after_number(tokens_iter),
            SIGN => self.after_sign(tokens_iter),
            _ => false,
        }
    }
}

impl Default for CheckerSyntax {
    fn default() -> Self {
        Self::new()
    }
}
    
#[test]
fn it_works() {
    let tokens: Vec<String> = "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0".split(' ').map(|s| s.to_string()).collect();
    let tokens = create_dict_token(tokens);
    for token in tokens {
        println!("{}", token);
    }
}