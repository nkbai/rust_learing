struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut temp = nums[0];
        let mut count = 0;
        for num in &nums {
            if *num != temp {
                count -= 1;
                if count == 0 {
                    temp = *num;
                    count += 1;
                }
            } else {
                count += 1;
            }
        }
        temp
    }
}

/*
给定一个大小为 n 的数组，找到其中的众数。众数是指在数组中出现次数大于 ⌊ n/2 ⌋ 的元素。

你可以假设数组是非空的，并且给定的数组总是存在众数。

示例 1:

输入: [3,2,3]
输出: 3
示例 2:

输入: [2,2,1,1,1,2,2]
输出: 2

*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![3, 2, 3];
        assert_eq!(3, Solution::majority_element(v));
    }

    #[test]
    fn test2() {
        let v = vec![2,2,1,1,1,2,2];
        assert_eq!(2, Solution::majority_element(v));
    }

    #[test]
    fn test3() {
        let v = vec![3, 3, 4];
        assert_eq!(3, Solution::majority_element(v));
    }
    
}


fn main() {
    let answer = Solution::majority_element(vec![3, 2, 3]);
    println!("{}", answer);
}