impl Solution {
  pub fn max_area(height: Vec<i32>) -> i32 {
    use std::cmp::Ordering::*;

    let mut max_area = 0;

    let mut height = height.iter().enumerate();
    let mut l_ptr = height.next();
    let mut r_ptr = height.next_back();

    while let (Some((l, l_height)), Some((r, r_height))) = (l_ptr, r_ptr) {
      max_area = max_area.max((r - l) as i32 * l_height.min(r_height));

      match l_height.cmp(r_height) {
        Less => l_ptr = height.next(),
        Greater => r_ptr = height.next_back(),
        Equal => {
          l_ptr = height.next();
          r_ptr = height.next_back();
        }
      }
    }

    max_area
  }
}
