# @leetup=info id=207 lang=python3 slug=course-schedule



# @leetup=code

from collections import deque


class Solution:
    def canFinish(self, numCourses: int, prerequisites: list[list[int]]) -> bool:
        graph = [[] for _ in range(numCourses)]
        in_degree = [0] * numCourses
        for pre in prerequisites:
            graph[pre[1]].append(pre[0])
            in_degree[pre[0]] += 1

        visited = set()
        q = deque()
        for i, n in enumerate(in_degree):
            if n == 0:
                q.append(i)

        while q:
            a = q.popleft()
            if a in visited:
                continue
            visited.add(a)
            for b in graph[a]:
                in_degree[b] -= 1
                if in_degree[b] == 0:
                    q.append(b)

        return numCourses == len(visited)


# @leetup=code
