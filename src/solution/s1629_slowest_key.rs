pub struct Solution {}

impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut curr = release_times[0];
        let mut index = 0;
        let chars: Vec<char> = keys_pressed.chars().collect();
        let size = release_times.len();
        for i in 1..size {
            let diff = release_times[i] - release_times[i - 1];
            if diff > curr  {
                curr = diff;
                index = i;
            }else if diff == curr && chars[i] > chars[index]{
                curr = diff;
                index = i;
            }
        }
        return chars[index];
    }
}