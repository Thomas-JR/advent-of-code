fn get_score(a: u32, b0: u32) -> u32 {
    let a = a - 65;
    let mut b = b0 - 88;
   
    println!("{b}");
   
    if b == 0 {
        b = (a + 3 - 1) % 3;
    } else if b == 1 {
        b = a;
    } else {
        b = (a + 1) % 3;
    }
   
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