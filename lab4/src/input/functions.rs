
#[derive(Debug)]
pub struct InputFunction {
    pub description: &'static str,
    pub function: fn(f64) -> f64
}

const F1: InputFunction = InputFunction {
        description: "x^2",
        function: |x| x.powi(2)
    };

const F2: InputFunction = InputFunction {
        description: "sin(x)",
        function: |x| x.sin()
    };

const F3: InputFunction = InputFunction {
        description: "e^(2x^2 - 1.5x)",
        function: |x| (2. * x * x - 1.5 * x).exp()
    };

const F4: InputFunction = InputFunction {
        description: "atan(x^2)",
        function: |x| (x * x).atan()
    };

pub const INPUT_FUNCTIONS: [InputFunction; 4] = [F1, F2, F3, F4];