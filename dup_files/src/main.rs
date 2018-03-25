extern crate crypto;

use std::fs::{self, File};
use std::path::Path;
use std::collections::HashMap;
use std::io::{Result, BufReader, BufRead};
use crypto::digest::Digest;
use crypto::sha2::Sha256;


fn main() {
    let _ = find_dup_files("/Users/Tony/Projects/Rust/dup_files/test_data");
}

fn find_dup_files(root: &str) -> Result<Vec<Vec<String>>> {
    let path = Path::new(root);
    let mut result = Vec::new();
    if !path.exists() || !path.is_dir() {
        return Ok(result);
    }

    let mut dirs_to_visit = vec![root.to_owned()];
    let mut path_by_size: HashMap<u64, Vec<String>> = HashMap::new();
    
    while dirs_to_visit.len() > 0 {
        let cur_dir = dirs_to_visit.pop().unwrap();
        println!("Visitng {:?}", cur_dir);
        let cur_dir_path = Path::new(&cur_dir);
        assert_eq!(cur_dir_path.is_dir(), true);

        for entry in fs::read_dir(cur_dir_path)? {
            if let Ok(ok_entry) = entry {
                let entry_buff = ok_entry.path();
                if entry_buff.is_dir() {
                    dirs_to_visit.push(entry_buff.as_path().to_str().unwrap().to_owned())
                }
                else {
                    if let Ok(metadata) = ok_entry.metadata() {
                        let file_size = metadata.len();
                        let mut paths = path_by_size.entry(file_size).or_insert_with(Vec::new);
                        paths.push(entry_buff.to_str().unwrap().to_owned())
                    }
                    else {
                        assert!(false);
                    }
                }
            }
        }
    }

    let mut path_by_hash: HashMap<String, Vec<String>> = HashMap::new();
    for vec_ref in path_by_size.values() {
        if vec_ref.len() == 1 {
            continue;
        }
        for path_str in vec_ref {
            let file_path = Path::new(path_str);
            assert!(!file_path.is_dir());
            
            let file = File::open(path_str)?;
            let mut reader = BufReader::with_capacity(3, file);
            let mut hasher = Sha256::new();

            loop {
                let len = { 
                    let buffer = reader.fill_buf().unwrap();
                    // println!("Reading {} bytes for {:?}", buffer.len(), path_str);
                    if buffer.len() == 0 {
                        break;
                    }
                    hasher.input(buffer);
                    buffer.len()
                };
                reader.consume(len);
            }
            
            let mut vec = path_by_hash.entry(hasher.result_str().to_owned()).or_insert_with(Vec::new);
            vec.push(path_str.to_owned())
        }
    }

    result = path_by_hash.values()
    .fold(vec![], 
          |mut total, v|  
          {
            if v.len() > 1 {
                total.push(v.to_owned());
            }
            total
          });

    println!("path_by_hash: {:?}", result);
    Ok(result)
}
