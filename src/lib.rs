#[calcite::deno_op]
fn multiply(a: f64, b: f64) -> f64 {
    multiply_impl(a, b)
}

#[calcite::deno_op]
fn welcome(name: &str) -> String {
    return format!("Hello {}!", name);
}

fn multiply_impl(a: f64, b: f64) -> f64 {
    a * b
}

calcite::export!(multiply, welcome);

#[cfg(test)]
mod tests {
    use crate::multiply_impl;

    #[test]
    fn test_multiply() {
        assert_eq!(multiply_impl(2.0, 2.0), 4.0);
    }
}
