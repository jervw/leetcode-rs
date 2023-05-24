// Leetcode problem 013
// https://leetcode.com/problems/roman-to-integer/

const ROMAN_TABLE: [(char, i32); 7] = [
    ('I', 1),
    ('V', 5),
    ('X', 10),
    ('L', 50),
    ('C', 100),
    ('D', 500),
    ('M', 1000),
];

pub fn roman_to_int(mut s: String) -> i32 {
    let mut sum = 0;

    s = s.replace("IV", "IIII").replace("IX", "VIIII");
    s = s.replace("XL", "XXXX").replace("XC", "LXXXX");
    s = s.replace("CD", "CCCC").replace("CM", "DCCCC");

    for c in s.chars() {
        for (r, n) in ROMAN_TABLE.iter() {
            if *r == c {
                sum += n;
            }
        }
    }

    sum
}

#[test]
fn test() {
    assert_eq!(roman_to_int(String::from("III")), 3);
    assert_eq!(roman_to_int(String::from("LVIII")), 58);
    assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
}
