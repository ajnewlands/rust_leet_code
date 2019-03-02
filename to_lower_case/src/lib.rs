impl Solution {
    pub fn to_lower_case(str: String) -> String {
        let mut lower: String = str.clone();
        lower.make_ascii_lowercase();
        return lower;
    }
}
