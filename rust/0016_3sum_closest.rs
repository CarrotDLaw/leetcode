impl Solution {
  pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    use std::cmp::Ordering::*;
  
    nums.sort();
    let mut min_diff = i32::MAX;
  
    for i in 0..nums.len() {
      let mut l = i + 1;
      let mut r = nums.len() - 1;
  
      while l < r {
        let sum = nums[i] + nums[l] + nums[r];
        let diff = sum - target;
  
        if diff.abs() < min_diff.abs() {
          min_diff = diff;
        }
  
        match diff.cmp(&0) {
          Less => l += 1,
          Equal => return target,
          Greater => r -= 1,
        }
      }
    }
  
    target + min_diff
  }
}
