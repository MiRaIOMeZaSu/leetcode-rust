pub struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        // 先转成数字再转成2进制
        let sizeA = a.len();
        let sizeB = b.len();
        let altB = sizeA > sizeB;
        let mut longIndex;
        let mut shortIndex;
        let mut longBytes;
        let mut shortBytes;
        if altB {
            longIndex = sizeA - 1;
            longBytes = a.as_bytes();
            shortIndex = sizeB - 1;
            shortBytes = b.as_bytes();
        } else {
            longIndex = sizeB - 1;
            longBytes = b.as_bytes();
            shortIndex = sizeA - 1;
            shortBytes = a.as_bytes();
        }
        let mut arr: Vec<u8> = Vec::new();
        // 开始遍历
        let mut next: u8 = 0;
        let pivot: u8 = 1.to_string().as_bytes()[0];
        while shortIndex >= 0 {
            let mut curr: u8 = next;
            if longBytes[longIndex] == pivot {
                curr = curr + 1;
            }
            if shortBytes[shortIndex] == pivot {
                curr = curr + 1;
            }
            next = curr / 2;
            arr.push(curr % 2);
            if longIndex != 0 {
                longIndex = longIndex - 1;
            }
            if shortIndex != 0 {
                shortIndex = shortIndex - 1;
            } else {
                break;
            }
        }
        if sizeA != sizeB {
            while longIndex >= 0 {
                let mut curr: u8 = next;
                if longBytes[longIndex] == pivot {
                    curr = curr + 1;
                }
                next = curr / 2;
                arr.push(curr % 2);
                if longIndex != 0 {
                    longIndex = longIndex - 1;
                } else {
                    break;
                }
            }
        }
        if next == 1 {
            arr.push(1);
        }
        return arr.iter().rev().map(|x| x.to_string()).collect();
    }
}