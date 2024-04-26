/*!
1146. 快照数组
 */

struct SnapshotArray {
    snap_id: i32,
    data: Vec<Vec<(i32, i32)>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {
    fn new(length: i32) -> Self {
        let mut obj = SnapshotArray {
            snap_id: 0,
            data: vec![vec![]; length as usize],
        };

        obj
    }

    fn set(&mut self, index: i32, val: i32) {
        self.data[index as usize].push((self.snap_id, val));
    }

    fn snap(&mut self) -> i32 {
        let id = self.snap_id;
        self.snap_id += 1;
        id
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let arr = &self.data[index as usize];
        let i = Self::binary_search(arr, snap_id);
        if i == 0 {
            return 0;
        }
        return arr[i - 1].1;
    }

    fn binary_search(arr: &Vec<(i32, i32)>, snap_id: i32) -> usize {
        let mut low = 0;
        let mut high = arr.len();
        while low < high {
            let mid = low + ((high - low) / 2);
            let (x, y) = arr[mid];
            if x > snap_id + 1 || (x == snap_id + 1 && y >= 0) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        low
    }
}

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_array() {
        let mut obj = SnapshotArray::new(1);
        obj.set(0, 4);
        obj.set(0, 16);
        obj.set(0, 13);
        assert_eq!(obj.snap(), 0);
        assert_eq!(obj.get(0, 0), 13);
        assert_eq!(obj.snap(), 1);
    }
}
