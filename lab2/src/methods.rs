// 1, 3, 5, 7
// пд, Ньют, пи(урав), пи(системы)

use std::ops::RangeInclusive;

use crate::input::functions::SystemOfEquations;
use crate::solutions::{Conditions, DifferentiableFunction, SolveError};
use crate::n_derivative;
use crate::solutions::function::OneArgumentFunction;

pub fn half_div_method(function: &DifferentiableFunction<OneArgumentFunction>, conditions: &Conditions) -> Result<f64, SolveError> {
    let Conditions {mut a, mut b, epsilon, max_iterations} = conditions;

    let f: OneArgumentFunction  = n_derivative!(function, 0);

    let mut x = (a + b) / 2.;

    for _ in 0..*max_iterations {
        x = (a + b) / 2.;
        
        if f(a) * f(x) > 0. {
            a = x;
        } else {
            b = x;
        }

        if (a - b).abs() <= *epsilon && f(x).abs() < *epsilon {
            return Ok(x);
        }
    }

    Err(SolveError::NoSolveError)
}

pub fn neutons_method(function: &DifferentiableFunction<OneArgumentFunction>, conditions: &Conditions) -> Result<f64, SolveError> {
    let Conditions {a, b, epsilon, max_iterations} = conditions;

    let f: OneArgumentFunction  = n_derivative!(function, 0);

    let df: OneArgumentFunction = n_derivative!(function, 1);

    let mut x = (a + b) / 2.;

    for _ in 0..*max_iterations {
        let new_x = x - f(x)/df(x);

        if ((new_x - x).abs() <= *epsilon &&
            f(new_x).abs() < *epsilon) || 
            (f(new_x)/df(new_x)).abs() <= *epsilon {
                return if (a..b).contains(&&new_x) {
                    Ok(new_x)
                } else {
                    Err(SolveError::NoSolveError)
                };
        }
        x = new_x;
    }

    Err(SolveError::NoSolveError)
}


pub fn simple_iterations_method(function: &DifferentiableFunction<OneArgumentFunction>, conditions: &Conditions) -> Result<f64, SolveError> {
    let Conditions {a, b, epsilon, max_iterations} = conditions;

    let fi = get_fi_one_arg(function, conditions)?;

    let mut x = (a + b) / 2.;

    for _ in 0..*max_iterations {
        let new_x = fi(x);

        if (new_x - x).abs() <= *epsilon {
            return Ok(new_x);
        }
        x = new_x;
    }

    Err(SolveError::NoSolveError)
}

pub fn system_symple_iterations_method(system: &SystemOfEquations, conditions: &Conditions) -> Result<(f64, f64), SolveError> {
    let Conditions {a, b, epsilon, max_iterations} = conditions;

    let (fi1, fi2) = get_fi_two_arg(system, conditions)?;

    let mut x: (f64, f64) = (0., 0.);

    for _ in 0..*max_iterations {
        let new_x: (f64, f64) = (fi1(x), fi2(x));

        if f64::max(
            (new_x.0 - x.0).abs(),
            (new_x.1 - x.1).abs()
        ) <= *epsilon {
            return Ok(new_x);
        }
        x = new_x;
    }

    Err(SolveError::NoSolveError)
}

fn get_fi_one_arg(function: &DifferentiableFunction<OneArgumentFunction>, conditions: &Conditions) -> Result<impl Fn(f64) -> f64, SolveError> {

    let f: OneArgumentFunction  = n_derivative!(function, 0);

    let df: OneArgumentFunction = n_derivative!(function, 1);

    let lambda = -1. / get_max_on_range(conditions.a..=conditions.b, df);

    let fi = move |x: f64| -> f64 {x + lambda * f(x)};
    let dfi = |x: f64| -> f64 {1. + lambda * df(x)};

    let q = get_max_on_range(conditions.a..=conditions.b, dfi);

    if q > 1. {
        return Err(SolveError::NoConvergenceError);
    }

    Ok(fi)
}

fn get_fi_two_arg(system: &SystemOfEquations, conditions: &Conditions) -> Result<(impl Fn((f64, f64)) -> f64, impl Fn((f64, f64)) -> f64), SolveError> {
    let f1 = n_derivative!(system.0, 0);
    let f2 = n_derivative!(system.1, 0);
    Ok((
        move |args: (f64, f64)| -> f64 {f1(args.0, args.1) + args.0},
        move |args: (f64, f64)| -> f64 {f2(args.0, args.1) + args.1},
    ))
}

fn get_max_on_range<F: Fn(f64) -> f64>(range: RangeInclusive<f64>, f: F) -> f64 {
    let dfa = f(*range.start());
    let dfb = f(*range.end());

    if dfa > dfb {dfa} else {dfb}
}