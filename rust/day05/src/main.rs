extern crate regex;
use std::time::Instant;
use std::fs;
use regex::Regex;

fn main() {
    let start_time = Instant::now();
    //////////////////////////////////////////////////////////////////////////////////
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    
    let lines: Vec<&str> = contents.split("\n").collect();
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let l1: Vec<char> = "BGSC".chars().collect();
    let l2: Vec<char> = "TMWHJNVG".chars().collect();
    let l3: Vec<char> = "MQS".chars().collect();
    let l4: Vec<char> = "BSLTWNM".chars().collect();
    let l5: Vec<char> = "JZFTVGWP".chars().collect();
    let l6: Vec<char> = "CTBGQHS".chars().collect();
    let l7: Vec<char> = "TJPBW".chars().collect();
    let l8: Vec<char> = "GDCZFTQM".chars().collect();
    let l9: Vec<char> = "NSHBPF".chars().collect();

    let mut lists_1 = vec![l1, l2, l3, l4, l5, l6, l7, l8, l9];
    let mut lists_2 = lists_1.clone();

    for line in lines {
        let captures: (&str, [&str; 3]) = re.captures(line).unwrap().extract();
        let qt = captures.1[0].parse::<usize>().unwrap();
        let from_ = captures.1[1].parse::<usize>().unwrap() - 1;
        let to_ = captures.1[2].parse::<usize>().unwrap() - 1;
        for _ in 0..qt {
            let c = lists_1[from_].pop().unwrap();
            lists_1[to_].push(c);
        }
        let len_l = lists_2[from_].len();
        let tmp: Vec<_> = lists_2[from_].drain(len_l-qt..).collect();
        lists_2[to_].extend(tmp);
    }

    print!("Part 1: ");
    for l in lists_1 {
        print!("{}", l[l.len()-1]);
    }
    print!("\nPart 2: ");
    for l in lists_2 {
        print!("{}", l[l.len()-1]);
    }
    println!("");


    //////////////////////////////////////////////////////////////////////////////////
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    println!("Rust program executed in {:?}", duration);
}
