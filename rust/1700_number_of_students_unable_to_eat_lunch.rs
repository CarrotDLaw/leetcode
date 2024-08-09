impl Solution {
  pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    let mut counts = [0, 0].to_vec();

    for s in students {
      counts[s as usize] += 1;
    }

    for s in sandwiches {
      counts[s as usize] -= 1;

      if counts[s as usize] < 0 {
        return counts[1 - s as usize];
      }
    }

    0
  }
}
