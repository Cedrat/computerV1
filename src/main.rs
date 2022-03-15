use std::env;

mod tokeniser;
pub mod token;
pub mod coefficient;
pub mod facto;
pub mod resolve;
use coefficient::Coefficient;

use crate::tokeniser::create_dict_token;
use crate::tokeniser::CheckerSyntax;
use crate::coefficient::convert_in_coeff_list;
use crate::resolve::resolve_equation;

fn print_factorised(coeffs : &[Coefficient]) -> bool {
    if coeffs.is_empty() {
        println!("Infinite number of solutions");
        return false;
    }
    print!("Reduced form: ");
    for coeff in coeffs {
        print!("{} ", coeff);
    }
    println!("= 0");
    true
}
fn main()  {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Incorrect nb of arguments");
        return ;
    }
    let tokens: Vec<String> = args[1].split(' ').map(|s| s.to_string()).collect();

    let list_tokens = create_dict_token(tokens);

    let mut check_syntax = CheckerSyntax::new() ;
    if !check_syntax.verify_syntax(&list_tokens) {
        eprintln!("Bad Syntax");
        return ;
    }
    let coeffs = convert_in_coeff_list(&list_tokens);
    
    if !print_factorised(&coeffs) {
        return ;
    }
    resolve_equation(coeffs);
}