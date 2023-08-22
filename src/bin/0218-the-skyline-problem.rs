// @leetup=info id=218 lang=rust slug=the-skyline-problem

// @leetup=code

// https://leetcode.com/problems/the-skyline-problem/solutions/61193/short-java-solution/?envType=featured-list&envId=top-interview-questions

use std::collections::*;

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut height: Vec<(i32, i32)> = Vec::new();
        for b in buildings {
            // start of a building, height stored as negtive
            height.push((b[0], -b[2]));
            // end of a building, height stored as positive
            height.push((b[1], b[2]));
        }
        // Sort the height list
        height.sort();

        // Stores all the encountered buildings' heights in a sorted
        let mut height_map: BTreeMap<i32, i32> = BTreeMap::new();
        height_map.insert(0, 1);

        let mut previous_max_height = 0;

        for (x, h) in height {
            if h < 0 {
                // h < 0, that means it meets a new building, so add it to treeMap.
                *height_map.entry(-h).or_insert(0) += 1;
            } else {
                // h >=0, that means it has reached the end of the building, so remove it.
                match height_map.entry(h) {
                    btree_map::Entry::Occupied(mut entry) => {
                        if *entry.get() == 1 {
                            entry.remove_entry();
                        } else {
                            *entry.get_mut() -= 1;
                        }
                    }
                    btree_map::Entry::Vacant(_) => {
                        unreachable!()
                    }
                }
            }

            // The current max height in all encountered buildings.

            // [2023-06-25 Sun] BTreeMap::last_entry is unstable in leetcode.
            // let current_max_height = *height_map.last_entry().unwrap().key();
            let current_max_height = *height_map.iter().rev().next().unwrap().0;

            // if the max height is different from the previous one,
            // that means a critical point is met, add to result list
            if current_max_height != previous_max_height {
                res.push(vec![x, current_max_height]);
                previous_max_height = current_max_height;
            }
        }
        res
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::get_skyline;
}
