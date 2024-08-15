impl Solution {
  pub fn my_atoi(s: String) -> i32 {
    let mut s = s
      .trim_start()
      .chars()
      .skip_while(|c| c.is_ascii_whitespace());
    let first_char = s.next().unwrap_or(' ');
  
    if first_char != '-' && first_char != '+' && !first_char.is_ascii_digit() {
      return 0;
    }
  
    let sign = if first_char == '-' { -1 } else { 1 };
    let initial_value = first_char.to_digit(10).unwrap_or(0) as i32;
  
    s.map_while(|c| c.to_digit(10))
      .fold(initial_value, |acc, c| {
        acc.saturating_mul(10).saturating_add(sign * c as i32)
      })
  }
}
