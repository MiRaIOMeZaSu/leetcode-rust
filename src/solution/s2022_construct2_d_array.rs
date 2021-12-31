pub struct Solution {}

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        let m_usize: usize = usize::from(m as u16);
        let n_usize: usize = usize::from(n as u16);
        let size: usize = original.len();
        if usize::from((m * n) as u16) != size {
            return ans;
        }
        // 创建
        let mut index: usize = 0;
        let mut i: usize = 0;
        let mut j: usize = 0;
        while i < m_usize {
            j = 0;
            let mut temp: Vec<i32> = vec![];
            while j < n_usize {
                temp.push(original[index]);
                j += 1;
                index += 1;
            }
            ans.push(temp);
            i += 1;
        }
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2022() {
        Solution::construct2_d_array(vec![1,2,3,4],2,2);
    }
}