/*!
169.多数元素

利用众数的性质，利用摩尔投票算法，遍历数组，如果当前元素与候选元素相同，则计数器加1，否则减1，当计数器为0时，候选元素更新为当前元素。
 */

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut candidate = 0;
    let mut count = 0;
    for x in nums {
        if count == 0 {
            candidate = x;
        }

        if x == candidate {
            count += 1;
        } else {
            count -= 1;
        }
    }
    candidate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_element() {
        assert_eq!(majority_element(vec![3, 2, 3]), 3);
        assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
