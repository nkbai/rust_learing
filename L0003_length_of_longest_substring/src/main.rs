use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut window = HashSet::new();
        let mut (window_len, max_len) = (0, 0);

        let string_iter: Vec<char> = s.chars().collect();

        for (idx, string_char) in string_iter.iter().enumerate() {
            if window.contains(string_char) {
                for i in (idx - window_len)..idx {
                    let temp_char = string_iter[i];
                    window.remove(&string_iter[i]);
                    window_len -= 1;
                    if temp_char == *string_char {
                        break
                    }
                }
            }
            window.insert(string_char);
            window_len += 1;
            max_len = max_len.max(window_len);
        }
        // println!("{:?}", window);
        max_len as i32
    }
}
/* 执行4ms 快了12ms
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let seq: Vec<char> = s.chars().collect();
        let len = seq.len();
        let (mut start, mut end, mut max) = (0, 0, 0);

        while end < len {
            for idx in start..end {
                if seq[end] == seq[idx] {
                    start = idx + 1;
                    break
                }
            }
            let curr = end - start + 1;
            if curr > max { max = curr }
            end += 1
        }
        max as i32
    }
    
}
*/

/*
给定一个字符串，请你找出其中不含有重复字符的 最长子串 的长度。

示例 1:

输入: "abcabcbb"
输出: 3 
解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
示例 2:

输入: "bbbbb"
输出: 1
解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
示例 3:

输入: "pwwkew"
输出: 3
解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
     请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。

*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::length_of_longest_substring("abcabcbb".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::length_of_longest_substring("bbbbb".to_string()));
    }
    #[test]
    fn test_3() {
        assert_eq!(3, Solution::length_of_longest_substring("pwwkew".to_string()));
    }

    #[test]
    fn main_test() {
        assert_eq!(7, Solution::length_of_longest_substring("abcbcdefgabcde".to_string()));
    }
}

fn main() {
    let s = String::from("abcbcdefgabcde");
    let length = Solution::length_of_longest_substring(s);
    println!("{}", length);
}