use serde::Deserialize;


#[derive(Deserialize, Clone, Debug)]
pub struct Conditions {
    pub a: f64,
    pub b: f64,
    pub epsilon: f64,
    pub max_iterations: usize,
}