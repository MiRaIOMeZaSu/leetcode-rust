struct Solution {}

impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut maxLen: i32 = 0;
        let mut ans = 0;
        for rectangle in rectangles {
            let min = rectangle[0].min(rectangle[1]);
            if min == maxLen {
                ans += 1;
            } else if min > maxLen {
                maxLen = min;
                ans = 1;
            }
        }
        return ans;
    }
}