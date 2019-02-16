// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;


impl Solution {
    pub fn check_depth(node: &Rc<RefCell<TreeNode>>) -> (bool, usize) {

        let depth_l = match &node.borrow().left {
            None =>  (true, 0),
            Some(left) => Solution::check_depth(&left),
        };
        
        let depth_r = match &node.borrow().right {
            None =>  (true, 0),
            Some(right) => Solution::check_depth(&right),
        };        
        
        let mut balanced = depth_l.0 & depth_r.0;
        if (depth_l.1 as i32 - depth_r.1 as i32).abs() > 1
        {
            balanced = false;
        }
        
        if (depth_l.1 > depth_r.1)
        {
            return (balanced, 1 + depth_l.1);
        }
        else
        {
            return (balanced, 1 + depth_r.1);
        }
    }
    
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {        
        let res = match root {
            Some(node) => Solution::check_depth(&node),
            None => (true,0),
        };
        return res.0;
    }
}