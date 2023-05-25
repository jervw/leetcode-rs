// Leetcode problem 009
// https://leetcode.com/problems/palindrome-number/

pub fn is_palindrome(x: i32) -> bool {
    let s: String = x.to_string().chars().rev().collect();

    if x.to_string() == s {
        return true;
    }

    false
}

#[test]
fn test() {
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(-121), false);
    assert_eq!(is_palindrome(10), false);
    assert_eq!(is_palindrome(-101), false);
}
