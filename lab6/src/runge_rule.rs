use crate::{input::{functions::DiffEquation, Conditions}, Dots, SolveMethod};




pub fn apply_runge_rule(
        method: &SolveMethod, 
        method_period: usize, 
        eq: &DiffEquation, 
        mut conditions: Conditions
) -> Dots {

    loop {
        let dots_h_step = method(eq, &conditions);

        conditions.h = conditions.h / 2.;
        let dots_h_half_step = method(eq, &conditions);
        if (
            dots_h_step.last().unwrap().1 - 
            dots_h_half_step.last().unwrap().1
            ).abs() / 
                2_usize.pow(method_period as u32) as f64 - 1.
                <= conditions.eps {
            return dots_h_step;
        }
    }
    // method(eq, &conditions)
}