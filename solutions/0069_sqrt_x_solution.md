# Intuition
<!-- Describe your first thoughts on how to solve this problem. -->

# Approach
<!-- Describe your approach to solving the problem. -->
Binary search.

# Complexity
- Time complexity: $O(\log n)$
<!-- Add your time complexity here, e.g. $$O(n)$$ -->

- Space complexity: $O(1)$
<!-- Add your space complexity here, e.g. $$O(n)$$ -->

# Code
```rs
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        use std::cmp::Ordering;

        let mut l = 1;
        let mut r = x / 2 + 1;

        while l <= r {
            let m = l + (r - l) / 2;

            match m.checked_mul(m) {
                Some(m_squared) => match m_squared.cmp(&x) {
                    Ordering::Less => l = m + 1,
                    Ordering::Equal => return m,
                    Ordering::Greater => r = m - 1,
                },
                None => r = m - 1,
            }
        }

        r
    }
}
```
