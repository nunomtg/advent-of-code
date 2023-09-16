use std::time::Instant;
use std::fs;
fn main() {
    let start_time = Instant::now();
    //////////////////////////////////////////////////////////////////////////////////
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    
    let lines: Vec<&str> = contents.split("\n").collect();
    
    // X,Y,Z
    const DRAW: [&str; 3] = ["X","Y","Z"];
    const WIN: [&str; 3] = ["Y","Z","X"];
    const LOSE: [&str; 3] = ["Z","X","Y"];

    let mut part1: u32 = 0;
    let mut part2: u32 = 0;

    for line in lines {
        let (p1, p2) = line.trim().split_once(' ').unwrap();
        let idx = "ABC".find(p1).unwrap();
        
        part1 += p2.chars().next().unwrap() as u32 - 'X' as u32 + 1;

        if p2 == WIN[idx] {
            part1 += 6;
        } else if p2 == DRAW[idx] {
            part1 += 3;
        } else if p2 == LOSE[idx] {
            part1 += 0;
        }
        
        match p2 {
            "X" => part2 += 0 + LOSE[idx].chars().next().unwrap() as u32 - 'X' as u32 + 1,
            "Y" => part2 += 3 + DRAW[idx].chars().next().unwrap() as u32 - 'X' as u32 + 1,
            "Z" => part2 += 6 + WIN[idx].chars().next().unwrap() as u32 - 'X' as u32 + 1,
            _ => (),}

    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    //////////////////////////////////////////////////////////////////////////////////
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    println!("Rust program executed in {:?}", duration);
}
