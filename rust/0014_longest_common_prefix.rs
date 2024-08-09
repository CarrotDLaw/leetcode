impl Solution {
  pub fn longest_common_prefix(strs: Vec<String>) -> String {
    strs
      .into_iter()
      .reduce(|acc, s| {
        acc
          .bytes()
          .zip(s.bytes())
          .take_while(|(x, y)| x == y)
          .map(|(x, _)| x as char)
          .collect()
      })
      .unwrap_or_else(|| "".to_string())
  }
}
