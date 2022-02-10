struct Solution {}

struct Number {
    val: f64,
    molecular: i32,
    distribution: i32,
}

impl Solution {
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        // 本实现使用排序,使用哈希集合进行存在判断同样需要针对浮点数进行适配
        // n^2, n == 100
        let mut ans: Vec<String> = vec![];
        let mut floats: Vec<Number> = vec![];
        for i in 1..n {
            let i_f64: f64 = i as f64;
            for j in i + 1..n + 1 {
                let j_f64: f64 = j as f64;
                let val: f64 = i_f64 / j_f64;
                floats.push(Number {
                    val: val,
                    molecular: i,
                    distribution: j,
                });
            }
        }
        floats.sort_by(|a, b| a.val.partial_cmp(&b.val).unwrap());
        let mut last_val: f64 = 0.0;
        let mut last_molecular: i32 = 0;
        let size = floats.len();
        for i in 0..size {
            let float = &floats[i];
            if float.val == last_val {
                if last_molecular > float.molecular {
                    // 如果大于
                    ans[i - 1] = format!("{}/{}", float.molecular, float.distribution);
                }else{
                    continue;
                }
            }else {
                ans.push(format!("{}/{}", float.molecular, float.distribution));
            }
            last_val = float.val;
            last_molecular = float.molecular;
        }
        return ans;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1447() {
        let mut left: Vec<String> = vec![];
        let param = vec!["1/2", "1/3", "2/3"];
        for word in param {
            left.push(String::from(word));
        }
        assert_eq!(left, Solution::simplified_fractions(3));
    }
}