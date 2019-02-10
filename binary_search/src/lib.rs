pub mod binary_search{
    pub struct Solution {}

    impl Solution {
        pub fn search(nums: Vec<i32>, target: i32) -> i32 {
            let mut begin =0;
            let mut last = (nums.len() -1) as i32;
        
            while begin <= last
            {
                let mid = ((begin + last)/2) as usize;
            
                if nums[mid] < target
                {
                    begin = mid as i32 + 1;
                }
                else if nums[mid] > target
                {
                    last = mid as i32 -1;
                }
                else if nums[mid] == target
                {
                    return mid as i32;
                }
                else
                {
                    return -1;
                }
            }
            return -1;

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_single_item_vec() {
        assert_eq!( binary_search::Solution::search(vec![1],1), 0);
    }

    #[test]
    fn find_single_missing_item_vec() {
        assert_eq!( binary_search::Solution::search(vec![2],1), -1);
    }

    #[test]
    fn find_multi_item_vec() {
        assert_eq!( binary_search::Solution::search(vec![1,2,3,4,5,6],4), 3);
    }

    #[test]
    fn find_multi_item_missing_vec() {
        assert_eq!( binary_search::Solution::search(vec![1,2,3,5,6,7],4), -1);
    }
}
