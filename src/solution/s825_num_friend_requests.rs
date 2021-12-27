pub struct Solution {}

impl Solution {
    pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
        // 使用滑动窗口法
        let mut agesMut: Vec<i32> = vec![];
        for age in ages {
            agesMut.push(age);
        }
        agesMut.sort();
        let size: usize = agesMut.len();
        let mut left: i32 = -1;
        // let mut right: i32 = -1;
        let mut ans: i32 = 0;
        let mut i: usize = 0;
        while i < size {
            let mut index: usize = usize::from((left + 1) as u16);
            while index < i {
                if agesMut[index] * 2 <= agesMut[i] + 14 {
                    // 不可以发送分支
                    left += 1;
                    index += 1;
                    // left+1不能发送
                } else {
                    // 可以发送分支
                    // left+1能够发送,但left不能发送
                    break;
                }
            }
            if index < i {
                ans += i as i32 - (left + 1);
            }
            i += 1;
        }
        // 检查向后走相同的
        i = 0;
        let mut curr: i32 = 1;
        while i < size {
            if i + 1 < size && agesMut[i + 1] == agesMut[i] {
                curr += 1;
            } else {
                // 断掉了
                // 总长为curr
                if curr > 1 && agesMut[i]  > 14 {
                    ans += (curr * (curr - 1)) / 2;
                }
                curr = 1;
            }
            i += 1;
        }
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_825() {
        assert_eq!(2, Solution::num_friend_requests(vec![16, 16]));
        assert_eq!(2, Solution::num_friend_requests(vec![16, 17, 18]));
        assert_eq!(29, Solution::num_friend_requests(vec![73,106,39,6,26,15,30,100,71,35,46,112,6,60,110]));
    }
}