impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max_len = 0;
    let mut pos = [0; 128];

    let mut l = 0;

    for (r, c) in s.bytes().enumerate() {
      l = l.max(pos[c as usize]);
      max_len = max_len.max(r - l + 1);
      pos[c as usize] = r + 1;
    }

    max_len as i32
  }
}
