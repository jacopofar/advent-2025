use std::collections::HashMap;
use std::io;
use std::iter::FromIterator;

#[derive(PartialEq)]
enum OpType {
    Sum,
    Mul,
}

fn process_block(lines: &Vec<String>) -> u64 {
    let mut numbers: Vec<u64> = Vec::new();
    let op: char = lines.last().unwrap().chars().last().unwrap();
    for line in lines {
        let this_number: String = line.chars().filter(|c| c.is_digit(10)).collect();
        numbers.push(this_number.parse::<u64>().unwrap())
    }
    // meh, repeated code, but no non-verbose way to implement char -> optype
    // and optype(vec) -> result ?
    match op {
        '+' => numbers.iter().sum::<u64>(),
        '*' => numbers.iter().product::<u64>(),
        _ => panic!("Unknown operation '{}'", op),
    }
}

fn main() {
    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut all_chars = HashMap::<(usize, usize), char>::new();
    let mut ops: Vec<OpType> = Vec::new();

    let mut longest_line = 0;

    for (ridx, line_maybe) in io::stdin().lines().enumerate() {
        let untrim_line = line_maybe.ok().unwrap().to_string();
        if untrim_line.len() > longest_line {
            longest_line = untrim_line.len();
        }
        // store raw characters position, including spaces
        for (cidx, elem) in untrim_line.chars().enumerate() {
            all_chars.insert((ridx, cidx), elem);
        }
        let line = untrim_line.trim().to_string();
        if line == "" {
            continue;
        }
        let ps: Vec<&str> = line.split(' ').filter(|&x| x != "").collect();
        println!("elements: {:?} ", ps);

        // first line we see, create all vecs for numbers
        if numbers.len() == 0 {
            for _ in 0..ps.len() {
                numbers.push(Vec::<u64>::new());
            }
        }
        if ps[0] == "+" || ps[0] == "*" {
            for o in ps {
                ops.push(match o {
                    "+" => OpType::Sum,
                    "*" => OpType::Mul,
                    _ => panic!("Unknown operation '{}'", o),
                });
            }
        } else {
            for (idx, n) in ps.iter().enumerate() {
                numbers[idx].push(n.parse::<u64>().unwrap());
            }
        }
    }
    // transpose the characters to get the "cephalopod" representation
    let mut cephalopod_lines: Vec<Vec<&char>> = Vec::new();
    for _ in 0..longest_line {
        cephalopod_lines.push(Vec::new());
    }

    for cix in (0..longest_line).rev() {
        for rix in 0..=(numbers[0].len()) {
            cephalopod_lines[longest_line - cix - 1]
                .push(all_chars.get(&(rix, cix)).unwrap_or(&' '));
        }
    }

    /*
    now cephalopod_lines looks like this
      4
    431
    623+

    175
    581
     32*

    356
    24
    1  *

    we accumulate strings until an empty one is found, and give them to
    process_block, then do it a last time for the rmaining lines
    */
    let mut sol2 = 0;

    let mut lines_acc: Vec<String> = Vec::new();
    for line_v in cephalopod_lines {
        let l = String::from_iter(line_v);
        let l = l.trim();
        if l == "" {
            sol2 += process_block(&lines_acc);
            lines_acc.clear()
        } else {
            lines_acc.push(l.to_string());
        }
    }
    sol2 += process_block(&lines_acc);

    let mut sol1 = 0;
    for (idx, op) in ops.iter().enumerate() {
        if *op == OpType::Sum {
            sol1 += numbers[idx].iter().sum::<u64>();
        }
        if *op == OpType::Mul {
            sol1 += numbers[idx].iter().product::<u64>();
        }
    }

    println!("SOLUTION PART 1: {} ", sol1);
    println!("SOLUTION PART 2: {} ", sol2);
}
