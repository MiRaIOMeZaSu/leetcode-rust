use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();
        let mut removed: HashSet<i32> = HashSet::new();
        let mut ans = 0;
        for num in nums {
            if !set.contains(&num) {
                set.insert(num);
                ans += num;
            } else {
                if !removed.contains(&num) {
                    ans -= num;
                    removed.insert(num);
                }
            }
        }
        return ans;
    }
}