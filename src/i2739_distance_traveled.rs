/*!
2739. 总行驶距离
 */


pub struct Solution;

impl Solution {
    pub fn distance_traveled(main_tank: i32, additional_tank: i32) -> i32 {
        10 * (main_tank + additional_tank.min((main_tank - 1) / 4))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_element() {
        assert_eq!(Solution::distance_traveled(5, 10), 60);
        assert_eq!(Solution::distance_traveled(1, 2), 10);
        assert_eq!(Solution::distance_traveled(9, 2), 110);
    }
}
