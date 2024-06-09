use crate::{func, solutions::DifferentiableFunction};

use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct InputFunction {
    pub description: &'static str,
    pub function: DifferentiableFunction,
}

const F1: Lazy<InputFunction> = Lazy::new(|| {
    InputFunction {
            description: "-2x^3 - 4x^2 + 8x - 4",
            function: func!(
                |x: f64| -> f64 {-2.*x.powi(3) - 4.*x.powi(2) + 8.*x - 4.}
            )
        }
    }
);

const F2: Lazy<InputFunction> = Lazy::new(|| {
    InputFunction {
            description: "sin(x)",
            function: func!(
                |x: f64| -> f64 {x.sin()}
            )
        }
    }
);

const F3: Lazy<InputFunction> = Lazy::new(|| {
    InputFunction {
            description: "e^(2x^2 - 1.5x)",
            function: func!(
                |x: f64| -> f64 {(2. * x * x - 1.5 * x).exp()}
            )
        }
    }
);

const F4: Lazy<InputFunction> = Lazy::new(|| {
    InputFunction {
            description: "atan(x^2)",
            function: func!(
                |x: f64| -> f64 {(x * x).atan()}
            )
        }
    }
);

pub const INPUT_FUNCTIONS: [Lazy<InputFunction>; 4] = [F1, F2, F3, F4];