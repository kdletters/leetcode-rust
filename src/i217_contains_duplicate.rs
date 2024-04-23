/*!
217.存在重复元素
 */

pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
    nums.sort_unstable();
    let mut i = 0;
    for x in &nums {
        if i + 1 < nums.len() {
            if nums[i + 1] == *x {
                return true;
            }
        }

        i += 1;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
    }
}
