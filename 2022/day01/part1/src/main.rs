use std::fs::read_to_string;
use std::cmp;

fn main() {
    // let file = read_to_string("../test-input.txt").expect("Unable to read file");
    let file = read_to_string("../input.txt").expect("Unable to read file");

    let mut max = 0;
    let mut cur = 0;
    for line in file.lines() {
        if line == "" {
            max = cmp::max(max, cur);
            cur = 0;
            continue;
        }
        cur += line.parse::<u32>().unwrap();
    }

    println!("max = {}", max);
}
