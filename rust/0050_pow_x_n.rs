impl Solution {
  pub fn my_pow(x: f64, n: i32) -> f64 {
    if n == 0 {
      return 1.0;
    }

    let mut result = 1.0;
    let mut base = x;
    let mut exp = n.unsigned_abs();

    while exp > 0 {
      if exp % 2 == 1 {
        result *= base;
      }

      base *= base;
      exp /= 2;
    }

    if n < 0 {
      return 1.0 / result;
    }

    result 
  }
}
