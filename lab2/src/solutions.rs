pub mod condition;
pub mod function;

pub use self::condition::Conditions;
pub use self::function::DifferentiableFunction;

#[derive(Debug)]
pub enum SolveError {
    DifferentiationError{power: usize},
    NoSolveError,
    NoConvergenceError
}