# @leetup=info id=15 lang=python3 slug=3sum



# @leetup=code
class Solution:
    def threeSum(self, nums: list[int]) -> list[list[int]]:
        nums.sort()
        N = len(nums)
        s = set()

        for i in range(N):
            j = i + 1
            k = N - 1
            while j < k:
                sum = nums[i] + nums[j] + nums[k]
                if sum == 0:
                    s.add((nums[i], nums[j], nums[k]))
                    j += 1
                    k -= 1
                elif sum < 0:
                    j += 1
                else:
                    k -= 1

        return list(s)


# @leetup=code
