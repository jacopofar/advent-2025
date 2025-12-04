use std::io;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct Rc {
    r: usize,
    c: usize,
}

fn adjacents(rc: Rc, max_r: usize, max_c: usize) -> HashSet<Rc>{
    let mut ret = HashSet::<Rc>::new();

    for dr in -1isize..=1{
        for dc in -1isize..=1{
            if dr != 0 || dc != 0{
                let check_r:isize = (rc.r as isize) + dr;
                if check_r < 0 || check_r > (max_r as isize){
                    continue
                }
                let check_c:isize = (rc.c as isize) + dc;
                if check_c < 0|| check_c > (max_c as isize){
                    continue
                }
                ret.insert(Rc{r: check_r as usize, c: check_c as usize});
            }
        }
    }
    ret
}

fn get_accessible_rolls(rolls: &HashSet<Rc>, total_rows: usize, total_cols: usize)-> HashSet<Rc>{
    let mut accessible_rolls = HashSet::<Rc>::new();

    for r in 0..total_rows{
        for c in 0..total_rows{
            let this_roll = Rc{r:r, c:c};
            if !rolls.contains(&this_roll){
                continue;
            }
            let mut count_adj = 0;
            for ad in adjacents(Rc{r:r, c:c}, total_rows, total_cols){
                if rolls.contains(&ad){
                    count_adj += 1
                }
            }
            if count_adj < 4 {
                accessible_rolls.insert(this_roll);
            }
        }
    }
    accessible_rolls
}

fn main() {
    let mut rolls = HashSet::<Rc>::new();
    let mut total_rows = 0;
    let mut total_cols = 0;

    for (ridx, line_maybe) in io::stdin().lines().enumerate() {
        let line = line_maybe.ok().unwrap().trim().to_string();
        for (cidx, elem) in line.chars().enumerate(){
            if elem == '@'{
                rolls.insert(Rc{r: ridx, c: cidx});
            }
        }
        if ridx == 0 {
            total_cols = line.len();
        }
        total_rows = ridx + 1;
    }
    // println!("{:?} ", rolls);
    // println!("{:?} {:?}", total_rows, total_cols);
    let mut accessible_rolls = get_accessible_rolls(&rolls, total_rows, total_cols);
    println!("accessible rolls: {:?}", accessible_rolls);
    println!("total rolls: {:?}", rolls.len());
    let initial_rolls = rolls.len();
    let sol1: usize = accessible_rolls.len();
    while accessible_rolls.len() > 0{
        println!("# accessible rolls: {:?}", accessible_rolls.len());
        println!("# rolls: {:?}", rolls.len());
        rolls = rolls.difference(&accessible_rolls).copied().collect();
        println!("# new rolls: {:?}", rolls.len());
        accessible_rolls = get_accessible_rolls(&rolls, total_rows, total_cols);
        println!("# new accessible rolls: {:?}", accessible_rolls.len());

    }
    let sol2: usize = initial_rolls - rolls.len();

    println!("SOLUTION PART 1: {} ", sol1);
    println!("SOLUTION PART 2: {} ", sol2);

}