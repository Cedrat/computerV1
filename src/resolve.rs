
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
        let  mut sol1p = - 1.0 * b / (2.0 * a);
        let  mut sol1 = (my_sqrt(-delta))/(2.0 * a);
        let  mut sol2 = -1.0*(my_sqrt(-delta))/(2.0 * a);
        
        if sol1p == 0.0 {
            sol1p = 0.0;
        }

        if sol1 < sol2 {
            std::mem::swap(&mut sol1, &mut sol2);
        }

        println!("The two complexes solutions are \n Solution 1 = {}+{}i\n Solution 2 = {}{}i", sol1p, sol1, sol1p,sol2);

    }
    else if delta == 0.0 {
        let mut sol = (-b)/(2.0 * a);

        if sol == 0.0 {
            sol = 0.0;
        }

        println!("The solution is {}" , sol);
    }
    else if delta > 0.0 {
        let mut sol1 = (-b - my_sqrt(delta))/(2.0 * a);
        let mut sol2 = (-b + my_sqrt(delta))/(2.0 * a);
        if sol1 == 0.0 {
            sol1 = 0.0;
        }
        if sol2 == 0.0 {
            sol2 = 0.0;
        }

        println!("The two solutions are :\n Solution 1 = {}\n Solution 2 = {}", sol1, sol2);
    }
}

fn resolve_equation_first_degree(coeffs : Vec<Coefficient>) {
    let a = get_coefficients_by_power_value(&coeffs, 1);
    let b = get_coefficients_by_power_value(&coeffs, 0);
    let mut sol = -b/a;

    if sol == 0.0 {
        sol = 0.0;
    }
    println!("The solution is :\n {}", sol);
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