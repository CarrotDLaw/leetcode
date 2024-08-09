impl Solution {
  pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
      return Vec::new();
    }

    let key_map = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
    let mut combinations = ["".to_string()].to_vec();

    for digit in digits.bytes() {
      let mut new_combinations = Vec::new();
      for combination in combinations {
        for letter in key_map[(digit - b'2') as usize].bytes() {
          new_combinations.push(format!("{}{}", combination, letter as char));
        }
      }

      combinations = new_combinations;
    }

    combinations.iter().map(|s| s.to_string()).collect()
  }
}
