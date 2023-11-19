use std::collections::{VecDeque, HashSet};
use std::time::Instant;
use std::fs;
use ndarray::Array2;

fn height(c: char) -> u8 {
    match c {
        'S' => 0,
        'E' => 25,
        _ => (c as u32 - 'a' as u32) as u8,
    }
}

fn main() {
    let start_time = Instant::now();
    //////////////////////////////////////////////////////////////////////////////////
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut m: Array2<u8> = Array2::zeros((lines.len(), lines[0].len()));
    let mut m2: Array2<i32> = Array2::zeros((lines.len(), lines[0].len()));

    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.trim().chars().enumerate() {
            if c == 'S' {
                m[[i, j]] = 0;
                start = (i, j);
            }
            else if c == 'E' {
                m[[i, j]] = 25;
                end = (i, j);  
            } 
            m[[i, j]] = height(c);
            m2[[i, j]] = -(height(c) as i32);
        }
    }

    let mut q: VecDeque<(usize, usize, u32)> = VecDeque::new();
    let mut v:HashSet<(usize, usize)> = HashSet::new();
    q.push_back((start.0, start.1, 0));

    while !q.is_empty() {
        let (row, col, c) = q.pop_front().unwrap();
        if v.contains(&(row,col)) {
            continue;
        }
        v.insert((row, col));
        if (row, col) == end {
            println!("Part 1: {}", c);
            break;
        }
        for (rr, cc) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (nrow, ncol) = (row as i32 + rr, col as i32 + cc);
            if nrow < 0 || nrow >= m.nrows() as i32 || ncol < 0 || ncol >= m.ncols() as i32 {
                continue;
            }
            let (nrow, ncol) = (nrow as usize, ncol as usize);
            if m[[nrow, ncol]] as i32 - m[[row, col]] as i32 > 1 {
                continue;
            }
            q.push_back((nrow, ncol, c + 1));
        }
    }

    let mut q: VecDeque<(usize, usize, u32)> = VecDeque::new();
    let mut v:HashSet<(usize, usize)> = HashSet::new();
    q.push_back((end.0, end.1, 0));

    while !q.is_empty() {
        let (row, col, c) = q.pop_front().unwrap();
        if v.contains(&(row,col)) {
            continue;
        }
        v.insert((row, col));
        if m2[[row, col]] == 0 {
            println!("Part 2: {}", c);
            break;
        }
        for (rr, cc) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (nrow, ncol) = (row as i32 + rr, col as i32 + cc);
            if nrow < 0 || nrow >= m.nrows() as i32 || ncol < 0 || ncol >= m.ncols() as i32 {
                continue;
            }
            let (nrow, ncol) = (nrow as usize, ncol as usize);
            if m2[[nrow, ncol]] as i32 - m2[[row, col]] as i32 > 1 {
                continue;
            }
            q.push_back((nrow, ncol, c + 1));
        }
    }
    
    println!();
    //////////////////////////////////////////////////////////////////////////////////
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    println!("Rust program executed in {:?}", duration);
}
