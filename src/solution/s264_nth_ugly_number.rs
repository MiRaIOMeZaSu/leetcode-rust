use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut a: i32 = 1;
        let mut b: i32 = 1;
        let mut c: i32 = 1;
        let finalA = 2;
        let finalB = 3;
        let finalC = 5;
        let mut i = 1;
        let mut nextA = a * finalA;
        let mut nextB = b * finalB;
        let mut nextC = c * finalC;
        while i < n {
            let minNum = cmp::min(nextA, cmp::min(nextB, nextC));
            if minNum == nextA {
                a = nextA;
                nextA *= finalA;
            } else if minNum == b * finalB {
                b = nextB;
                nextB *= finalB;
            } else {
                c = nextC;
                nextC *= finalC;
            }
            i += 1;
        }
        return a * b * c;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_264() {
        assert_eq!(12, Solution::nth_ugly_number(10));
        assert_eq!(1, Solution::nth_ugly_number(1));
    }
}