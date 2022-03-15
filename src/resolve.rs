
use crate::Coefficient;

fn get_coefficients_by_power_value(coeffs : &[Coefficient], value_power: u32) -> f64 {
    let value = coeffs.binary_search_by_key(&value_power, |b| b.power);
    match value {
        Ok(val) => {
        if coeffs[val].sign == '+' {
            coeffs[val].value as f64
        }
        else {
            coeffs[val].value as f64 * (-1.0)
        }
    },
        Err(_e) => 0.0,
    }


}

fn resolve_equation_second_degree(coeffs : Vec<Coefficient>) {
    let a = get_coefficients_by_power_value(&coeffs, 2);
    let b = get_coefficients_by_power_value(&coeffs, 1);
    let c = get_coefficients_by_power_value(&coeffs, 0);

    let delta:f64 = b * b - 4.0 * a * c;
    if delta == 0.0 {
        eprintln!("a = {} b = {} c = {}", a, b,c);
        println!("The solution is {}" , (-b)/(2.0 * a));
    }
    if delta > 0.0 {
        let sol1 = (-b - delta.sqrt())/(2.0 * a);
        let sol2 = (-b + delta.sqrt())/(2.0 * a);
        println!("The two solutions are [Think to create sqrt fct]:\n Solution 1 = {}\n Solution 2 = {}", sol1, sol2);
    }
}

fn resolve_equation_first_degree(coeffs : Vec<Coefficient>) {
    let a = get_coefficients_by_power_value(&coeffs, 1);
    let b = get_coefficients_by_power_value(&coeffs, 0);

    println!("The solution is :\n {}", -b/a);
}


pub fn resolve_equation(coeffs : Vec<Coefficient>) -> bool {

    let max_power = coeffs.last().unwrap().power;
    println!("Polynomial degree: {}", max_power);
    match max_power {
        2 => resolve_equation_second_degree(coeffs),
        1 => resolve_equation_first_degree(coeffs),
        0 => println!("Wrong equation, not Solvable"),
        _ => println!("The polynomial degree is stricly greater than 2, I can't solve."),
    }
    true
}