use std::time::Instant;
use std::fs;
fn main() {
    let start_time = Instant::now();
    //////////////////////////////////////////////////////////////////////////////////
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    
    let lines: Vec<&str> = contents.split("\n").collect();
    
    let mut elfs: Vec<u32> = Vec::new();
    let mut curr_elf = 0;
    for s in lines {
        let s = s.trim();
        match s {
            "" => {
                elfs.push(curr_elf);
                curr_elf = 0;
            },
            _ => {
                curr_elf += s.parse::<u32>().unwrap();
            }
        }   
    }
    // Sort elfs
    elfs.sort_unstable();
    println!("Part 1: {}", elfs[elfs.len()-1]);
    println!("Part 2: {}", elfs[elfs.len()-1] + elfs[elfs.len()-2] + elfs[elfs.len()-3]);
    //////////////////////////////////////////////////////////////////////////////////
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    println!("Rust program executed in {:?}", duration);
}
