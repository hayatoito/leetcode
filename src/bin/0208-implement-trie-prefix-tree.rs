// @leetup=info id=208 lang=rust slug=implement-trie-prefix-tree

// @leetup=code

use std::collections::*;

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

struct Trie {
    root: Node,
}

impl Trie {
    fn new() -> Self {
        Self { root: Node::new() }
    }

    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(|| Node::new());
        }
        node.is_word = true;
    }

    fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            if let Some(n) = node.children.get(&c) {
                node = n;
            } else {
                return false;
            }
        }
        node.is_word == true
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for c in prefix.chars() {
            if let Some(n) = node.children.get(&c) {
                node = n;
            } else {
                return false;
            }
        }
        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
// @leetup=code

fn main() {
    let mut trie = Trie::new();
    trie.insert("hello".to_owned());
    let _ = trie.search("hello".to_owned());
    let _ = trie.starts_with("hel".to_owned());
}
