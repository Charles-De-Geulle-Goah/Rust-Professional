/*
    Longest Substring Without Repeating Characters
    Given a string, find the length of the longest substring without repeating characters. 
    The substring must not contain any duplicate characters, and its length should be maximized.

    You need to implement the function `longest_substring_without_repeating_chars(s: String) -> i32`.
    The function should return the length of the longest substring without repeating characters.
    
    Hint: Consider using the sliding window technique to efficiently solve this problem in O(n) time complexity.
*/

use std::fmt::{self, Display, Formatter};

pub fn longest_substring_without_repeating_chars(s: String) -> i32 {
    // TODO: Implement the logic to find the longest substring without repeating characters
    let s = s.as_bytes(); //转为字节数组
    let mut left = 0;   //滑动窗口左指针
    let mut ans = 0; //最终答案
    let mut cnt = [0;128]; //可以用哈希表，这里用数组提高效率
    //right:滑动窗口右指针  c：滑动窗口最右的字符对应的bit值
    for (right,&c) in s.iter().enumerate() {  
        let c = c as usize;
        cnt[c] += 1; //更新哈希表
        while cnt[c] > 1 {  //出现重复就去重直至没有重复
            cnt[s[left] as usize] -= 1; //移除左端的字符
            left += 1;
        }
        ans = ans.max(right-left+1);
    }
    ans as _ // Placeholder return value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring_1() {
        let s = "abcabcbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3);  // "abc"
    }

    #[test]
    fn test_longest_substring_2() {
        let s = "bbbbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 1);  // "b"
    }

    #[test]
    fn test_longest_substring_3() {
        let s = "pwwkew".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3);  // "wke"
    }

    #[test]
    fn test_longest_substring_4() {
        let s = "".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 0);  // Empty string
    }

    #[test]
    fn test_longest_substring_5() {
        let s = "abcde".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 5);  // "abcde"
    }
}
