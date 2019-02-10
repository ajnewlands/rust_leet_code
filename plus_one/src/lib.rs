pub mod plus_one_mod {

    use std::collections::VecDeque;

    pub struct Solution {}

    impl Solution {
        pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
            let mut ret = VecDeque::with_capacity(digits.len() +1 );
            let mut carry = 1;

            for position in (0..digits.len()).rev()
            {
                let val =  (digits[position] + carry) % 10;
                if val != 0 && carry == 1
                {
                    carry = 0;
                }
                ret.push_front(val);            
            }
            if carry==1
            {
                ret.push_front(carry);
            }

            return Vec::from(ret);
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_no_carry()
    {
        assert_eq!(plus_one_mod::Solution::plus_one( vec![1,2,3,4] ), vec![1,2,3,5] );
    }

    #[test]
    fn test_with_carry()
    {
        assert_eq!(plus_one_mod::Solution::plus_one( vec![9,9,9,9] ), vec![1,0,0,0,0] );
    }
}
