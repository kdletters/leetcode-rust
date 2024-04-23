/*!
229.多数元素 II
 */

use std::collections::HashSet;

/// 通过对数进行排序，然后遍历数组，计算出出现次数大于n/3的数
pub fn majority_element(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();
    let mut result = HashSet::new();
    let count = nums.len() / 3;
    let mut l = 0;
    let mut cur = nums[0];
    for i in 0..nums.len() {
        let temp = nums[i];
        if temp != cur {
            if (i - l) > count {
                result.insert(cur);
            }
            l = i;
            cur = temp;
        }
    }

    if (nums.len() - l) > count {
        result.insert(cur);
    }

    let mut result = result.iter().map(|x| *x).collect::<Vec<i32>>();
    result.sort_unstable();
    result
}

/// 摩尔投票算法
/// 三个数不同则抵消，直至最后剩下2个或者1个数，然后检测他的数量是否符合条件
pub fn majority_element_1(mut nums: Vec<i32>) -> Vec<i32> {
    let mut e_1 = i32::MIN;
    let mut v_1 = 0;
    let mut e_2 = i32::MAX;
    let mut v_2 = 0;

    for &num in &nums {
        if num == e_1 {
            v_1 += 1;
        } else if num == e_2 {
            v_2 += 1;
        } else if v_1 == 0 {
            e_1 = num;
            v_1 += 1;
        } else if v_2 == 0 {
            e_2 = num;
            v_2 += 1;
        } else {
            v_1 -= 1;
            v_2 -= 1;
        }
    }

    let mut maj = Vec::with_capacity(2);
    for e in [e_1, e_2] {
        if nums.iter().filter(|&&x| x == e).count() > nums.len() / 3 {
            maj.push(e);
        }
    }
    maj
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_element() {
        assert_eq!(majority_element(vec![3, 2, 3]), vec![3]);
        assert_eq!(majority_element(vec![1]), vec![1]);
        assert_eq!(majority_element(vec![1, 2]), vec![1, 2]);
    }
}
