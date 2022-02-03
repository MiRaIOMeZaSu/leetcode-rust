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
        let mut front_mins: Vec<i32> = vec![];
        let mut back_maxs: Vec<i32> = vec![0; size];
        let mut pres_min: i32 = i32::MAX;
        for i in 0..size {
            pres_min = min(pres_min, nums[i]);
            front_mins.push(pres_min);
        }
        let mut i = size - 1;
        let mut pres_max: i32 = i32::MIN;
        loop {
            pres_max = max(pres_max, nums[i]);
            back_maxs[i] = pres_max;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        for i in 1..size - 1 {
            if front_mins[i - 1] < nums[i] && nums[i] < back_maxs[i + 1] {
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