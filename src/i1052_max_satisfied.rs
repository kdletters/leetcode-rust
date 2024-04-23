/*!
1052.爱生气的书店老板

滑动窗口在移动的时候除了首尾以外，中间的元素是不会改变的，所以可以通过首尾增量的方式来减少循环次数。
 */

pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    let mut count = 0;
    let n = customers.len();
    let m = minutes as usize;
    let mut sum = 0;
    let mut max_sum = 0;

    for i in 0..m {
        count += customers[i] * (1 - grumpy[i]);
        sum += customers[i] * grumpy[i];
    }
    max_sum = sum;
    for i in m..n {
        count += customers[i] * (1 - grumpy[i]);
        sum += customers[i] * grumpy[i];
        sum -= customers[i - m] * grumpy[i - m];
        if sum > max_sum {
            max_sum = sum;
        }
    }
    count + max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_satisfied() {
        assert_eq!(max_satisfied(vec![1, 0, 1, 2, 1, 1, 7, 5], vec![0, 1, 0, 1, 0, 1, 0, 1], 3), 16);
        assert_eq!(max_satisfied(vec![1], vec![0], 1), 1);
    }
}
