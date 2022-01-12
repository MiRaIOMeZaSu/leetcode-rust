pub struct Solution {}

fn min(a: i32, b: i32) -> i32 {
    if a > b {
        return b;
    }
    return a;
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }
    return b;
}

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        // 简单的前后比较
        let size = nums.len();
        let mut frontMins: Vec<i32> = vec![];
        let mut backMaxs: Vec<i32> = vec![0; size];
        let mut presMin: i32 = i32::MAX;
        for i in 0..size {
            presMin = min(presMin, nums[i]);
            frontMins.push(presMin);
        }
        let mut i = size - 1;
        let mut presMax: i32 = i32::MIN;
        loop {
            presMax = max(presMax, nums[i]);
            backMaxs[i] = presMax;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        for i in 1..size - 1 {
            if frontMins[i - 1] < nums[i] && nums[i] < backMaxs[i + 1] {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_334() {
        // assert_eq!(true,Solution::increasing_triplet(vec![1,2,3,4,5]));
        assert_eq!(true, Solution::increasing_triplet(vec![20, 100, 10, 12, 5, 13]));
    }
}