impl Solution {
  pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::cmp::Ordering::*;

    let mut res = Vec::new();
    nums.sort();

    for i in 0..nums.len() {
      if i > 0 && nums[i] == nums[i - 1] {
        continue;
      }

      let mut l = i + 1;
      let mut r = nums.len() - 1;

      while l < r {
        let sum = nums[i] + nums[l] + nums[r];

        match sum.cmp(&0) {
          Greater => r -= 1,
          Less => l += 1,
          Equal => {
            res.push([nums[i], nums[l], nums[r]].to_vec());
            l += 1;

            while l < r && nums[l - 1] == nums[l] {
              l += 1;
            }
          }
        }
      }
    }
    
    res
  }
}
