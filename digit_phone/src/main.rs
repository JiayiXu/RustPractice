use std::fs::File;
use std::collections::{HashSet, HashMap};
use std::io::{Result, BufReader, BufRead};

fn main() {
    //println!("Hello, world!")
    let dictionary = populate_dict("/Users/Tony/Projects/Rust/digit_phone/words.txt").unwrap();
    // println!("dictionary: {:?}", dictionary);
    assert!(dictionary.contains("apple"));

    let mut digit_to_char = HashMap::<u32, Vec<&str>>::new();
    digit_to_char.insert(0, vec![]);
    digit_to_char.insert(1, vec![]);
    digit_to_char.insert(2, vec!["A", "B", "C"]);
    digit_to_char.insert(3, vec!["D", "E", "F"]);
    digit_to_char.insert(4, vec!["G", "H", "I"]);
    digit_to_char.insert(5, vec!["J", "K", "L"]);
    digit_to_char.insert(6, vec!["M", "N", "O"]);
    digit_to_char.insert(7, vec!["P", "Q", "R", "S"]);
    digit_to_char.insert(8, vec!["T", "U", "V"]);
    digit_to_char.insert(9, vec!["W", "X", "Y", "Z"]);

    combination("3278227", &digit_to_char, &dictionary);
}

fn combination(digits:&str, digit_to_char:&HashMap<u32, Vec<&str>>, dict: &HashSet<String>) {
    assert!(digits.len() == 7);
    let splits = vec![3, 4, 7];
    for split in splits {
        let (first, second) = combination_helper(split, digits, digit_to_char, dict);
        println!("got first: {:?}, second: {:?}", first, second);
    }
}

fn combination_helper(split:usize, digits:&str, digit_to_char:&HashMap<u32, Vec<&str>>, dict: &HashSet<String>) -> (Vec<String>, Vec<String>){
    let mut first_half = Vec::new();
    let mut second_half = Vec::new();
    get_string(&digits[..split], "", digit_to_char, dict, &mut first_half);
    if first_half.len() > 0 && split < digits.len(){
        println!("Last half: {}", &digits[split..]);
        get_string(&digits[split..], "", digit_to_char, dict, &mut second_half);
    }
    (first_half, second_half)
}


fn populate_dict(file_path:&str) -> Result<HashSet<String>>{
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut dict: HashSet<String> = HashSet::new();
    for line in reader.lines() {
        if let Ok(line_) = line {
            dict.insert(line_);
        }
    }
    return Ok(dict);
}

fn get_string(digits:&str, candidate:&str, digit_to_char:&HashMap<u32, Vec<&str>>, dict: &HashSet<String>, result: &mut Vec<String>) {
    if digits.len() == 0 {
        if dict.contains(&candidate.to_lowercase()) {
            result.push(candidate.to_owned());
        }
    }
    else {
        //println!("digits: {:?}", digits);
        let digit = digits.chars().next().unwrap().to_string().parse::<u32>().unwrap();
        if let Some(next_candidates) = digit_to_char.get(&digit) {
            for next_candidate in next_candidates {
                get_string(&digits[1..], &(candidate.to_string() + next_candidate), digit_to_char, dict, result);
            }
        }
    }
}


