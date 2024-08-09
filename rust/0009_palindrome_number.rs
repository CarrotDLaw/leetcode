impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
      return false;
    }

    let mut org: i32 = x;
    let mut new: i32 = 0;

    while org != 0 {
      new *= 10;
      new += org % 10;
      org /= 10;
    }

    x == new
  }
}
