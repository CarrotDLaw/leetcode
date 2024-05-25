impl Solution {
  pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut res = nums[0];
    let mut cnt = 0;

    for n in nums {
      if cnt == 0 {
        cnt = 1;
        res = n;
      } else if n == res {
        cnt += 1;
      } else {
        cnt -= 1;
      }
    }

    res 
  }
}
