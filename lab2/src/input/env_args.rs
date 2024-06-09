const FILE_PATH_INDEX: usize = 1;

pub struct EnvArgs {
    pub file_path: Option<String>
}

impl std::convert::From<Vec<String>> for EnvArgs {
    fn from(vec: Vec<String>) -> EnvArgs {
        EnvArgs {file_path: vec.get(FILE_PATH_INDEX).map(|string| string.clone())}
    }
}

impl EnvArgs {
    pub fn get() -> EnvArgs {
        EnvArgs::from(std::env::args().collect::<Vec<String>>())
    }
}