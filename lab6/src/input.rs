pub mod env_args;
pub mod functions;


use self::{env_args::EnvArgs, functions::{DiffEquation, INPUT_FUNCTIONS}};
use core::fmt;
use std::io;

use serde::Deserialize;
use serde_json;


pub enum InputError {
    FileError(std::io::Error),
    ParseError(serde_json::Error),
    CannotGetBounds,
    FunctionIndexError(usize)
} 

impl fmt::Debug for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileError(err) => write!(f, "Error with input file: {}", err),
            Self::ParseError(err) => write!(f, "Invalid parse format: {}", err),
            Self::CannotGetBounds => write!(f, "Cannot get bounds"),
            Self::FunctionIndexError(index) => write!(f, "Invalid function index {index}"),
        }
    }
}

#[derive(Deserialize)]
pub struct Input{
    pub eq_i: usize,
    pub x: f64,
    pub h: f64,
    pub y_0: f64,
    pub n: usize,
    pub eps: f64,
}

impl Input {
    pub fn get_func(&self) -> Result<DiffEquation, InputError> {
        INPUT_FUNCTIONS
            .get(self.eq_i - 1)
            .ok_or_else(|| InputError::FunctionIndexError(self.eq_i))
            .map(|i| i.function)
    }
}

impl Into<Conditions> for Input {
    fn into(self) -> Conditions {
        Conditions {
            x: self.x,
            h: self.h,
            y_0: self.y_0,
            x_n: self.x + self.n as f64 * self.h,
            eps: self.eps,
        }
    }
}

#[derive(Clone)]
pub struct Conditions {
    pub x: f64,
    pub h: f64,
    pub y_0: f64,
    pub x_n: f64,
    pub eps: f64,
}

// impl Input {
//     pub fn get_bounds(&self) -> Option<(f64, f64)> {
//         match self.variant {
//             InputVariant::Dots(ref dots) => {
//                 Some(
//                     (
//                         dots.iter().map(|a| a.0).min_by(|a, b| a.partial_cmp(b).unwrap())?,
//                         dots.iter().map(|a| a.0).max_by(|a, b| a.partial_cmp(b).unwrap())?
//                     )
//                 )
//             },
//             InputVariant::Function{func_index: _, a, b, n: _} => {
//                 Some((a, b))
//             }
//         }
//     }
// }

pub fn try_get_input() -> Result<Input, InputError> {
    let content = get_input().map_err(|err| InputError::FileError(err))?;

    parse_input(&content)
}

fn get_input() -> io::Result<String> {
    let env_args = EnvArgs::get();

    let mut file: Box<dyn io::Read> = match env_args.file_path {
        Some(file_path) => Box::new(std::fs::File::open(file_path)?),
        None => Box::new(io::stdin())
    };

    let mut content = String::new();

    file.read_to_string(&mut content)?;

    Ok(content)
}

fn parse_input(content: &String) -> Result<Input, InputError> {
    serde_json::from_str::<Input>(content).map_err(|err| InputError::ParseError(err))
}