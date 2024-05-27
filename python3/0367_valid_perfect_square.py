class Solution:
    def isPerfectSquare(self, num: int) -> bool:
        l = 1
        r = num // 2 + 1

        while l <= r:
            m = l + (r - l) // 2
            m_squared = m * m

            if m_squared == num:
                return True
            elif m_squared < num:
                l = m + 1
            elif m_squared > num:
                r = m - 1

        return False
