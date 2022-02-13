struct Solution {}

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        // balloon
        let pivot: u8 = 'a' as u8;
        let mut ans = i32::MAX;
        let mut counts: Vec<i32> = vec![0; 26];
        for i in text.as_bytes() {
            counts[(i - 'a' as u8) as usize] += 1;
        }
        ans = ans.min(counts[('b' as u8 - pivot) as usize]);
        ans = ans.min(counts[('a' as u8 - pivot) as usize]);
        ans = ans.min(counts[('n' as u8 - pivot) as usize]);
        ans = ans.min(counts[('l' as u8 - pivot) as usize] / 2);
        ans = ans.min(counts[('o' as u8 - pivot) as usize] / 2);
        return ans;
    }
}