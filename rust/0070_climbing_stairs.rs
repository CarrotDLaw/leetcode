impl Solution {
  pub fn climb_stairs(n: i32) -> i32 {
    fn climb(n: i32, v: &mut [i32]) -> i32 {
      if n < 0 {
        return 0;
      }

      if n == 0 {
        return 1;
      }

      if v[n as usize] != -1 {
        return v[n as usize];
      }

      v[n as usize] = climb(n - 1, v) + climb(n - 2, v);
      v[n as usize]
    }

    let mut v = vec![-1; (n + 1) as usize];
    climb(n, &mut v)
  }
}
