// Lesson: split needs to call collect()

fn main() {
    println!("Hello, world!");
}

fn arithmatic(input: &str) -> i32 {
    let tokens: Vec<&str> = input.split("+").collect();
    let mut sum = 0;
    for token in tokens {
        if token.len() == 0 {
            continue;
        }
        else if token.len() == 1 {
            sum += token.parse::<i32>().unwrap();
        }
        else {
            let multi:Vec<&str> = token.split("*").collect();
            let mut res = 1;
            for m in multi {
                res *= m.parse::<i32>().unwrap();
            }
            sum += res;
        }
    }
    return sum;
}

#[test]
fn verify_arithmatic() {
    assert_eq!(0, arithmatic(""));
    assert_eq!(5, arithmatic("5"));
    assert_eq!(14, arithmatic("5+9"));
    assert_eq!(45, arithmatic("5*9"));
    assert_eq!(567, arithmatic("9*9*7"));
    assert_eq!(11, arithmatic("5+2*3"));
}
