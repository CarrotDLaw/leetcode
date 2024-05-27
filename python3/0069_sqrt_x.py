class Solution:
    def mySqrt(self, x: int) -> int:
        l = 1
        r = x // 2 + 1

        while l <= r:
            m = l + (r - l) // 2
            m_squared = m * m

            if m_squared == x:
                return m
            elif m_squared < x:
                l = m + 1
            elif m_squared > x:
                r = m - 1

        return r
