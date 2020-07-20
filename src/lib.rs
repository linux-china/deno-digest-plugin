#[calcite::deno_op]
fn multiply(a: f64, b: f64) -> f64 {
    multiply_impl(a, b)
}

fn multiply_impl(a: f64, b: f64) -> f64 {
    a * b
}

calcite::export!(multiply);

#[cfg(test)]
mod tests {
    use crate::multiply_impl;

    #[test]
    fn test_multiply() {
        let mut a = 2.0_f64;
        let mut b = 2.0_f64;
        let result = multiply_impl(a, b);
        print!("{}", result);
    }
}
