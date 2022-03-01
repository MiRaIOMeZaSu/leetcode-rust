struct Solution {}

impl Solution {
    pub fn pancake_sort(arr: Vec<i32>) -> Vec<i32> {
        // 如何使用
        // 简单的从n - 1 到 1进行排序
        // 如何翻转?
        // 使用迭代法?
        // 翻转的数组必须为0-k-1
        let mut curr_target = arr.len() as i32;
        let mut ans: Vec<i32> = vec![];
        let mut arr_mut = arr;
        while curr_target >= 2 {
            // 如何继续
            let mut index = arr_mut.len() - 1;
            if arr_mut[index] == curr_target {
                curr_target -= 1;
                arr_mut = Vec::from(&arr_mut[0..index]);
                continue;
            }
            let mut arr_mut_next = vec![];
            while index > 0 && arr_mut[index] != curr_target {
                arr_mut_next.push(arr_mut[index]);
                index -= 1;
            }
            if index != 0 {
                ans.push(1 + index as i32);
            }
            for i in 0..index {
                arr_mut_next.push(arr_mut[i]);
            }
            ans.push(curr_target);
            arr_mut = arr_mut_next;
            curr_target -= 1;
        }
        if arr_mut[0] != 1 {
            ans.push(2);
        }
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_969() {
        assert_eq!(vec![4, 2, 4, 3], Solution::pancake_sort(vec![1, 3, 2, 4]));
    }
}