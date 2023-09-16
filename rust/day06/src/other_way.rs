// This version even if more verbose takes about the same time as the other one on average.

use std::time::Instant;
use std::fs;
use itertools::izip;
use std::collections::HashSet;
fn main() {
    let start_time = Instant::now();
    //////////////////////////////////////////////////////////////////////////////////
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    
    
    let data = contents.chars();
    let data1 = contents.chars().skip(1);
    let data2 = contents.chars().skip(2);
    let data3 = contents.chars().skip(3);
    let mut part_1 = 4;

    for l in izip!(data, data1, data2, data3) {
        let mut set = HashSet::new();
        set.insert(l.0);
        set.insert(l.1);
        set.insert(l.2);
        set.insert(l.3);
        if set.len() == 4 {
            break;
        }
        part_1 += 1;
    }
    let mut part_2 = 14;
    let data = contents.chars();
    let data1 = contents.chars().skip(1);
    let data2 = contents.chars().skip(2);
    let data3 = contents.chars().skip(3);
    let data4 = contents.chars().skip(4);
    let data5 = contents.chars().skip(5);
    let data6 = contents.chars().skip(6);
    let data7 = contents.chars().skip(7);
    let data8 = contents.chars().skip(8);
    let data9 = contents.chars().skip(9);
    let data10 = contents.chars().skip(10);
    let data11 = contents.chars().skip(11);
    let data12 = contents.chars().skip(12);
    let data13 = contents.chars().skip(13);

    for l in izip!(data, data1, data2, data3, data4, data5, data6, data7, data8, data9, data10, data11, data12, data13) {
        let mut set = HashSet::new();
        set.insert(l.0);
        set.insert(l.1);
        set.insert(l.2);
        set.insert(l.3);
        set.insert(l.4);
        set.insert(l.5);
        set.insert(l.6);
        set.insert(l.7);
        set.insert(l.8);
        set.insert(l.9);
        set.insert(l.10);
        set.insert(l.11);
        set.insert(l.12);
        set.insert(l.13);
        if set.len() == 14 {
            break;
        }
        part_2 += 1;
    }


    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);

    //////////////////////////////////////////////////////////////////////////////////
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    println!("Rust program executed in {:?}", duration);
}
