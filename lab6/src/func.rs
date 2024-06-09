use std::ops::Mul;

use itermore::IterArrayWindows;

use crate::Dots;


pub fn dots_to_func(dots: &Dots) -> impl Fn(f64) -> f64 {
    let dots = dots.clone();
    move |x: f64| -> f64 {
        dots.iter()
            .enumerate()
            .map(|(i, (x_i, y_i))| {
                y_i * {
                    dots.iter()
                        .enumerate()
                        .filter(|(j, _)| i != *j)
                        .map(|(_, (x_j, _))| {
                            (x - x_j) / (x_i - x_j)
                        })
                        .fold(1.,  Mul::<f64>::mul)
                }
            })
            .sum()
    }
}