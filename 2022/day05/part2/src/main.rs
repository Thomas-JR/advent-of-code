use std::fs::read_to_string;

fn main() {
    let file = read_to_string("../input.txt").expect("Unable to read file");

    let crate_count = (file.lines().next().unwrap().len() + 1) / 4;
    let mut lists: Vec<Vec<char>> = Vec::new();
    for _ in 0..crate_count {
        lists.push(Vec::new());
    }

    for line in file.lines() {
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
    for line in file.lines() {
        if line.eq("") {
            instructions_started = true;
            continue
        }
       
        if !instructions_started {
            continue;
        }
       
        let mut words = line.split_whitespace();
        let mut count = words.nth(1).expect("Failed to read word").parse::<u32>().unwrap();
        let from = words.nth(1).expect("Failed to read word").parse::<u32>().unwrap() - 1;
        let to = words.nth(1).expect("Failed to read word").parse::<u32>().unwrap() - 1;
       
        let mut to_add = Vec::new();
        while count > 0 {
            to_add.push(lists[from as usize].pop().expect("Failed to pop from list"));
            count -= 1;
        }
        to_add.reverse();
        for e in to_add {
            lists[to as usize].push(e);
        }
    }
   
    let mut final_string = Vec::new();
   
    println!("moved elements");
    for list in lists {
        final_string.push(list[list.len() - 1]);
    }
   
    let final_string: String = final_string.into_iter().collect();
   
    println!("{}", final_string);
}