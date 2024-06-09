mod methods;
mod input;
mod graph;

use input::{foo, functions::INPUT_FUNCTIONS, Dots, InputError, InputVariant};
use methods::*;

use plotters::prelude::*;

use crate::{graph::Graph, input::try_get_input};

fn main() -> Result<(), InputError> {

    foo()?;

    let input = try_get_input()?;

    let bounds = input.get_bounds().ok_or(InputError::CannotGetBounds)?;
    let range = bounds.0..=bounds.1;

    let dots = match input.variant {
        InputVariant::Dots(dots) => dots,
        InputVariant::Function{func_index, a, b, n} => {
            let function = &INPUT_FUNCTIONS.get(func_index-1)
                                                                    .ok_or(InputError::FunctionIndexError(func_index))?
                                                                    .function;
            
            let h = (b - a) / (n - 1) as f64;

            (0..n)
                .map(|x| x as f64 * h + a)
                .map(|x| (x, function(x)))
                .collect()
        }
    };
    
    let linear = linear_method(&dots);
    let quad = quad_method(&dots);
    let cubic = cubic_method(&dots);
    let exponential = exponential_method(&dots);
    let logarifmic = logarifmic_method(&dots);
    let power = power_method(&dots);

    print_aprox_info(&dots, &linear, "linear");
    print_aprox_info(&dots, &quad, "quad");
    print_aprox_info(&dots, &cubic, "cubic");
    print_aprox_info(&dots, &exponential, "exponential");
    print_aprox_info(&dots, &logarifmic, "logarifmic");
    print_aprox_info(&dots, &power, "power");


    let mut graph = Graph::new();

    graph.draw_function(Box::new(linear), range.clone(), BLACK);
    graph.draw_function(Box::new(quad), range.clone(), CYAN);
    graph.draw_function(Box::new(cubic), range.clone(), MAGENTA);
    graph.draw_function(Box::new(exponential), range.clone(), RED);
    graph.draw_function(Box::new(logarifmic), range.clone(), GREEN);
    graph.draw_function(Box::new(power), range, BLUE);


    graph.draw_dots(dots, MAGENTA);

    Ok(())
}



fn get_epsilon(dots: &Dots, f: &dyn Fn(f64) -> f64) -> Vec<(f64, f64)> {
    dots.iter()
        .map(|(x, y)| {
            let f_x = f(*x);
            (f_x, (y - f_x).powi(2))
        }).collect()
}

fn epsilon_sum(epsilons: &Vec<(f64, f64)>) -> f64 {
    epsilons.iter()
        .map(|(_, e)| e)
        .sum()
}

fn print_aprox_info(dots: &Dots, f: &dyn Fn(f64) -> f64, name: &str) {
    println!("Function: {name}");
    let epsilons = get_epsilon(dots, f);
    print!("fi_x | ");
    epsilons.iter().for_each(|(y, _)| print!("{y:.4} "));
    print!("\ne_i  | ");
    epsilons.iter().for_each(|(_, e)| print!("{e:.4} "));

    println!("\nE = {}\n", epsilon_sum(&epsilons));
}