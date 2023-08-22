// @leetup=info id=341 lang=rust slug=flatten-nested-list-iterator

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

// @leetup=code

struct NestedIterator {
    flat_list: Vec<i32>,
    index: usize,
}

fn flatten(list: Vec<NestedInteger>) -> Vec<i32> {
    let mut v = vec![];
    for i in list {
        match i {
            NestedInteger::Int(x) => v.push(x),
            NestedInteger::List(xs) => v.append(&mut flatten(xs)),
        }
    }
    v
}

impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        Self {
            flat_list: flatten(nested_list),
            index: 0,
        }
    }

    fn next(&mut self) -> i32 {
        let a = self.flat_list[self.index];
        self.index += 1;
        a
    }

    fn has_next(&self) -> bool {
        self.index < self.flat_list.len()
    }
}

// @leetup=code

fn main() {
    let _ = NestedIterator::new;
    let _ = NestedIterator::next;
    let _ = NestedIterator::has_next;
}
