use crate::token::*;
use std::fmt;
use std::ops;

pub struct Coefficient {
    pub sign: char,
    pub value : f64,
    pub power: u32,
}

impl fmt::Display for Coefficient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} * X^{}", self.sign, self.value, self.power)
    }
}



impl ops::AddAssign for Coefficient{
    fn add_assign(&mut self, _rhs: Coefficient) {
        if self.sign == _rhs.sign {
            self.value += _rhs.value;
        }
        else {
            self.value -= _rhs.value;
        }
        if self.value < 0.0 {
            self.value *= -1.0;
            self.sign = opposite_sign(self.sign, true);
        }
    }
}

fn opposite_sign(sign : char,  is_inversed : bool) -> char {
    if is_inversed {
        match sign {
            '-' => '+',
            '+' => '-',
            _ => panic!("Bad argument to opposite_sign"),
        }

    }
    else {
        sign
    }
}

fn create_coeff<'a>(tokens_iter: &mut impl Iterator<Item = &'a Token>, sign_is_inverted: bool) -> Coefficient {
    let mut coeff = Coefficient { sign: opposite_sign('+', sign_is_inverted), value: 0.0, power: 0};
    loop{
        let token = tokens_iter.next();
        if token.is_none() {
            break ;
        }
        let token = token.unwrap();
        match &token.type_arg as &str {
            SIGN => coeff.sign = opposite_sign(token.arg.chars().next().unwrap(), sign_is_inverted),
            NUMBER => coeff.value = token.arg.parse::<f64>().unwrap(),
            POWER =>  {coeff.power = token.arg.split('^').nth(1).unwrap().parse::<u32>().unwrap(); break;}
            MULTIPLY => (),
            _ => break,
        }
    }
    coeff
}


pub fn convert_in_coeff_list(tokens: &[Token]) -> Vec<Coefficient> {
    let mut list_coeff: Vec<Coefficient> = Vec::new();
    let mut tokens_iter = tokens.iter().peekable();
    let mut sign_is_inverted = false;
    let mut coeff;

    while tokens_iter.peek().is_some() {
        if tokens_iter.peek().unwrap().type_arg == EQUALITY {
            tokens_iter.next();
            sign_is_inverted = true;
            continue;
        }
        coeff = create_coeff(&mut tokens_iter, sign_is_inverted);
        
        let present = list_coeff.binary_search_by_key(&coeff.power, |a| a.power);
        match present {
            Ok(t) => {
            list_coeff[t] += coeff;
            if list_coeff[t].value == 0.0 {
                list_coeff.remove(t);
            }},
            Err(_e) => {
                if coeff.value != 0.0 {
                    list_coeff.push(coeff);
                }
            }
        }
    }
    list_coeff.sort_by_key(|k| k.power);

    list_coeff
}