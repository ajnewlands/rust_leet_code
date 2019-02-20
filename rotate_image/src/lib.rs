struct Solution {}

impl Solution {
    pub unsafe fn swap(a: *mut i32, b: *mut i32) {
        *a = *a ^ *b;
        *b = *a ^ *b;
        *a = *a ^ *b;
    }
    
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let length = matrix.len();
        
        for x in 0..length{
            for y in 0..x {
                unsafe{
                    Solution::swap(&mut matrix[y][x] as *mut i32, 
                        &mut matrix[x][y] as *mut i32);

                }
            }
        }
        for x in 0..length{
            matrix[x].reverse();
        }
    }
}

#[cfg(test)]
mod tests {
use super::*;
    #[test]
    fn it_works() {
		let mut input = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
		Solution::rotate(&mut input);
		let expected = vec![vec![7,4,1],vec![8,5,2],vec![9,6,3]];
        assert_eq!(input, expected);
    }
}
