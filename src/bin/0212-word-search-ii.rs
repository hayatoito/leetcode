// @leetup=info id=212 lang=rust slug=word-search-ii

// @leetup=code

// [2023-04-12 Wed] Timeout occurs for testcase 63.
// [2023-04-12 Wed] Passed after optimizing; don't use HashSet nor HashMap.

struct Node {
    children: [Option<Box<Node>>; 26],
    word: Option<String>,
}

impl Node {
    fn new() -> Self {
        Node {
            children: Default::default(),
            word: None,
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
        for c in word.as_bytes() {
            node = node.children[(c - b'a') as usize].get_or_insert_with(|| Box::new(Node::new()));
        }
        node.word = Some(word);
    }
}

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::new();
        for word in words {
            trie.insert(word);
        }

        let mut res = Vec::new();

        for r in 0..board.len() {
            for c in 0..board[0].len() {
                Solution::dfs(r, c, &mut trie.root, &mut board, &mut res);
            }
        }
        res
    }

    fn dfs(r: usize, c: usize, node: &mut Node, board: &mut [Vec<char>], res: &mut Vec<String>) {
        let cur_char = board[r][c];
        if let Some(node) = &mut node.children[(cur_char as u8 - b'a') as usize] {
            if let Some(word) = node.word.take() {
                res.push(word);
            }
            board[r][c] = '*';

            for (dr, dc) in [(0, 1), (0, !0), (1, 0), (!0, 0)] {
                let (r, c) = (r.wrapping_add(dr), c.wrapping_add(dc));
                if r < board.len() && c < board[0].len() && board[r][c] != '*' {
                    Self::dfs(r, c, node.as_mut(), board, res);
                }
            }
            board[r][c] = cur_char;
        }
    }
}

// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::find_words;
}
