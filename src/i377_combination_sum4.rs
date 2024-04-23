/*!
377.组合总和 Ⅳ
 */

pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    let mut dp = vec![0; (target + 1) as usize];
    dp[0] = 1;
    for i in 1..=target {
        for num in &nums {
            if *num <= i {
                dp[i as usize] += dp[(i - num) as usize];
            }
        }
    }
    dp[target as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum4() {
        assert_eq!(combination_sum4(vec![1, 2, 3], 4), 7);
        assert_eq!(combination_sum4(vec![9], 3), 0);
    }
}
