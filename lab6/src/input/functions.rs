
pub type DiffEquation = fn(f64, f64) -> f64;

#[derive(Debug)]
pub struct InputFunction {
    pub description: &'static str,
    pub function: DiffEquation,
}

const F1: InputFunction = InputFunction {
        description: "y' = 2x + y",
        function: |x, y| 2. * x + y,
    };

const F2: InputFunction = InputFunction {
        description: "y' = sin(x + y)",
        function: |x, y| (x + y).sin()
    };

const F3: InputFunction = InputFunction {
        description: "y' = x^2 + y",
        function: |x, y| x.powi(2) + y
    };

const F4: InputFunction = InputFunction {
        description: "y' = x + 1",
        function: |x, y| x + 1.
    };

pub const INPUT_FUNCTIONS: [InputFunction; 4] = [F1, F2, F3, F4];