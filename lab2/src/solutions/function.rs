pub type OneArgumentFunction = fn(f64) -> f64;
pub type TwoArgumentFunction = fn(f64, f64) -> f64;

#[derive(Debug, Clone)]
pub struct DifferentiableFunction<T> {
    derivatives: Vec<T>,
}

impl<T> DifferentiableFunction<T> {
    pub fn get_n_derivative(&self, n: usize) -> Option<&T> {
        self.derivatives.get(n)
    }

    pub fn add_derivative(&mut self, f: T) {
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
    ($type:ty, $( $x:expr ),* ) => {
        {
            let mut temp = DifferentiableFunction::<$type>::new();
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