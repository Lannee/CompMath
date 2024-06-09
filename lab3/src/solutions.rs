pub mod integral;
pub mod function;

pub use self::integral::Integral;
pub use self::function::DifferentiableFunction;

#[derive(Debug)]
pub enum SolveError {
    DifferentiationError{power: usize},
    NoSolveError,
}