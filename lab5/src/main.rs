mod methods;
mod input;
mod graph;

use input::{functions::INPUT_FUNCTIONS, InputError, InputVariant};
use methods::*;

use plotters::prelude::*;

use crate::{graph::Graph, input::try_get_input};

fn main() -> Result<(), InputError> {

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
    
    let lagrang = lagrang_method(&dots);
    let neutons_sep_method = neutons_sep_method(&dots);
    let neutons_fin_method = neutons_fin_method(&dots);

    let mut graph = Graph::new();

    graph.draw_function(Box::new(lagrang), range.clone(), BLACK);
    graph.draw_function(Box::new(neutons_sep_method), range.clone(), RED);
    graph.draw_function(Box::new(neutons_fin_method), range, GREEN);

    graph.draw_dots(dots, MAGENTA);

    Ok(())
}