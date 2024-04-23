/*!
268.丢失的数字
 */

pub fn missing_number(nums: Vec<i32>) -> i32 {
    (0..=nums.len() as i32).chain(nums).fold(0, |acc, i| { acc ^ i })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_number() {
        assert_eq!(missing_number(vec![3, 0, 1]), 2);
        assert_eq!(missing_number(vec![0, 1]), 2);
        assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
        assert_eq!(missing_number(vec![0]), 1);
    }
}
