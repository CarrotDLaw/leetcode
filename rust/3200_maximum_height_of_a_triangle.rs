impl Solution {
  pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
    use core::cmp::max;

    fn max_height(red: i32, blue: i32) -> i32 {
      let (mut red, mut blue) = (red, blue);
      let mut h = 0;
      let mut i = 1;
  
      loop {
        match i % 2 {
          0 => {
            if red < i {
              break;
            }
  
            red -= i;
          }
          1 => {
            if blue < i {
              break;
            }
  
            blue -= i;
          }
          _ => panic!(),
        }
  
        h += 1;
        i += 1;
      }
  
      h
    }
  
    max(max_height(red, blue), max_height(blue, red))
  }
}
