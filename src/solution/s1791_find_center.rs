use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        // 是否会存在重复的边?
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut max: i32 = 0;
        let mut ans: i32 = 0;
        for edge in edges {
            for point in edge {
                let count = map.entry(point).or_insert(0);
                *count += 1;
                if *count > max {
                    max = *count;
                    ans = point;
                }
            }
        }
        return ans;
    }
}