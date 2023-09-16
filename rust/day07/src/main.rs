use std::time::Instant;
use std::fs;
use std::collections::HashMap;
fn main() {
    let start_time = Instant::now();
    //////////////////////////////////////////////////////////////////////////////////
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut dirs = HashMap::<String, u64>::new();
    let mut curr_dir = Vec::<&str>::new();

    for line in lines {
        let args: Vec<&str> = line.trim().split(' ').collect();
        let args_slice = &args[..];
        match args_slice {
            ["$", "cd", "/"] => {
                curr_dir.push("home");
                dirs.insert(String::from("home"), 0);
            },
            ["$", "cd", ".."] => {
                curr_dir.pop();
            },
            ["$", "cd", dir] => {
                curr_dir.push(dir);
                let path = curr_dir.join("/");
                dirs.insert(path, 0);
            },
            ["$", "ls"] => {continue},
            ["dir", _] => {continue},
            [size, _] => {
                // print size
                let size = size.parse::<u64>().unwrap();
                *dirs.get_mut("home").unwrap() += size;
                for i in 1..curr_dir.len() {
                    let path = curr_dir[..i+1].join("/");
                    *dirs.get_mut(&path).unwrap() += size;
                }
            },
            _ => {continue},
        }
    }

    let mut part_1: u64 = 0;
    for (_, size) in dirs.iter() {
        if *size <= 100_000 {
            part_1 += size;
        }
    }
    println!("Part 1: {}", part_1);

    let mut part_2: u64 = 100_000_000;
    let space_needed: i64 = 30_000_000 - (70_000_000 - dirs["home"] as i64);
    let space_needed = space_needed as u64;
    
    for (_, size) in dirs {
        if size >= space_needed {
            if size < part_2 {
                part_2 = size;
            }
        }
    }
    println!("Part 2: {}", part_2);
    //////////////////////////////////////////////////////////////////////////////////
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    println!("Rust program executed in {:?}", duration);
}
