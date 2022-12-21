use std::io::{stdout, Write};
use curl::easy::Easy;

fn get_score(a: u32, b: u32) -> u32 {
    let a = a - 65;
    let b = b - 88;
   
    let mut score: u32 = b + 1;
   
    if b == (a + 1) % 3 {
        score += 6;
    } else if a == b {
        score += 3;
    }
   
    println!("{}:{} = {}", a, b, score);
   
    return score;
}

fn main() {
    // get input
    // let mut easy = Easy::new();
    // easy.url("https://www.rust-lang.org/").unwrap();
    // easy.write_function(|data| {
    //     stdout().write_all(data).unwrap();
    //     Ok(data.len())
    // }).unwrap();
    // easy.perform().unwraj();

    // println!(easy);
    const INPUT: &str = "";

    let mut total: u32 = 0;
   
    for line in INPUT.lines() {
        let cur_line = line.as_bytes();
       
        let a = cur_line[0] as u32;
        let b = cur_line[2] as u32;
       
        // let a = std::char::from_u8(cur_line[0]);
        // let b = std::char::from_u8(cur_line[2]);
       
        total += get_score(a, b);
    }
   
    println!("final score = {}", total);
}