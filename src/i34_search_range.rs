/*!
34. 在排序数组中查找元素的第一个和最后一个位置

给你一个按照非递减顺序排列的整数数组 <code>nums</code>，和一个目标值 <code>target</code>。请你找出给定目标值在数组中的开始位置和结束位置。

如果数组中不存在目标值 <code>target</code>，返回 <code>[-1, -1]</code>。

你必须设计并实现时间复杂度为 <code>O(log n)</code> 的算法解决此问题。
 */

pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();
        return match len {
            0 => vec![-1, -1],
            1 => {
                if nums[0] == target {
                    vec![0, 0]
                } else {
                    vec![-1, -1]
                }
            }
            _ => {
                let mut l = 0;
                let mut r = len - 1;
                let mut t = -1i32;

                while l < r {
                    let m = (l + r) / 2;
                    if nums[m] == target {
                        t = m as i32;
                        break;
                    } else if nums[m] < target {
                        l = m + 1;
                    } else {
                        r = m;
                    }
                }

                if l == r {
                    if nums[l] == target {
                        t = l as i32;
                    }
                }

                if t == -1 {
                    vec![-1, -1]
                } else {
                    l = t as usize;
                    r = t as usize;
                    for i in (0..l).rev() {
                        if nums[i] == target {
                            l = i;
                        } else {
                            break;
                        }
                    }
                    for i in (r + 1)..len {
                        if nums[i] == target {
                            r = i;
                        } else {
                            break;
                        }
                    }

                    vec![l as i32, r as i32]
                }
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_range() {
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
        assert_eq!(Solution::search_range(vec![1, 4], 4), vec![1, 1]);
    }
}