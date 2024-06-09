pub mod env_args;
pub mod functions;

use crate::solutions::Conditions;

use self::env_args::EnvArgs;
use core::fmt;
use std::io;

use serde::Deserialize;
use serde_json;


pub enum InputError {
    FileError(std::io::Error),
    ParseError(serde_json::Error),
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
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Input {
    pub func_index: usize,
    pub system_index: usize,
    pub conditions: Conditions
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