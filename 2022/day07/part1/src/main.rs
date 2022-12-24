use std::fs::read_to_string;

fn main() {
    let file = read_to_string("../test-input.txt").expect("failed to read file");
    for line in file.lines() {
        let mut chars = line.chars();

        println!("{line}");
        if chars.nth(0).expect("invalid string in line") == '$' {
            continue;
        }
        println!("  yes");
        // if line.chars()[0] == '$' {
        //     println!("action");
        // } else {
        //     println!("file");
        // }
    }
}
