use std::io;
use std::cmp::{min, max};

fn collapse_ranges(ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut new_ranges = ranges.clone();
    for ia in 0..ranges.len(){
        let (af, at) = ranges[ia];
        for ib in ia+1..ranges.len(){
            let (bf, bt) = ranges[ib];
            // println!("testing: {:?}-{:?}    {:?}-{:?}", af, at, bf, bt);

            if min(at, bt) >= max(af, bf) {
                // println!("overlap: {:?}-{:?}    {:?}-{:?}", af, at, bf, bt);
                new_ranges[ia] = (min(af, bf), max(at, bt));
                new_ranges.remove(ib);
                return new_ranges;
            }
        }
    }
    ranges
}

fn main() {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();

    for line_maybe in io::stdin().lines() {
        let line = line_maybe.ok().unwrap().trim().to_string();
        if line == ""{
            continue
        }
        // on wasmtime it gives a weird error if I remove this line WTF
        // also depends on the input
        println!(" ");
        if line.contains('-'){
            let ps: Vec<&str> = line.split('-').collect();
            let f = ps[0].parse::<u64>().unwrap();
            let t = ps[1].parse::<u64>().unwrap();
            ranges.push((f, t));
        }
        else{
            let ing = line.parse::<u64>().unwrap();
            ingredients.push(ing);
        }
    }
    // println!("{:?} ", ranges);
    // println!("{:?} ", ingredients);
    let mut sol1 = 0;
    for ing in ingredients {
        for (f, t) in &ranges{
            if *f <= ing && ing <= *t {
                sol1 += 1;
                break;
            }
        }
    }
    println!("before: {:?}", ranges);
    let mut current_best = ranges.len();
    let mut new_ranges = collapse_ranges(ranges);
    while new_ranges.len() < current_best{
        current_best = new_ranges.len();
        new_ranges = collapse_ranges(new_ranges);
        println!("now: {:?}", new_ranges);
    }
    let mut sol2 = 0;
    for (f, t) in new_ranges {
        sol2 += (t - f) + 1;
    }


    println!("SOLUTION PART 1: {} ", sol1);
    println!("SOLUTION PART 2: {} ", sol2);

}