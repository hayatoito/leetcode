from collections import defaultdict
from collections import deque


class Solution:
    def ladderLength(self, beginWord: str, endWord: str, wordList: list[str]) -> int:
        star_words = defaultdict(list)
        for word in wordList:
            for i in range(len(word)):
                star = word[:i] + "*" + word[i + 1 :]
                star_words[star].append(word)

        queue = deque()
        visited = set()
        queue.append((beginWord, 1))
        visited.add(beginWord)
        while queue:
            (word, level) = queue.popleft()
            for i in range(len(word)):
                star = word[:i] + "*" + word[i + 1 :]
                for next_word in star_words[star]:
                    if next_word == endWord:
                        return level + 1
                    if next_word in visited:
                        continue
                    queue.append((next_word, level + 1))
                    visited.add(next_word)
        return 0
