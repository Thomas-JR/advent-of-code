fn main() {
    let mut total = 0;
   
    for line in INPUT.lines() {
        let chars = line.chars();
        let n = line.chars().count() / 2;
        let mut chars_seen = HashSet::new();
       
        // let mut left = String::new();
        // let mut right = String::new();
       
        for (i, c) in chars.enumerate() {
            if i < n {
                chars_seen.insert(c);
            } else {
                if chars_seen.contains(&c) {
                    let mut c = c as u32 + 1;
                    if c < 'a' as u32 {
                        c += 58
                    }
                    c -= 'a' as u32;
                   
                    total += c;
                    break;
                }
            }
        }
    }
   
    println!("total = {total}");
}