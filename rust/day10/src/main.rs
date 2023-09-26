use std::time::Instant;
use std::fs;

fn check(X: i32, cycle: u32) -> i32 {
    const CHECKPOINTS : [u32; 6] = [20, 60, 100, 140, 180, 220];
    if CHECKPOINTS.contains(&cycle) {
        return X * cycle as i32;
    }
    return 0;
}
fn draw(X: i32, cycle: u32, board: &mut Vec<char>) {
    let curr_pointer: i32 = (cycle as i32 - 1) % 40;
    if (curr_pointer - X).abs() <= 1 {
        board.push('#');
    } else {
        board.push(' ');
    }
}


fn main() {
    let start_time = Instant::now();
    //////////////////////////////////////////////////////////////////////////////////
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut X = 1;
    let mut cycle = 1;
    let mut score = 0;

    let mut board : Vec<char> = Vec::new();


    for line in lines {
        match line.trim().split_ascii_whitespace().collect::<Vec<&str>>().as_slice() {
            ["addx", value] => {
                let value = value.parse::<i32>().unwrap();
                score += check(X, cycle);
                draw(X, cycle, &mut board);
                cycle += 1;

                score += check(X, cycle);
                draw(X, cycle, &mut board);
                cycle += 1;

                X += value;
            },
            _ => {
                score += check(X, cycle);
                draw(X, cycle, &mut board);
                cycle += 1;
            },
        }
    }
    println!("Part 1: {}", score);
    println!("Part 2:");
    for (c,i) in board.iter().enumerate() {
        if c % 40 == 0 && c != 0{
            println!("");
        }
        print!("{}", i);
    }
    println!();
    //////////////////////////////////////////////////////////////////////////////////
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    println!("Rust program executed in {:?}", duration);
}
