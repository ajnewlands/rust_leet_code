pub mod hamming_weight {
    use std::mem::size_of;

    pub struct Solution {}

    impl Solution {
        pub fn hamming_weight(n: i32) -> i32 {
            let mut ones = 0;

            for i in 0..size_of::<i32>()
            {
               
                if ( (n>>i) & 1) == 1
                {
                    ones+=1;
                }
            }   
            return ones;
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn count_ones_in_eight()
    {
        assert_eq!(hamming_weight::Solution::hamming_weight(8), 1);
    }


    #[test]
    fn count_ones_in_thirteen()
    {
        assert_eq!(hamming_weight::Solution::hamming_weight(13), 3);
    }
}
