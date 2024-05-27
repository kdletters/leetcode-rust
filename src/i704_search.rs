/*!
704. 二分查找
 */

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut i = -1i32;
        let mut j = nums.len() as i32;

        // 不变式: (i, j)
        while i + 1 < j {
            let m = (i + j) / 2;
            if nums[m as usize] == target {
                return m;
            } else if nums[m as usize] < target { // m 是 下界
                i = m;
            } else { // nums[m] > target, m 是上界
                // 不变式: [i, j)
                j = m;
            }
        }
        
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(Solution::search(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 5), 4);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}