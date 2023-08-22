# @leetup=info id=1 lang=python3 slug=two-sum

# @leetup=code
class Solution:
    def twoSum(self, nums: list[int], target: int) -> list[int]:
        visited = {}
        for i, a in enumerate(nums):
            b = target - a
            if b in visited:
                return [i, visited[b]]
            visited[a] = i


# @leetup=code
