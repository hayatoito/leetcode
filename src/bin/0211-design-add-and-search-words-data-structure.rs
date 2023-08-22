// @leetup=info id=211 lang=rust slug=design-add-and-search-words-data-structure

// @leetup=code

use std::collections::*;

// Trie
struct Node {
    is_word: bool,
    children: HashMap<char, Node>,
}

impl Node {
    fn new() -> Self {
        Node {
            is_word: false,
            children: Default::default(),
        }
    }
}

struct WordDictionary {
    root: Node,
}

impl WordDictionary {
    fn new() -> Self {
        Self { root: Node::new() }
    }

    fn add_word(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(|| Node::new());
        }
        node.is_word = true;
    }

    fn search(&self, word: String) -> bool {
        fn dfs(word: &[char], node: &Node) -> bool {
            if word.is_empty() {
                node.is_word
            } else if word[0] == '.' {
                node.children
                    .values()
                    .find(|child| dfs(&word[1..], child))
                    .is_some()
            } else {
                match node.children.get(&word[0]) {
                    Some(child) => dfs(&word[1..], child),
                    None => false,
                }
            }
        }

        let word: Vec<char> = word.chars().collect();
        dfs(&word, &self.root)
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
// @leetup=code

fn main() {
    let mut obj = WordDictionary::new();
    obj.add_word("hello".to_owned());
    let _ret_2: bool = obj.search("hello".to_owned());
}
