use std::{collections::HashMap, ops::Range};
/*
 *
 * 6 digit password: xxxxxx
 * at least 2 adjacent characters are the same: 122378
 * from left to right digits only increase: 112345
 * is inside the given range: xxxxxx - yyyyyy
 *
 * */

// turn the number into a string then, with each charater turn it into a counter with hashmap
// if the len of hashmap is less than 6 the number has repeating characters.
//
// digits increasing:
//
pub fn num_possible_passwords_for_container(range: Range<u32>) -> u32 {
    let mut count = 0;
    for num in range {
        let num_str = num.to_string();
        let mut chars_vec: Vec<char> = num_str.chars().collect();
        let original_chars = chars_vec.clone();
        chars_vec.sort();
        if original_chars != chars_vec {
            continue;
        }
        let hmap: HashMap<char, u32> = num_str.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(1) += 1;
            acc
        });
        if hmap.len() > 5 {
            continue;
        }

        count += 1;
    }
    count
}
pub fn extra_num_possible_passwords_for_container(range: Range<u32>) -> u32 {
    let mut count = 0;
    for num in range {
        //increasing digits
        let num_str = num.to_string();
        let mut chars_vec: Vec<char> = num_str.chars().collect();
        let original_chars = chars_vec.clone();
        chars_vec.sort();
        if original_chars != chars_vec {
            continue;
        }
        //repeating digits
        let num_str = num.to_string();
        let hmap: HashMap<char, u32> = num_str.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        if hmap.len() > 5 {
            continue;
        }
        let mut has_pair = false;
        for (_key, val) in hmap {
            if val == 2 {has_pair = true; break;};
        }
        if !has_pair {
            continue;
        }

        count += 1;
    }
    count
}
