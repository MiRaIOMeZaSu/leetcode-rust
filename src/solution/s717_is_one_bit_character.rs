struct Solution {}

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let size = bits.len();
        if size == 1 {
            return true;
        }
        let mut start: Vec<bool> = vec![false; size];
        // 表示可以从0的下标开始
        start[0] = true;
        for i in 0..size {
            if start[i] {
                if bits[i] == 1 {
                    if i + 2 < size{
                        start[i + 2] = true;
                    }
                } else {
                    if i + 1 < size {
                        start[i + 1] = true;
                    }
                }
            }
        }
        if bits[size - 2] == 1 && start[size - 2] {
            return false;
        }
        return true;
    }
}