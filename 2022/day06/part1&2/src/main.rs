use std::collections::HashSet;
use std::fs::read_to_string;
use std::collections::VecDeque;

fn main() {
    let file = read_to_string("../input.txt").expect("Unable to read file");
    println!("{file}");
    let mut deque = VecDeque::new();
    let n = 14;

    for (i, c) in file.chars().enumerate() {
        if deque.len() < n {
            deque.push_back(c);
            continue;
        }

        // check all unique
        deque.remove(0);
        deque.push_back(c);
        let mut set = HashSet::new();
        let mut valid = true;
        for c in &deque {
            if set.contains(&c) {
                valid = false;
                break;
            }
            set.insert(c);
        }

        if valid {
            println!("found unique at index {}", i+1);
            break;
        }
    }
}
