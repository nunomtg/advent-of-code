use std::time::Instant;
use std::collections::VecDeque;

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u64>,
    inspect_func: fn(u64) -> u64,
    test_ndiv: u64,
    throw_monkey: (usize, usize),
    inspect_counter: u64,
}

impl Monkey {
    fn inspect(&mut self, item: u64) -> u64{
        self.inspect_counter += 1;
        return (self.inspect_func)(item);
    }
    fn test(&mut self, item: u64) -> usize{
        if item % self.test_ndiv == 0 {
            return self.throw_monkey.0;
        }
        return self.throw_monkey.1;
    }
}

fn get_result(vec: Vec<Monkey>) -> u64 {
    let mut first = 0;
    let mut second = 0;
    for m in vec {
        let counter = m.inspect_counter;
        if counter > first {
            second = first;
            first = counter;
        } else if counter > second {
            second = counter;
        }
    }
    return first*second;
}

fn main() {
    let start_time = Instant::now();
    //////////////////////////////////////////////////////////////////////////////////
    let monkey1 = Monkey {
        items: VecDeque::from(vec![97, 81, 57, 57, 91, 61]),
        inspect_func: |x| x*7,
        test_ndiv: 11,
        throw_monkey: (5, 6),
        inspect_counter: 0,
    };
    let monkey2 = Monkey {
        items: VecDeque::from(vec![88, 62, 68, 90]),
        inspect_func: |x| x*17,
        test_ndiv: 19,
        throw_monkey: (4, 2),
        inspect_counter: 0,
    };
    let monkey3 = Monkey {
        items: VecDeque::from(vec![74, 87]),
        inspect_func: |x| x+2,
        test_ndiv: 5,
        throw_monkey: (7, 4),
        inspect_counter: 0,
    };
    let monkey4 = Monkey {
        items: VecDeque::from(vec![53, 81, 60, 87, 90, 99, 75]),
        inspect_func: |x| x+1,
        test_ndiv: 2,
        throw_monkey: (2, 1),
        inspect_counter: 0,
    };
    let monkey5 = Monkey {
        items: VecDeque::from(vec![57]),
        inspect_func: |x| x+6,
        test_ndiv: 13,
        throw_monkey: (7, 0),
        inspect_counter: 0,
    };
    let monkey6 = Monkey {
        items: VecDeque::from(vec![54, 84, 91, 55, 59, 72, 75, 70]),
        inspect_func: |x| x*x,
        test_ndiv: 7,
        throw_monkey: (6, 3),
        inspect_counter: 0,
    };
    let monkey7 = Monkey {
        items: VecDeque::from(vec![95, 79, 79, 68, 78]),
        inspect_func: |x| x+3,
        test_ndiv: 3,
        throw_monkey: (1, 3),
        inspect_counter: 0,
    };
    let monkey8 = Monkey {
        items: VecDeque::from(vec![61, 97, 67]),
        inspect_func: |x| x+4,
        test_ndiv: 17,
        throw_monkey: (0, 5),
        inspect_counter: 0,
    };


    let ceiling: u64 = 11*19*5*2*13*7*3*17;
    
    #[allow(non_snake_case)]
    let mut M1 = vec![monkey1, monkey2, monkey3, monkey4, monkey5, monkey6, monkey7, monkey8];
    #[allow(non_snake_case)]
    let mut M2 = M1.clone();

    for _ in 0..20 {
        for i in 0..M1.len() {
            let mut m = M1[i].clone();
            while !m.items.is_empty() {
                let mut item = m.items.pop_front().unwrap();
                item = m.inspect(item) / 3;
                let next_monkey = m.test(item);
                M1[next_monkey].items.push_back(item);
            }
            M1[i] = m;
        }
    }

    for _ in 0..10_000 {
        for i in 0..M2.len() {
            let mut m = M2[i].clone();
            while !m.items.is_empty() {
                let mut item = m.items.pop_front().unwrap();
                item = m.inspect(item) % ceiling;
                let next_monkey = m.test(item);
                M2[next_monkey].items.push_back(item);
            }
            M2[i] = m;
        }
    } 


    println!("Result: {}", get_result(M1));
    println!("Result: {}", get_result(M2));

    
    //////////////////////////////////////////////////////////////////////////////////
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    println!("Rust program executed in {:?}", duration);
}
