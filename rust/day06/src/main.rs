use std::time::Instant;
use std::fs;
use std::collections::HashSet;
fn main() {
    let start_time = Instant::now();
    //////////////////////////////////////////////////////////////////////////////////
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    
    
    for i in 0..contents.len() {
        let slice = &contents[i..i+4];
        let set = slice.chars().collect::<HashSet<_>>();
        if set.len() == 4 {
            println!("Part 1: {}", i+4);
            break;
        }
    }

    for i in 0..contents.len() {
        let slice = &contents[i..i+14];
        let set = slice.chars().collect::<HashSet<_>>();
        if set.len() == 14 {
            println!("Part 2: {}", i+14);
            break;
        }
    }

    //////////////////////////////////////////////////////////////////////////////////
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    println!("Rust program executed in {:?}", duration);
}
