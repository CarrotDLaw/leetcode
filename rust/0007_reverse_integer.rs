impl Solution {
  pub fn reverse(x: i32) -> i32 {
    let mut num: i32 = x;
    let mut rev: i32 = 0;
  
    while num != 0 {
      let digit: i32 = num % 10;
      num /= 10;
  
      if (rev > i32::MAX / 10) || (rev == i32::MAX / 10 && digit > i32::MAX % 10) {
        return 0;
      }
  
      if (rev < i32::MIN / 10) || (rev == i32::MIN / 10 && digit < i32::MIN % 10) {
        return 0;
      }
  
      rev = rev * 10 + digit;
    }
  
    rev
  }
}
