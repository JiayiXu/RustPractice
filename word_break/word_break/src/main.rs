// Lessons:
// String in rust is weird
// slice only works with usize
// HashSet yeah..
// mut& is a thing

use std::collections::HashSet;

fn main() {
    let word = "abcde";
    let mut dicts = HashSet::new();
    dicts.insert(String::from("ab"));
    dicts.insert(String::from("abc"));
    dicts.insert(String::from("de"));

    println!("result:{}", word_breaks(word, dicts));
}

fn word_breaks(word: &str, dict_map: HashSet<String>) -> String {
    let mut result = Vec::new();
    word_breaks_helper(word.as_bytes(), 0, &dict_map, &mut result);
    let mut r = String::new();
    if result.len() > 0 {
        let mut prev = "";
        for s in result {
            r.push_str(prev);
            r.push_str(&s[..]);
            prev = " ";
        }
    }

    return r;
}

fn word_breaks_helper(word: &[u8], start_index: u32, dict_map: &HashSet<String>, result: &mut Vec<String>) -> bool {
    if start_index >= word.len() as u32 {
        return true;
    }

    // start from start_index, find the right segment
    for j in start_index..word.len() as u32 {
        let start_index_usize = start_index as usize;
        let j_usize = (j+1) as usize;
        let slice = String::from_utf8(word[start_index_usize..j_usize].to_vec()).unwrap();
        if dict_map.contains(&slice) {
            result.push(slice);
            let is_sub_ok = word_breaks_helper(word, j+1, dict_map, result);
            if is_sub_ok {
                return true;
            }
            else {
                result.pop();
            }
        }
    }

    return false;
}

#[test]
fn verify_words() {
    let word = "abcde";
    let mut dicts = HashSet::new();
    dicts.insert(String::from("ab"));
    dicts.insert(String::from("abc"));
    dicts.insert(String::from("de"));

    assert_eq!("abc de", word_breaks(word, dicts));
}
