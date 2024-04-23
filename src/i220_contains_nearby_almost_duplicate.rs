/*!
220. 存在重复元素 III

将数据和下标组织在一起然后进行排序，然后遍历排序后的数组，如果两个元素之间的下标差小于k，且两个元素之间的绝对值差小于t，则返回true。
 */

pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
    let mut nums: Vec<_> = nums
        .iter()
        .enumerate()
        .map(|(idx, &x)| (x as i64, idx as i32))
        .collect();

    nums.sort_unstable();

    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[j].0 - nums[i].0 > t as i64 {
                break;
            }
            if (nums[i].1 - nums[j].1).abs() <= k {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_nearby_almost_duplicate() {
        assert_eq!(contains_nearby_almost_duplicate(vec![-3, 3, -6], 2, 3), true);
        assert_eq!(contains_nearby_almost_duplicate(vec![-3, 3], 2, 4), false);
        assert_eq!(contains_nearby_almost_duplicate(vec![-1, -1], 1, 0), true);
        assert_eq!(contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0), true);
        assert_eq!(contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3), false);
    }
}