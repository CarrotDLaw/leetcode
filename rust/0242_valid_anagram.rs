impl Solution {
  pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
      return false;
    }

    let mut count = [0; 26];

    for (&u, &v) in s.as_bytes().iter().zip(t.as_bytes().iter()) {
      count[u as usize - 97] += 1;
      count[v as usize - 97] -= 1;
    }

    for c in count {
      if c != 0 {
        return false;
      }
    }

    true
  }
}
