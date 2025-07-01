
use std::collections::HashMap;


pub fn are_anagrams(s1: &str, s2: &str) -> bool {
    let s1 = s1.to_lowercase();
    let s2 = s2.to_lowercase();


    if s1.len() != s2.len() {
        return false;
    }


    let mut count1 = HashMap::new();
    let mut count2 = HashMap::new();

    for char in s1.chars() {
        *count1.entry(char).or_insert(0) += 1;
    }

    for char in s2.chars() {
        *count2.entry(char).or_insert(0) += 1;
    }


    count1 == count2
    
}