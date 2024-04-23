/*!
219. 存在重复元素 II
 */

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    if k == 0 { return false; }
    if nums.len() == 1 { return false; }

    let mut set = std::collections::HashSet::new();
    let k = k as usize;
    for i in 0..nums.len() {
        if i > k {
            set.remove(&nums[i - k - 1]);
        }
        if !set.insert(nums[i]) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_nearby_duplicate() {
        assert_eq!(contains_nearby_duplicate(vec![1, 2, 2, 3], 3), true);
        assert_eq!(contains_nearby_duplicate(vec![1, 2, 1], 1), false);
        assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1], 3), true);
        assert_eq!(contains_nearby_duplicate(vec![1, 0, 1, 1], 1), true);
        assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2), false);
    }
}