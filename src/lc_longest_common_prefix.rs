// Leetcode problem 014
// https://leetcode.com/problems/longest-common-prefix/

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let strs = strs.split_first().unwrap();

    let mut prefix = strs.0.as_str();

    for s in strs.1 {
        while !s.starts_with(prefix) {
            prefix = &prefix[..prefix.len() - 1];
            if prefix.is_empty() {
                return String::new();
            }
        }
    }

    prefix.to_string()
}

#[test]
fn test() {
    let strs = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    let s = longest_common_prefix(strs);
    assert_eq!(s, "fl");
}
