mod input;
mod solutions;

use input::try_get_integral;
use solutions::Integral;

fn main() {
    match try_get_integral() {
        Err(e) => println!("{}", e),
        Ok(integral) => {
            match integral.clone().solve_with_accuracy_increase(Integral::solve_by_rectangle) {
                None => println!(""),
                Some(result) => println!("Значение интеграла по методу прямоугольника = {}", result),
            }

            match integral.clone().solve_with_accuracy_increase(Integral::solve_by_trapeze) {
                None => println!(""),
                Some(result) => println!("Значение интеграла по методу трапеции = {}", result),
            }

            match integral.clone().solve_with_accuracy_increase(Integral::solve_by_simpson) {
                None => println!(""),
                Some(result) => println!("Значение интеграла по методу Симпсона = {}", result),
            }
        }
    }
}