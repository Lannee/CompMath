pub mod env_args;
pub mod functions;

use std::fmt;
use serde::Deserialize;
use serde_json;
use std::fmt::Debug;
use std::io;

use crate::solutions::Integral;

use self::functions::INPUT_FUNCTIONS;
use self::env_args::EnvArgs;


pub enum GetEquationError {
    FileError(std::io::Error),
    ParseError(serde_json::Error),
    InputError(String),
    NoFunctionWithNumber(usize),
} 

impl fmt::Display for GetEquationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileError(err) => write!(f, "Error with input file: {}", err),
            Self::ParseError(err) => write!(f, "Invalid parse format: {}", err),
            Self::InputError(message) => write!(f, "Invalid input data: {}", message),
            Self::NoFunctionWithNumber(n) => write!(f, "function number {} not found", n),
        }
    }
}

#[derive(Deserialize, Debug)]
struct IntegralAsJson {
    pub left: f64,
    pub right: f64,
    pub accuracy: f64,
    pub partitions: usize,
    pub function: usize,
}

impl std::convert::TryInto<Integral> for IntegralAsJson {
    type Error = GetEquationError;

    fn try_into(self) -> Result<Integral, Self::Error> {
        let function = match INPUT_FUNCTIONS.get(self.function - 1) {
            None => return Err(GetEquationError::NoFunctionWithNumber(self.function)),
            Some(f) => f,
        }.function.clone();

        Ok(
            Integral {
                left: self.left,
                right: self.right,
                accuracy: self.accuracy,
                partitions: self.partitions,
                function,
            }
        )
    } 
}


pub fn try_get_integral() -> Result<Integral, GetEquationError> {
    let content = get_input().map_err(|err| GetEquationError::FileError(err))?;

    parse_input(&content)?.try_into()
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

fn parse_input(content: &String) -> Result<IntegralAsJson, GetEquationError> {
    serde_json::from_str::<IntegralAsJson>(content).map_err(|err| GetEquationError::ParseError(err))
}