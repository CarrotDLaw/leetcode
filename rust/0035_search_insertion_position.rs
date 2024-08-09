impl Solution {
  pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    use std::cmp::Ordering::*;
  
    let mut l = 0;
    let mut r = nums.len() as i32;
  
    while l <= r {
      let m = l + (r - l) / 2;
  
      match nums[m as usize].cmp(&target) {
        Less => l = m + 1,
        Greater => r = m - 1,
        Equal => return m,
      }
    }
  
    l
  }
}
