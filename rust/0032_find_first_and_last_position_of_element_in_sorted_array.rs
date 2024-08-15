impl Solution {
  pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let find_first_pos_of_an_element = |nums: &[i32], target: i32| -> i32 {
      let mut l = 0;
      let mut r = nums.len() as i32 - 1;

      while l <= r {
        let m = l + (r - l) / 2;
        if nums[m as usize] < target {
          l = m + 1;
        } else {
          r = m - 1;
        }
      }

      l
    };

    let left = find_first_pos_of_an_element(&nums, target);
    let right = find_first_pos_of_an_element(&nums, target + 1) - 1;

    if left <= right {
      [left, right].to_vec()
    } else {
      [-1, -1].to_vec()
    }
  }
}
