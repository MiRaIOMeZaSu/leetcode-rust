pub struct Solution {}

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        // 简单的指针法
        let mut ans: Vec<Vec<i32>> = vec![];
        let size1 = nums1.len();
        let size2 = nums2.len();
        let mut i1 = 0;
        let mut i2 = 0;
        for i in 0..k {
            ans.push(vec![nums1[i1], nums2[i2]]);
            if i1 + 1 < size1 {
                if i2 + 1 < size2 {
                    if nums1[i1 + 1] < nums2[i2 + 1] {
                        i1 += 1;
                    } else {
                        i2 += 1;
                    }
                } else {
                    i1 += 1;
                }
            } else if i2 + 1 < size2{
                i2 += 1;
            }else {
                return ans;
            }
        }
        return ans;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_373() {
        // assert_eq!(vec![vec![1, 2], vec![1, 4], vec![1, 6]], Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3));
        // assert_eq!(vec![vec![1, 1], vec![1, 1]], Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2));
        assert_eq!(vec![vec![1, 3], vec![2, 3]], Solution::k_smallest_pairs(vec![1, 2], vec![3], 3));
    }
}