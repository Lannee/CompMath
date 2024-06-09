use crate::{func, solutions::{function::{OneArgumentFunction, TwoArgumentFunction}, DifferentiableFunction}};

use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct InputFunction {
    pub description: &'static str,
    pub function: DifferentiableFunction<OneArgumentFunction>,
}

const F1: Lazy<InputFunction> = Lazy::new(|| {
    InputFunction {
            description: "x^3 - 1.89x^2 - 2x + 1.76",
            function: func!(
                OneArgumentFunction,
                |x: f64| -> f64 {x.powi(3) - 1.89*x.powi(2) - 2.*x + 1.76},
                |x: f64| -> f64 {3.*x.powi(2) - 3.78*x - 2.}
            )
        }
    }
);

const F2: Lazy<InputFunction> = Lazy::new(|| {
    InputFunction {
            description: "sin(x)",
            function: func!(
                OneArgumentFunction,
                |x: f64| -> f64 {x.sin()},
                |x: f64| -> f64 {x.cos()}
            )
        }
    }
);

const F3: Lazy<InputFunction> = Lazy::new(|| {
    InputFunction {
            description: "e^(2x^2 - 1.5x)",
            function: func!(
                OneArgumentFunction,
                |x: f64| -> f64 {(2. * x * x - 1.5 * x).exp()}
            )
        }
    }
);

const F4: Lazy<InputFunction> = Lazy::new(|| {
    InputFunction {
            description: "atan(x^2)",
            function: func!(
                OneArgumentFunction,
                |x: f64| -> f64 {(x * x).atan()}
            )
        }
    }
);

pub const INPUT_FUNCTIONS: [Lazy<InputFunction>; 4] = [F1, F2, F3, F4];


pub type SystemOfEquations = (Lazy<DifferentiableFunction<TwoArgumentFunction>>, Lazy<DifferentiableFunction<TwoArgumentFunction>>);


pub const SYSTEM1: SystemOfEquations = (
    Lazy::new(|| {
        func!(
            TwoArgumentFunction,
            |x: f64, y: f64| -> f64 {(x + y).sin() - 1.2 * x - 0.2}
        )
    }),
    Lazy::new(|| {
        func!(
            TwoArgumentFunction,
            |x: f64, y: f64| -> f64 {x.powi(2) + 2.*y.powi(2) - 1.}
        )
    }),
);

pub const SYSTEM2: SystemOfEquations = (
    Lazy::new(|| {
        func!(
            TwoArgumentFunction,
            |x: f64, y: f64| -> f64 {-2.*x.powi(2) + 1.6*x - y + 1.}
        )
    }),
    Lazy::new(|| {
        func!(
            TwoArgumentFunction,
            |x: f64, y: f64| -> f64 {x - 1.7 - y}
        )
    }),
);

pub const INPUT_SYSTEMS: [SystemOfEquations; 2] = [SYSTEM1, SYSTEM2];
