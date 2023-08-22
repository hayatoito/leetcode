# @leetup=info id=212 lang=python3 slug=word-search-ii



# @leetup=code



class TrieNode:
    def __init__(self):
        self.children = {}
        self.is_word = False


class Trie:
    def __init__(self):
        self.root = TrieNode()

    def add_word(self, word: str) -> None:
        node = self.root
        for c in word:
            if c not in node.children:
                node.children[c] = TrieNode()
            node = node.children[c]
        node.is_word = True


class Solution:
    def findWords(self, board: list[list[str]], words: list[str]) -> list[str]:

        R = len(board)
        C = len(board[0])
        res = []

        trie = Trie()
        for word in words:
            trie.add_word(word)

        def dfs(r: int, c: int, node: TrieNode):
            cur_char = board[r][c]
            if cur_char not in node.children:
                return
            child_node = node.children[cur_char]

            if child_node.is_word:
                path.append(cur_char)
                res.append("".join(path))
                path.pop()
                # To de-dup
                child_node.is_word = False

            for (dr, dc) in ((0, 1), (0, -1), (1, 0), (-1, 0)):
                (nr, nc) = (r + dr, c + dc)
                if not (0 <= nr < R and 0 <= nc < C):
                    continue
                if (nr, nc) in visited:
                    continue
                visited.add((nr, nc))
                path.append(cur_char)
                dfs(nr, nc, child_node)
                visited.remove((nr, nc))
                path.pop()

            return False

        for r in range(0, R):
            for c in range(0, C):
                visited = set()
                visited.add((r, c))
                path = []
                dfs(r, c, trie.root)

        return res


# @leetup=code
