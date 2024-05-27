/*!
35. 搜索插入位置
 */

pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
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

        i + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}