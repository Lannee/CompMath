pub type OneArgumentFunction = fn(f64) -> f64;

#[derive(Clone, Debug)]
pub struct DifferentiableFunction {
    derivatives: Vec<OneArgumentFunction>,
}

impl DifferentiableFunction {
    pub fn get_n_derivative(&self, n: usize) -> Option<&OneArgumentFunction> {
        self.derivatives.get(n)
    }

    pub fn add_derivative(&mut self, f: OneArgumentFunction) {
        self.derivatives.push(f);
    }

    pub fn new() -> Self {
        DifferentiableFunction {
            derivatives: Vec::new(),
        }
    }
}

#[macro_export]
macro_rules! func {
    ( $( $x:expr ),* ) => {
        {
            let mut temp = DifferentiableFunction::new();
            $(
                temp.add_derivative($x);
            )*
            temp
        }
    };
}

#[macro_export]
macro_rules! n_derivative {
    ( $func:expr, $n:expr) => {
        match $func.get_n_derivative($n) {
            None => return Err(SolveError::DifferentiationError { power: $n }),
            Some(f) => *f,
        }
    };
}