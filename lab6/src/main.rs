mod methods;
mod input;
mod graph;
mod func;
mod runge_rule;

use func::dots_to_func;
use input::{functions::INPUT_FUNCTIONS, Conditions, InputError};
use methods::*;

use plotters::prelude::*;

use crate::{graph::Graph, input::try_get_input};

fn main() -> Result<(), InputError> {

    let input = try_get_input()?;

    let equation = input.get_func()?;
    let conditions: Conditions = input.into();


    
    let eyler_dots = runge_rule::apply_runge_rule(
            &methods::eyler_method,
            1,
            &equation,
            conditions.clone()
        );
    let func_by_eyler = dots_to_func(&eyler_dots);
    let eyler_bounds = get_bounds(&eyler_dots);

    let improved_eyler_dots = runge_rule::apply_runge_rule(
            &methods::improved_eyler_method,
            2,
            &equation,
            conditions.clone()
        );
    let func_by_improved_eyler = dots_to_func(&improved_eyler_dots);
    let improved_eyler_bounds = get_bounds(&improved_eyler_dots);

    let adams_dots = runge_rule::apply_runge_rule(
            &methods::adams_method,
            4,
            &equation,
            conditions.clone()
        );
    let func_by_adams = dots_to_func(&adams_dots);
    let adams_bounds = get_bounds(&adams_dots);

    let mut graph = Graph::new();

    graph.draw_function(Box::new(func_by_eyler), eyler_bounds.0..=eyler_bounds.1, BLACK);
    graph.draw_dots(eyler_dots, BLACK);

    graph.draw_function(Box::new(func_by_improved_eyler), improved_eyler_bounds.0..=improved_eyler_bounds.1, RED);
    graph.draw_dots(improved_eyler_dots, RED);

    graph.draw_function(Box::new(func_by_adams), adams_bounds.0..=adams_bounds.1, GREEN);
    graph.draw_dots(adams_dots, GREEN);

    Ok(())
}