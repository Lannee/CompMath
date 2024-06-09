mod methods;
mod input;
mod solutions;

use std::ops::Deref;

use input::InputError;
use methods::*;

use plotters::prelude::*;

use crate::input::functions::INPUT_SYSTEMS;
use crate::input::try_get_input;
use crate::solutions::Conditions;
use input::functions::INPUT_FUNCTIONS;

fn main() -> Result<(), InputError> {

    let input = try_get_input()?;

    let func = INPUT_FUNCTIONS[input.func_index - 1].function.clone();
    let system = &INPUT_SYSTEMS[input.system_index - 1];

    draw_function(&input.conditions, func.get_n_derivative(0).unwrap());

    println!("Поиск нулей функций:");
    println!("Метод половинного деления: {:?}", half_div_method(&func, &input.conditions));
    println!("Метод ньютона: {:?}", neutons_method(&func, &input.conditions));
    println!("Метод простых итераций: {:?}", simple_iterations_method(&func, &input.conditions));

    println!("\nПоиск решения системы уравнений:");
    println!("Метод простой итерации: {:?}", system_symple_iterations_method(system, &input.conditions));

    Ok(())
}

fn draw_function<F : Fn(f64) -> f64>(conditions: &Conditions, f: F) {

    const RESERVE: f64 = 0.05; // 5%
    const ACCURACY: f64 = 100.;

    let left = conditions.a * (1. - RESERVE);
    let right = conditions.b * (1. + RESERVE);

    let drawing_area = BitMapBackend::new("graph.png", (1200, 800))
        .into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();
    
    let mut ctx = ChartBuilder::on(&drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(left..right, left..right)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        LineSeries::new(((left * ACCURACY) as i64..(right * ACCURACY) as i64)
            .map(|x| x as f64 / ACCURACY)
            .map(|x: f64| (x, f(x))) , &BLACK)
    ).unwrap();
}