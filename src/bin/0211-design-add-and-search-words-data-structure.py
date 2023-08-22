# @leetup=info id=211 lang=python3 slug=design-add-and-search-words-data-structure



# @leetup=code
class Node:
    def __init__(self):
        self.is_word = False
        self.children = {}


class WordDictionary:
    def __init__(self):
        self.root = Node()

    def addWord(self, word: str) -> None:
        node = self.root
        for c in word:
            if c not in node.children:
                node.children[c] = Node()
            node = node.children[c]
        node.is_word = True

    def search(self, word: str) -> bool:
        def dfs(i, node: Node) -> bool:
            if i == len(word):
                return node.is_word

            c = word[i]
            if c == ".":
                for child in node.children.values():
                    if dfs(i + 1, child):
                        return True
                return False
            if c not in node.children:
                return False
            return dfs(i + 1, node.children[c])

        return dfs(0, self.root)


# @leetup=code
