use crate::input::{functions::DiffEquation, Conditions};


pub type Dots = Vec<(f64, f64)>;

pub fn get_bounds(dots: &Dots) -> (f64, f64) {
    (
        dots.iter()
            .map(|el| el.0)
            .min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
        dots.iter()
            .map(|el| el.0)
            .max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
    )
}

pub type SolveMethod = dyn Fn(&DiffEquation, &Conditions) -> Dots;


pub fn eyler_method(eq: &DiffEquation, condition: &Conditions) -> Dots {
    let Conditions {mut x, h, mut y_0, x_n, eps: _} = condition;
    let n = ((x_n - x) / h) as usize;

    (1..=n+2)
        .map(|_| {
            y_0 = y_0 + *h * eq(x, y_0);
            x = x + *h;
            (x - *h, y_0)
        })
        .collect()
}

pub fn improved_eyler_method(eq: &DiffEquation, condition: &Conditions) -> Dots {
    let Conditions {mut x, h, mut y_0, x_n, eps: _} = condition;
    let n = ((x_n - x) / h) as usize;
    (1..=n)
        .map(|_| {
            y_0 = y_0 + 
                    *h / 2. * 
                (
                    eq(x, y_0) + 
                    eq(x + *h, y_0 + 
                        *h * 
                            eq(x, y_0)
                    )
                );
            x = x + *h as f64;
            (x, y_0)
        })
        .collect()
}

pub fn adams_method(eq: &DiffEquation, condition: &Conditions) -> Dots {
    let Conditions {x, h, y_0, x_n, eps} = condition;
    let n = ((x_n - x) / h) as usize;
    assert!(n >= 3);

    let (k0, k1, k2, k3) = get_adams_koefs(*h);

    let mut dots = improved_eyler_method(eq, &Conditions { x: *x, h: *h, y_0: *y_0, x_n: *x + 4. * *h, eps: *eps });
    dots.append(&mut vec![(0., 0.); n-2]);

    for i in 3..=n {
        let f_im3 = dots[i-3];
        let f_im2 = dots[i-2];
        let f_im1 = dots[i-1];
        let f_i = dots[i];
               
        dots[i+1] = 
            (
                f_i.0 + h,
                f_i.1 + k0 * eq(f_i.0, f_i.1) + k1 * eq(f_im1.0, f_im1.1) + k2 * eq(f_im2.0, f_im2.1) + k3 * eq(f_im3.0, f_im3.1)   
            );
    }

    dots.iter_mut().for_each(|el| el.0 -= h);

    dots
}

fn get_adams_koefs(h: f64) -> (f64, f64, f64, f64) {
    (
            h             +    h.powi(2)/2. + 5.*h.powi(3)/12. + 3.*h.powi(4)/8.,
        - h.powi(2)/2.  - 5.*h.powi(3)/6. - 9.*h.powi(4)/8.,
        5.*h.powi(3)/12. + 9.*h.powi(4)/8.,
        -3.*h.powi(4)/8.
    )
}