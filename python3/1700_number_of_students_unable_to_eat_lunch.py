class Solution:
    def countStudents(self, students: List[int], sandwiches: List[int]) -> int:
        counts = [0, 0]
        
        for s in students:
            counts[s] += 1

        for s in sandwiches:
            counts[s] -= 1

            if counts[s] < 0:
                return counts[1 - s]

        return 0
