impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        return s
            .chars()
            .rev()
            .skip_while(|&x| x.is_whitespace())
            .take_while(|&x| x.is_alphabetic())
            .count() as i32;
    }
}
