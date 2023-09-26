use std::time::Instant;
use std::fs;
use std::collections::HashSet;
#[derive(Debug, Clone, Copy)]
struct Node {
    row: i32,
    col: i32,
}
impl Default for Node {
    fn default() -> Self {
        Self { row: 0, col: 0 }
    }
}

fn update_tail(tail: &mut Node, head : Node) {
    let delta_row = head.row - tail.row;
    let delta_col = head.col - tail.col;
    match (delta_row, delta_col) {
        (0,2) => tail.col += 1,
        (0,-2) => tail.col -= 1,
        (2,0) => tail.row += 1,
        (-2,0) => tail.row -= 1,
        (1,2) => {
            tail.row += 1;
            tail.col += 1;
        },
        (1,-2) => {
            tail.row += 1;
            tail.col -= 1;
        },
        (-1,2) => {
            tail.row -= 1;
            tail.col += 1;
        },
        (-1,-2) => {
            tail.row -= 1;
            tail.col -= 1;
        },
        (2,1) => {
            tail.row += 1;
            tail.col += 1;
        },
        (2,-1) => {
            tail.row += 1;
            tail.col -= 1;
        },
        (-2,1) => {
            tail.row -= 1;
            tail.col += 1;
        },
        (-2,-1) => {
            tail.row -= 1;
            tail.col -= 1;
        },
        (2,2) => {
            tail.row += 1;
            tail.col += 1;
        },
        (2,-2) => {
            tail.row += 1;
            tail.col -= 1;
        },
        (-2,2) => {
            tail.row -= 1;
            tail.col += 1;
        },
        (-2,-2) => {
            tail.row -= 1;
            tail.col -= 1;
        },
        _ => return,
    }
}

fn main() {
    let start_time = Instant::now();
    //////////////////////////////////////////////////////////////////////////////////
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut visited_1: HashSet<(i32, i32)> = HashSet::new();
    let mut visited_2: HashSet<(i32, i32)> = HashSet::new();
    // let (mut head, mut tail) = (Node{row: 0, col: 0}, Node{row: 0, col: 0});
    let mut knots = vec![Node::default(); 10];
    visited_1.insert((0,0));
    visited_2.insert((0,0));

    for line in lines {
        let mut splitted_line = line.split_ascii_whitespace();
        let dir = splitted_line.next().unwrap();
        let steps = splitted_line.next().unwrap().parse::<i32>().unwrap();
        match dir {
            "U" => {
                for _ in 0..steps {
                    knots[0].row += 1;
                    for i in 1..knots.len() {
                        let head = knots[i - 1]; 
                        let tail = &mut knots[i]; 
                        update_tail(tail, head);
                    }
                    visited_1.insert((knots[1].row, knots[1].col));
                    visited_2.insert((knots[9].row, knots[9].col));
                }
            },
            "D" => {
                for _ in 0..steps {
                    knots[0].row -= 1;
                    for i in 1..knots.len() {
                        let head = knots[i - 1]; 
                        let tail = &mut knots[i]; 
                        update_tail(tail, head);
                    }
                    visited_1.insert((knots[1].row, knots[1].col));
                    visited_2.insert((knots[9].row, knots[9].col));
                }
            },
            "R" => {
                for _ in 0..steps {
                    knots[0].col += 1;
                    for i in 1..knots.len() {
                        let head = knots[i - 1]; 
                        let tail = &mut knots[i]; 
                        update_tail(tail, head);
                    }
                    visited_1.insert((knots[1].row, knots[1].col));
                    visited_2.insert((knots[9].row, knots[9].col));
                }
            },
            "L" => {
                for _ in 0..steps {
                    knots[0].col -= 1;
                    for i in 1..knots.len() {
                        let head = knots[i - 1]; 
                        let tail = &mut knots[i]; 
                        update_tail(tail, head);
                    }
                    visited_1.insert((knots[1].row, knots[1].col));
                    visited_2.insert((knots[9].row, knots[9].col));
                }
            },
            _ => println!("Error"),
        }
    }
    println!("Part 1: {}", visited_1.len());
    println!("Part 2: {}", visited_2.len());
    
    //////////////////////////////////////////////////////////////////////////////////
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    println!("Rust program executed in {:?}", duration);
}
