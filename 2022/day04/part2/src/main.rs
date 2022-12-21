const INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

fn main() {
    let mut count = 0;
   
    let mut pairs = Vec::new();
   
    for line in INPUT.lines() {
        let ranges: Vec<&str> = line.split(',').collect();
        let f = ranges[0];
        let s = ranges[1];
       
        let f_ranges: Vec<&str> = f.split('-').collect();
        let s_ranges: Vec<&str> = s.split('-').collect();
       
        let f0 = f_ranges[0].parse::<i32>().unwrap();
        let f1 = f_ranges[1].parse::<i32>().unwrap();
       
        let s0 = s_ranges[0].parse::<i32>().unwrap();
        let s1 = s_ranges[1].parse::<i32>().unwrap();
       
        pairs.push(vec![f0, f1]);
        pairs.push(vec![s0, s1]);
       
        // println!("{f0}-{f1},{s0}-{s1}");
    }
   
    let mut i = 0;
    for pair0 in pairs {
        let f0 = pair0[0];
        let f1 = pair0[1];
       
        let mut j = 0;
        for pair1 in pairs {
            if i == j {
                continue;
            }
           
            let s0 = pair1[0];
            let s1 = pair1[1];
           
            if f0 <= s0 && f1 >= s1 || s0 <= f0 && s1 >= f1 {
                count += 1;
                break;
            }
           
            j += 1;
        }
       
        i += 1;
        // println!("{a}-{b}");
    }
   
    println!("found {count}");
}