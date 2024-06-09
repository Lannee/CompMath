use crate::solutions::DifferentiableFunction;

#[derive(Clone)]
pub struct Integral {
    pub left: f64,
    pub right: f64,
    pub accuracy: f64,
    pub partitions: usize,
    pub function: DifferentiableFunction,
}


impl Integral {
    pub fn solve_by_rectangle(&self) -> Option<f64> {
        let Integral {left: a, right: b, accuracy, partitions: n, function: f} = self;

        let f = f.get_n_derivative(0).unwrap();

        let h = (b - a) / *n as f64;

        Some(
            (1..=*n)
                .map(|i| {f(a + h*i as f64)})
                .sum::<f64>() * h
        )
    }

    pub fn solve_by_trapeze(&self) -> Option<f64> {
        let Integral {left: a, right: b, accuracy, partitions: n, function: f} = self;

        let f = f.get_n_derivative(0).unwrap();

        let h = (b - a) / *n as f64;

        Some(
            (
                (1..*n)
                    .map(|i| {f(a + h*i as f64)})
                    .sum::<f64>() * 2. + f(*a) + f(*b)
            ) * h / 2.
        )
    }

    pub fn solve_by_simpson(&self) -> Option<f64> {
        let Integral {left: a, right: b, accuracy, partitions: n, function: f} = self;

        let f = f.get_n_derivative(0).unwrap();

        let h = (b - a) / *n as f64;

        Some(
            h / 3. * (
                f(*a) + f(*b) +
                (1..*n)
                    .map(|i| {
                        f(a + h*i as f64) * 
                        if i % 2 == 0 {
                            2.
                        } else {
                            4.
                        }
                    })
                    .sum::<f64>()
            )
        )
    }
}

impl Integral {
    pub fn solve_with_accuracy_increase(&mut self, solve_method: fn(&Self) -> Option<f64>) -> Option<f64> {
        let mut new_value: f64 = solve_method(self)?;
        let mut old_value: f64;

        loop {
            old_value = new_value;
            self.partitions *= 2;

            new_value = solve_method(self)?;

            if (new_value - old_value).abs() <= self.accuracy {
                break;
            }
        }

        Some(new_value)
    }
}