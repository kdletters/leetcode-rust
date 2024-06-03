/*!
1351. 统计有序矩阵中的负数
 */

pub struct Solution;

impl Solution {
    // pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    //     let mut count = 0;
    //     for row in grid.iter() {
    //         if row[0] < 0 {
    //             count += row.len() as i32;
    //         } else {
    //             for &num in row.iter() {
    //                 if num < 0 {
    //                     count += 1;
    //                 }
    //             }
    //         }
    //     }
    // 
    //     count
    // }

    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut num = 0;
        let m = grid[0].len() as i32;
        let mut pos = m - 1;

        for x in grid {
            let mut i = pos;
            loop {
                if i < 0 {
                    break;
                }
                if x[i as usize] >= 0 {
                    if i + 1 < m {
                        pos = i;
                        num += m - pos - 1;
                    }

                    break;
                }
                i -= 1;
            }

            if i == -1 {
                num += m;
                pos = -1;
            }
        }

        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_negatives() {
        assert_eq!(Solution::count_negatives(vec![vec![4, 3, 2, -1], vec![3, 2, 1, -1], vec![1, 1, -1, -2], vec![-1, -1, -2, -3]]), 8);
        assert_eq!(Solution::count_negatives(vec![vec![3, 2], vec![1, 0]]), 0);
    }
}