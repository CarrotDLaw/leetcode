impl Solution {
  pub fn is_perfect_square(num: i32) -> bool {
    use std::cmp::Ordering;

    let mut l = 1;
    let mut r = num / 2 + 1;

    while l <= r {
      let m = l + (r - l) / 2;

      match m.checked_mul(m) {
        Some(m_squared) => match m_squared.cmp(&num) {
          Ordering::Less => l = m + 1,
          Ordering::Equal => return true,
          Ordering::Greater => r = m - 1,
        },
        None => r = m - 1,
      }
    }

    false
  }
}
