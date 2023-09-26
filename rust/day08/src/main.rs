use std::time::Instant;
use std::fs;
use ndarray::prelude::*;
use ndarray::{Array2, Axis};
use std::cmp;
#[allow(non_snake_case)]
fn main() {
    let start_time = Instant::now();
    //////////////////////////////////////////////////////////////////////////////////
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    
    let lines: Vec<&str> = contents.split("\n").collect();
    // Create a Array2 from the lines (each digit is a number/entrie)
    let mut M = Array2::<u32>::zeros((lines.len(), lines.len()));
    for i in 0..lines.len() {
        let vec: Vec<u32> = lines[i].chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
        for d in 0..vec.len() {
            M[[i, d]] = vec[d];
        }
    }
    let (total_row, total_col) = M.dim();
    let mut part_1 = 2*total_col + 2*total_row - 4;
    let mut part_2 = 0;
    
    for row in 1..total_row-1 {
        for col in 1..total_col-1 {
            let mut up = M.slice(s![..row, col]);
            up.invert_axis(Axis(0));
            let down = M.slice(s![row+1.., col]);
            let mut left = M.slice(s![row, ..col]);
            left.invert_axis(Axis(0));
            let right = M.slice(s![row, col+1..]);
            
            let is_bigger_than_neighbors = 
                up.iter().all(|&x| x < M[[row,col]]) ||
                down.iter().all(|&x| x < M[[row,col]]) ||
                left.iter().all(|&x| x < M[[row,col]]) ||
                right.iter().all(|&x| x < M[[row,col]]);
            
            if is_bigger_than_neighbors {
                part_1 += 1;
            }
            let mut score = 1;
            for side in &[up, down, left, right] {
                let mut counter = 0;
                for tree in side.iter() {
                    counter += 1;
                    if *tree >= M[[row,col]] {
                        break;
                    }
                }
                score *= counter;
            }
            part_2 = cmp::max(part_2, score)
        }
    }
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
    
    //////////////////////////////////////////////////////////////////////////////////
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    println!("Rust program executed in {:?}", duration);
}
