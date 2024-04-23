/*!
88.合并两个有序数组

利用第一个数组后半空白部分长度与第二个数组长度相同以及两个数组有序的特性，从第一个数组的结尾使用双指针逆向比较，将较大的元素优先填充到结尾，直到两个数组都遍历完。
 */

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut p1 = m - 1;
    let mut p2 = n - 1;
    let mut t = m + n - 1;
    while p1 >= 0 || p2 >= 0 {
        if p1 == -1 {
            nums1[t as usize] = nums2[p2 as usize];
            p2 -= 1;
        } else if p2 == -1 {
            nums1[t as usize] = nums1[p1 as usize];
            p1 -= 1;
        } else if nums1[p1 as usize] > nums2[p2 as usize] {
            nums1[t as usize] = nums1[p1 as usize];
            p1 -= 1;
        } else {
            nums1[t as usize] = nums2[p2 as usize];
            p2 -= 1;
        }

        t -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }
}
