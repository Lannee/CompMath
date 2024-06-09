use std::ops::Mul;

use crate::input::Dots;
use itermore::prelude::*;

pub fn lagrang_method(dots: &Dots) -> impl Fn(f64) -> f64 {
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


pub fn neutons_sep_method(dots: &Dots) -> impl Fn(f64) -> f64 {
    let dots = dots.clone();
    move |x: f64| -> f64 {
        (0..dots.len())
        .map(|k| {
            sep_f(&dots[..k+1]) * 
            dots.iter()
                .enumerate()
                .filter(|(j, _)| *j < k)
                .map(|(_, (x_j, _))| x - x_j)
                .fold(1.,  Mul::<f64>::mul)
        }).sum()
    }
}

fn sep_f(series: &[(f64, f64)]) -> f64 {
    if series.len() == 1 {series[0].1}
    else {
        (sep_f(&series[1..]) - sep_f(&series[..series.len()-1])) 
        / (series[series.len()-1].0 - series[0].0)
    }
}


pub fn neutons_fin_method(dots: &Dots) -> impl Fn(f64) -> f64 {
    assert!(dots.len() > 1);
    let dots = dots.clone();
    let h = dots.iter()
        .array_windows()
        .map(|[y_i, y_ip1]| (y_ip1.0 - y_i.0).abs())
        .sum::<f64>() / (dots.len() - 1) as f64;

    let ys: Vec<f64> = dots.iter().map(|dot| dot.1).collect();

    let mut dy_ns: Vec<_> = Vec::with_capacity(dots.len() + 1);
    dy_ns.push(ys);

    for n in 0..dots.len()-1 {
        dy_ns.push(get_diffs(&dy_ns[n]));
    }

    let dy_ns: Vec<f64> = dy_ns.iter()
        .rev()
        .enumerate()
        .map(|(n, vec)| vec[n])
        .rev()
        .collect();


    move |x: f64| -> f64 {
        let t = |x: f64| (x - dots[dots.len()-1].0) / h;

        dy_ns.iter()
            .enumerate()
            .map(|(n, y_n)| {
                y_n * {
                    (0..n)
                        .map(|i| t(x) + i as f64)
                        .product::<f64>()
                } / (1..=n).product::<usize>() as f64  // n!
            })
            .sum()
    }
}

fn get_diffs(numbers: &Vec<f64>) -> Vec<f64> {
    numbers.iter()
        .array_windows()
        .map(|[y_i, y_ip1]| y_ip1 - y_i)
        .collect()
}