struct Solution{}

impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        // 若长度不等,则最长的为更长的字符串
        let a_size =a.len();
        let b_size = b.len();
        if a_size > b_size {
            return a_size as i32;
        } else if b_size > a_size {
            return b_size as i32;
        }
        if a.eq(&b){
            return -1;
        }
        return a_size as i32;
    }
}