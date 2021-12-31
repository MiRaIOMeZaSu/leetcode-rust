use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

pub struct Solution {}

#[derive(PartialEq, Eq)]
pub struct SmallI32(i32);

impl PartialOrd for SmallI32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for SmallI32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl SmallI32 {
    fn new(value: i32) -> SmallI32 {
        SmallI32(value)
    }
}


impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        // 使用池子
        let nums = [2, 3, 5];
        let mut heap: BinaryHeap<SmallI32> = BinaryHeap::new();
        let mut set: HashSet<i32> = HashSet::new();
        let mut list: Vec<i32> = vec![];
        let n_usize = usize::from(n as u16);
        list.push(1);
        let mut i: usize;
        let mut index: usize = 0;
        while list.len() < n_usize {
            // 开始计算
            i = 0;
            if heap.is_empty() || list[index] * 2 < heap.peek().unwrap().0 {
                while i < nums.len() {
                    let temp = list[index] * nums[i];
                    if temp < 0 {
                        break;
                    }
                    heap.push(SmallI32::new(temp));
                    i += 1;
                }
                index += 1;
            }
            // while !heap.is_empty() && heap.peek().unwrap().0 < 0 {
            //     heap.pop();
            // }
            let val: i32 = heap.peek().unwrap().0;
            if !set.contains(&val) {
                list.push(val);
                set.insert(val);
            }
            heap.pop();
        }
        return list[n_usize - 1];
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_264() {
        // assert_eq!(12, Solution::nth_ugly_number(10));
        // assert_eq!(1, Solution::nth_ugly_number(1));
        assert_eq!(2123366400, Solution::nth_ugly_number(1690));
    }
}