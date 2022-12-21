fn main() {
    let mut count = 0;
   
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
       
        // println!("{f0}-{f1},{s0}-{s1}");
       
        if f0 <= s0 && f1 >= s1 || s0 <= f0 && s1 >= f1 {
            count += 1;
            // println!("found");
        }
    }
   
    println!("found {count}");
}