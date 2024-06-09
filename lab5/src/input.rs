pub mod env_args;
pub mod functions;


use self::env_args::EnvArgs;
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

pub type Dots = Vec<(f64, f64)>;

#[derive(Deserialize)]
pub struct Input{
    pub variant: InputVariant
}

#[derive(Deserialize)]
pub enum InputVariant {
    Function{
        func_index: usize,
        a: f64,
        b: f64,
        n: usize
    },
    Dots(Dots)
}

impl Input {
    pub fn get_bounds(&self) -> Option<(f64, f64)> {
        match self.variant {
            InputVariant::Dots(ref dots) => {
                Some(
                    (
                        dots.iter().map(|a| a.0).min_by(|a, b| a.partial_cmp(b).unwrap())?,
                        dots.iter().map(|a| a.0).max_by(|a, b| a.partial_cmp(b).unwrap())?
                    )
                )
            },
            InputVariant::Function{func_index: _, a, b, n: _} => {
                Some((a, b))
            }
        }
    }
}

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