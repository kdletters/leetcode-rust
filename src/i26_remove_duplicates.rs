/*!
26. 删除有序数组中的重复项
 */

pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 { return 1; }
        let mut l = 0;

        for r in 1..nums.len() {
            if nums[l] != nums[r] {
                l += 1;
                nums[l] = nums[r];
            }
        }

        return (l + 1) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
    }
}