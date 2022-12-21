fn main() {
    let crate_count = (INPUT.lines().next().unwrap().len() + 1) / 4;
    let mut lists: Vec<Vec<char>> = Vec::new();
    for _ in 0..crate_count {
        lists.push(Vec::new());
    }

    for line in INPUT.lines() {
        // check end of crate inputs
        let bytes = line.as_bytes();
        if line.len() > 1 && bytes[1] == 49 {
            println!("found end");
            break;
        }
       
        // extract crate elements
        let mut i = 1;
        while i < line.len() {
            // check non-space char
            if bytes[i] != 32 {
                lists[(i) / 4].insert(0, bytes[i] as char);
            }
            i += 4;
        }
    }
   
    let mut instructions_started = false;
    for line in INPUT.lines() {
        if line.eq("") {
            instructions_started = true;
            continue
        }
       
        if !instructions_started {
            continue;
        }
       
        let mut words = line.split_whitespace();
        let mut count = match words.nth(1) {
            Some(x) => x,
            None => "-1",
        }.parse::<u32>().unwrap();
        let from = match words.nth(1) {
            Some(x) => x,
            None => "",
        }.parse::<u32>().unwrap() - 1;
        let to = match words.nth(1) {
            Some(x) => x,
            None => "",
        }.parse::<u32>().unwrap() - 1;
       
        while count > 0 {
            // println!("move {} from {} to {}", count, from, to);
            // println!("start: {:?}", lists[from as usize]);
            // println!("       {:?}", lists[to as usize]);
            let from_val = match lists[from as usize].pop() {
                Some(x) => x,
                None    => '0',
            };
            lists[to as usize].push(from_val);
            // println!("end:   {:?}", lists[from as usize]);
            // println!("       {:?}", lists[to as usize]);
            count -= 1;
           
        }
    }
   
    let mut final_string = Vec::new();
   
    println!("moved elements");
    for list in lists {
        final_string.push(list[list.len() - 1]);
    //     for e in list {
    //         print!("{e} ");
    //     }
    //     println!();
    }
   
    let final_string: String = final_string.into_iter().collect();
   
    println!("{}", final_string);
}
