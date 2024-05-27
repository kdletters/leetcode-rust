/*!
744. 寻找比目标字母大的最小字母
 */

pub struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        if &letters[letters.len() - 1] <= &target || &letters[0] > &target {
            return letters[0];
        }

        let mut i = 0i32;
        let mut j = letters.len() as i32 - 1;

        // 不变式: (i, j)
        while i < j {
            let m = (i + j) / 2;
            if letters[m as usize] <= target {
                i = m + 1;
            } else { // nums[m] > target, m 是上界
                // 不变式: [i, j)
                j = m;
            }
        }

        letters[i as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_greatest_letter() {
        // assert_eq!(Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'a'), 'c');
        assert_eq!(Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'c'), 'f');
        // assert_eq!(Solution::next_greatest_letter(vec!['x', 'x', 'y', 'y'], 'z'), 'x');
    }
}