const ROMAN_NUMERALS: &[(&str, i32)] = &[
  ("M", 1000),
  ("CM", 900),
  ("D", 500),
  ("CD", 400),
  ("C", 100),
  ("XC", 90),
  ("L", 50),
  ("XL", 40),
  ("X", 10),
  ("IX", 9),
  ("V", 5),
  ("IV", 4),
  ("I", 1),
];

impl Solution {
  pub fn int_to_roman(mut num: i32) -> String {
    ROMAN_NUMERALS
      .iter()
      .map(|&(rn, n)| {
        let (d, m) = (num / n, num % n);
        num = m;
        rn.repeat(d as usize)
      })
      .collect()
  }
}
