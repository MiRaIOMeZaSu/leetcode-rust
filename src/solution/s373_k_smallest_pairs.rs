pub struct Solution {}

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        // 简单的指针法
        let mut ans: Vec<Vec<i32>> = vec![];
        let size1 = nums1.len();
        let size2 = nums2.len();
        let mut num1_indexs: Vec<usize> = vec![0; size1];
        let mut left = 0;
        let mut right = 0;
        let allSize = (size1 * size2) as i32;
        let mut size = k;
        if size > allSize {
            size = allSize;
        }
        for i in 0..size {
            let mut last_min_num1_index = 0;
            let mut temp: Vec<i32> = vec![];
            let mut min = i32::MAX;
            while right + 1 < size1 && (left > right || nums1[right] + nums2[num1_indexs[right]] > nums1[right + 1] + nums2[0]) {
                right += 1;
            }
            for j in left..right + 1 {
                let num1 = nums1[j];
                let num2 = nums2[num1_indexs[j]];
                let sum = num1 + num2;
                if sum < min {
                    temp = vec![num1, num2];
                    min = sum;
                    last_min_num1_index = j;
                }
            }
            num1_indexs[last_min_num1_index] += 1;
            if num1_indexs[last_min_num1_index] == size2 {
                left += 1;
            }
            ans.push(temp)
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
        assert_eq!(vec![vec![1, 3], vec![2, 3]], Solution::k_smallest_pairs(vec![1, 2], vec![3], 3));
        // assert_eq!(vec![vec![1, 2], vec![1, 4], vec![1, 6]], Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3));
        // assert_eq!(vec![vec![1, 1], vec![1, 1]], Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2));
        // assert_eq!(vec![vec![1, 3], vec![2, 3]], Solution::k_smallest_pairs(vec![1, 2], vec![3], 3));
        // assert_eq!(vec![vec![1, 3], vec![2, 3]], Solution::k_smallest_pairs(
        //     vec![1],
        //     vec![3,5,6,7,8,100],
        //     4));
    }
}
