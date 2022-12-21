fn main() {
    let mut total = 0;
   
    let mut counter = 0;
    let mut chars_counter = HashMap::new();
   
    for line in INPUT.lines() {
        let mut line_chars = HashSet::new();
       
        for c in line.chars() {
            if line_chars.contains(&c) {
                continue;
            }
           
            match chars_counter.get(&c) {
                Some(val) => chars_counter.insert(c, val + 1),
                None => chars_counter.insert(c, 1),
            };
           
            if chars_counter[&c] == 3 {
                println!("found {c}");
               
                let mut c = c as u32 + 1;
                if c < 'a' as u32 {
                    c += 58
                }
                c -= 'a' as u32;
               
                total += c;
            }
           
            line_chars.insert(c);
        }
       
        counter += 1;
       
        if counter % 3 == 0 {
            chars_counter = HashMap::new();
        }
    }
   
    println!("total = {total}");
}