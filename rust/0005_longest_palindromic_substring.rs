impl Solution {
  pub fn longest_palindrome(s: String) -> String {
    let s = s.as_bytes();
    let mut pos = if s.get(1).is_some_and(|&c| c == s[0]) {
      (0, 1)
    } else {
      (0, 0)
    };

    let mut l;
    let mut r;

    for i in 1..(s.len() - 1) {
      if s[i] == s[i + 1] {
        l = i;
        r = i + 1;

        while l > 0 && r + 1 < s.len() && s[l - 1] == s[r + 1] {
          l -= 1;
          r += 1;
        }

        if r - l > pos.1 - pos.0 {
          pos = (l, r);
        }
      }

      if s[i - 1] == s[i + 1] {
        l = i - 1;
        r = i + 1;

        while l > 0 && r + 1 < s.len() && s[l - 1] == s[r + 1] {
          l -= 1;
          r += 1;
        }

        if r - l > pos.1 - pos.0 {
          pos = (l, r);
        }
      }
    }

    s[pos.0..=pos.1].iter().map(|&c| c as char).collect()
  }
}
