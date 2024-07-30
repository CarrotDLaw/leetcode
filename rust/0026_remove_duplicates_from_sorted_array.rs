impl Solution {
  pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut left_ptr: usize = 1;
    for right_ptr in 1..nums.len() {
      if nums[right_ptr] == nums[right_ptr - 1] {
        continue;
      }

      nums[left_ptr] = nums[right_ptr];
      left_ptr += 1;
    }

    left_ptr as i32
  }
}
