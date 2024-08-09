impl Solution {
  pub fn my_sqrt(x: i32) -> i32 {
    use std::cmp::Ordering::*;

    let mut l = 1;
    let mut r = x / 2 + 1;

    while l <= r {
      let m = l + (r - l) / 2;

      match m.checked_mul(m) {
        Some(m_squared) => match m_squared.cmp(&x) {
          Less => l = m + 1,
          Equal => return m,
          Greater => r = m - 1,
        },
        None => r = m - 1,
      }
    }

    r
  }
}
