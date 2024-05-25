impl Solution {
  pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;

    let nums_len = nums.len();
    let set: HashSet<i32> = HashSet::from_iter(nums);
    
    nums_len != set.len()
  }
}
