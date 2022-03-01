struct Solution {}

struct Letter {
    count: i32,
    val: char,
}

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut counts: Vec<Letter> = vec![Letter {
            count: a,
            val: 'a',
        }, Letter {
            count: b,
            val: 'b',
        }, Letter {
            count: c,
            val: 'c',
        }];
        let mut total = a + b + c;
        let mut ans = String::new();
        let mut lastChar: char = 'd';
        // let mut flag = true;
        while total != 0 {
            counts.sort_by_key(|k| k.count);
            let mut index = 2;
            if lastChar == counts[index].val {
                index -= 1;
                if counts[index].count == 0 {
                    return ans;
                }
                // flag = counts[2].count - counts[1].count < 2 && index == 1;
            }
            let mut times = 0;
            if counts[index].count > 0 {
                if counts[index].count >= 2 && index == 2 {
                    counts[index].count -= 2;
                    times += 2;
                } else {
                    counts[index].count -= 1;
                    times += 1;
                }
                lastChar = counts[index].val;
            }
            for _ in 0..times {
                ans.push(counts[index].val);
            }
        }
        return ans;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1405() {
        // Solution::gray_code(2);
        assert_eq!(String::from("ccaccbcc"), Solution::longest_diverse_string(1, 1, 7));
    }
}