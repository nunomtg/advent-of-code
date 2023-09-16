extern crate regex;
use std::time::Instant;
use std::fs;
use std::collections::HashSet;
use std::cmp::max;
use regex::Regex;

fn main() {
    let start_time = Instant::now();
    //////////////////////////////////////////////////////////////////////////////////
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    
    let lines: Vec<&str> = contents.split("\n").collect();
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    let mut score_1 = 0;
    let mut score_2 = 0;

    for line in lines {
        let captures: (&str, [&str; 4]) = re.captures(line).unwrap().extract();
        let l1 = captures.1[0].parse::<u32>().unwrap();
        let r1 = captures.1[1].parse::<u32>().unwrap();
        let l2 = captures.1[2].parse::<u32>().unwrap();
        let r2 = captures.1[3].parse::<u32>().unwrap();
        let range1 : HashSet<u32> = (l1..=r1).collect(); 
        let range2 : HashSet<u32> = (l2..=r2).collect();
        let union_len = (&range1 | &range2).len();
        if union_len == max(range1.len(), range2.len()) {
            score_1 += 1;
        }
        if union_len != range1.len() + range2.len() {
            score_2 += 1;
        }
    }
    println!("Part 1: {}", score_1);
    println!("Part 2: {}", score_2);
    //////////////////////////////////////////////////////////////////////////////////
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    println!("Rust program executed in {:?}", duration);
}
