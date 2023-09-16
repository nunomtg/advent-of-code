use std::time::Instant;
use std::fs;
use std::collections::HashSet;
fn main() {
    let start_time = Instant::now();
    //////////////////////////////////////////////////////////////////////////////////
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut score_1 = 0;

    for line in lines.iter() {
        let line = line.trim();
        let middle = (line.len() / 2) as usize;
        let (p1, p2) = (&line[..middle], &line[middle..]);
        let set1: HashSet<char> = p1.chars().collect();
        let set2: HashSet<char> = p2.chars().collect();
        let c = *set1.intersection(&set2).next().unwrap();
        if c.is_ascii_uppercase() {
            score_1 += c as u32 - 'A' as u32 + 27
        }
        else {
            score_1 += c as u32 - 'a' as u32 + 1
        }
    }
    let mut score_2 = 0;
    let mut iter1 = lines.iter().step_by(3);
    let mut iter2 = lines.iter().skip(1).step_by(3);
    let mut iter3 = lines.iter().skip(2).step_by(3);
    while let (Some(l1), Some(l2), Some(l3)) = (iter1.next(), iter2.next(), iter3.next()) {
        let l1 = l1.trim();
        let l2 = l2.trim();
        let l3 = l3.trim();

        let set1 = l1.chars().collect::<HashSet<char>>();
        let set2 = l2.chars().collect::<HashSet<char>>();
        let set3 = l3.chars().collect::<HashSet<char>>();
        let c = &(&set1 & &set2) & &set3;
        let c = *c.iter().next().unwrap();
        if c.is_ascii_uppercase() {
            score_2 += c as u32 - 'A' as u32 + 27
        }
        else {
            score_2 += c as u32 - 'a' as u32 + 1
        }
    }


    println!("Part 1: {}", score_1);
    println!("Part 2: {}", score_2);

    //////////////////////////////////////////////////////////////////////////////////
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    println!("Rust program executed in {:?}", duration);
}
