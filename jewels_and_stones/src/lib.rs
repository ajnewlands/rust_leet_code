use std::collections::HashSet;

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let jewels: HashSet<_> = j.chars().collect();
        return s.chars().filter(|x| jewels.contains(&x)).count() as i32;
    }
}
