use std::fs::read_to_string;

fn main() {
    // let file = read_to_string("../test-input.txt").expect("Unable to read file");
    let file = read_to_string("../input.txt").expect("Unable to read file");

    let mut v: Vec<u32> = Vec::new();
    let mut cur = 0;
    for line in file.lines() {
        if line == "" {
            v.push(cur);
            cur = 0;
            continue;
        }
        cur += line.parse::<u32>().unwrap();
    }

    v.sort();
    v.reverse();

    if v.len() < 3 {
        panic!("Expected more inputs than {}", v.len());
    }

    println!("sum = {}", v[0] + v[1] + v[2]);
}
