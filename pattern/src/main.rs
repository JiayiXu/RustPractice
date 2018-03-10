extern crate itertools;
use std::collections::HashMap;
use itertools::zip;

fn main() {
    println!("Hello, world!");
}

fn is_match_pattern(pattern: &str, input: &str) -> bool {
    let mut hash_map: HashMap<&str, char> = HashMap::new();
    let splits: Vec<&str> = input.split(" ").collect();
    
    if splits.len() != pattern.len() {
        return false;
    }

    for (p, w) in zip(pattern.chars(), splits) {
        if hash_map.contains_key(w) {
            if p != hash_map[w] {
                return false;
            }
        }
        else {
            hash_map.insert(w, p);
        }
    }
    return true;
}

#[test]
fn verify_pattern() {
    assert_eq!(false, is_match_pattern("abbc", "hello world world hello"));
    assert_eq!(true, is_match_pattern("abba", "hello world world hello"));
}
