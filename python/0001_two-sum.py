class Solution(object):
    def twoSum(self, nums: list[int], target: int) -> list[int]:
        visited = {}
        for i, a in enumerate(nums):
            b = target - a
            if b in visited:
                return [i, visited[b]]
            visited[a] = i


# print(Solution().twoSum([2, 7, 11, 15], 9))
