
use crate::Coefficient;
use crate::sqrt::my_sqrt;

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
    println!("Delta = {}", delta);
    if delta < 0.0 {
        let  sol1p = - 1.0 * b / (2.0 * a);
        let  sol1 = (my_sqrt(-delta))/(2.0 * a);
        let  sol2 = -1.0*(my_sqrt(-delta))/(2.0 * a);

        println!("The two complexes solutions are \n Solution 1 = {}+{}i\n Solution 2 = {}{}i", sol1p, sol1, sol1p,sol2); //rework

    }
    else if delta == 0.0 {
        let sol = (-b)/(2.0 * a);

        println!("The solution is {}" , sol);
    }
    else if delta > 0.0 {
        let sol1 = (-b - my_sqrt(delta))/(2.0 * a);
        let sol2 = (-b + my_sqrt(delta))/(2.0 * a);

        println!("The two solutions are :\n Solution 1 = {}\n Solution 2 = {}", sol1, sol2);
    }
}

fn resolve_equation_first_degree(coeffs : Vec<Coefficient>) {
    let a = get_coefficients_by_power_value(&coeffs, 1);
    let b = get_coefficients_by_power_value(&coeffs, 0);

    println!("The solution is :\n {}", -b/a);
}


pub fn resolve_equation(coeffs : Vec<Coefficient>) -> bool {

    let max_power = coeffs.last().unwrap().power;
    println!("Polynomial degree : {}", max_power);
    match max_power {
        2 => resolve_equation_second_degree(coeffs),
        1 => resolve_equation_first_degree(coeffs),
        0 => println!("Wrong equation, not Solvable"),
        _ => println!("The polynomial degree is stricly greater than 2, I can't solve."),
    }
    true
}