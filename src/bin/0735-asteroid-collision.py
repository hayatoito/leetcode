# @leetup=info id=735 lang=python3 slug=asteroid-collision



from typing import Optional

# @leetup=code
class Solution:
    def asteroidCollision(self, asteroids: list[int]) -> list[int]:
        res = []
        for s in asteroids:
            while res and s < 0 < res[-1]:
                if res[-1] == -s:
                    res.pop()
                    s = 0
                elif res[-1] > -s:
                    s = 0
                else:
                    res.pop()
            if s:
                res.append(s)
        return res


# @leetup=code
