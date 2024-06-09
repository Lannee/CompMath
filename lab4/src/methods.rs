use crate::input::Dots;
use mathru::
algebra::linear::{
        matrix::{General, Solve},
        vector::Vector,
    };

pub fn linear_method(dots: &Dots) -> impl Fn(f64) -> f64 {

    let (a0, a1) = get_linear_coeffs(dots);
    println!("linear function: a0 = {a0:.3}, a1 = {a1:.3}");

    move |x: f64| -> f64 {
        a0 * x + a1
    }
}


pub fn quad_method(dots: &Dots) -> impl Fn(f64) -> f64 {
    let mut matrix = General::<f64>::zero(3, 3);
    let mut vector = Vector::<f64>::zero(3);

    dots.iter()
        .for_each(|(x, y)| {
            matrix[[0, 0]] += 1.;        matrix[[0, 1]] += x;         matrix[[0, 2]] += x.powi(2);
            matrix[[1, 0]] += x;         matrix[[1, 1]] += x.powi(2); matrix[[1, 2]] += x.powi(3);
            matrix[[2, 0]] += x.powi(2); matrix[[2, 1]] += x.powi(3); matrix[[2, 2]] += x.powi(4);

            vector[0] += *y;
            vector[1] += x * y;
            vector[2] += x * x * y;
        });

    let coeffs = matrix.solve(&vector).unwrap();
    let a0 = coeffs[0];
    let a1 = coeffs[1];
    let a2 = coeffs[2];
    println!("quadratic function: a0 = {a0:.3}, a1 = {a1:.3}, a2 = {a2:.3}");


    move |x: f64| -> f64 {
        a0 + a1 * x + a2 * x.powi(2)
    }
}

pub fn cubic_method(dots: &Dots) -> impl Fn(f64) -> f64 {
    let mut matrix = General::<f64>::zero(4, 4);
    let mut vector = Vector::<f64>::zero(4);

    dots.iter()
        .for_each(|(x, y)| {
            matrix[[0, 0]] += 1.;        matrix[[0, 1]] += x;         matrix[[0, 2]] += x.powi(2); matrix[[0, 3]] += x.powi(3);
            matrix[[1, 0]] += x;         matrix[[1, 1]] += x.powi(2); matrix[[1, 2]] += x.powi(3); matrix[[1, 3]] += x.powi(4);
            matrix[[2, 0]] += x.powi(2); matrix[[2, 1]] += x.powi(3); matrix[[2, 2]] += x.powi(4); matrix[[2, 3]] += x.powi(5);
            matrix[[3, 0]] += x.powi(3); matrix[[3, 1]] += x.powi(4); matrix[[3, 2]] += x.powi(5); matrix[[3, 3]] += x.powi(6);

            vector[0] += *y;
            vector[1] += x * y;
            vector[2] += x.powi(2) * y;
            vector[3] += x.powi(3) * y;
        });

    let coeffs = matrix.solve(&vector).unwrap();
    let a0 = coeffs[0];
    let a1 = coeffs[1];
    let a2 = coeffs[2];
    let a3 = coeffs[3];

    println!("cubic function: a0 = {a0:.3}, a1 = {a1:.3}, a2 = {a2:.3}, a3 = {a3:.3}");

    move |x: f64| -> f64 {
        a0 + a1 * x + a2 * x.powi(2) + a3 * x.powi(3)
    }
}

pub fn exponential_method(dots: &Dots) -> impl Fn(f64) -> f64 {

    let dots: Vec<_> = 
        dots.iter()
            .map(|(x, y)| (*x, y.ln()))
            .collect();

    let (a0, a1) = get_linear_coeffs(&dots);

    println!("exponential function: a0 = {a0:.3}, a1 = {a1:.3}");

    move |x: f64| -> f64 {
        (a0 * x + a1).exp()
    }
}

pub fn logarifmic_method(dots: &Dots) -> impl Fn(f64) -> f64 {
    let dots: Vec<_> = 
        dots.iter()
            .map(|(x, y)| (x.ln(), *y))
            .collect();

    let (a0, a1) = get_linear_coeffs(&dots);

    println!("logarifmic function: a0 = {a0:.3}, a1 = {a1:.3}");

    move |x: f64| -> f64 {
        a0 * x.ln() + a1
    }
}

pub fn power_method(dots: &Dots) -> impl Fn(f64) -> f64 {
    let dots: Vec<_> = 
        dots.iter()
            .map(|(x, y)| (x.ln(), y.ln()))
            .collect();

    let (a0, a1) = get_linear_coeffs(&dots);

    println!("power function: a0 = {a0:.3}, a1 = {a1:.3}");
    
    move |x: f64| -> f64 {
        a0.exp() * x.powf(a1)
    }
}

fn get_linear_coeffs(dots: &Dots) -> (f64, f64) {
    let (sx, sxx, sy, sxy) = 
    dots.iter()
        .fold((0., 0., 0., 0.), |(sx, sxx, sy, sxy), (x, y)| {
            (sx + x, sxx + x.powi(2), sy + y, sxy + x*y)
        });

    let n = dots.len() as f64;
    let a = (sxy * n - sx * sy)/(sxx * n - sx.powi(2)); 
    let b = (sxx * sy - sx * sxy)/(sxx * n - sx.powi(2)); 

    (a, b)
}