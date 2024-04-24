/*!
377.组合总和 Ⅳ
 */
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, start: i32, ans: &mut i32) -> (i32, bool) {
            if let Some(x) = node {
                let x = x.borrow();
                let (l_len, l_found) = dfs(x.left.as_ref(), start, ans);
                let (r_len, r_found) = dfs(x.right.as_ref(), start, ans);
                if x.val == start {
                    // 计算子树 start 的最大深度
                    // 注意这里和方法一的区别，max 后面没有 +1，所以算出的也是最大深度
                    *ans = l_len.max(r_len);
                    return (1, true); // 找到了 start
                }
                if l_found || r_found {
                    // 只有在左子树或右子树包含 start 时，才能更新答案
                    *ans = (*ans).max(l_len + r_len); // 两条链拼成直径
                    // 保证 start 是直径端点
                    return ((if l_found { l_len } else { r_len }) + 1, true);
                }
                return (l_len.max(r_len) + 1, false);
            }
            (0, false)
        }
        let mut ans = 0;
        dfs(root.as_ref(), start, &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amount_of_time() {

    }
}
