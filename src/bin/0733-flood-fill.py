# @leetup=info id=733 lang=python3 slug=flood-fill



from typing import Optional

# @leetup=code
class Solution:
    def floodFill(
        self, image: list[list[int]], sr: int, sc: int, color: int
    ) -> list[list[int]]:
        R = len(image)
        C = len(image[0])

        src_color = image[sr][sc]
        if src_color == color:
            return image

        def dfs(r, c):
            if not (0 <= r < R and 0 <= c < C):
                return
            if image[r][c] != src_color:
                return
            image[r][c] = color
            dfs(r, c + 1)
            dfs(r, c - 1)
            dfs(r + 1, c)
            dfs(r - 1, c)

        dfs(sr, sc)
        return image


# @leetup=code
