use std::io;
fn is_invalid(e: u64, only_two: bool) -> bool{
    let repr = e.to_string();
	let sl = repr.len();

	for split_length_candidate in (1..=(sl / 2)).rev() {
        // println!("split_length_candidate {:?}", split_length_candidate);

		// non-exact split
		if sl%split_length_candidate != 0 {
			continue
		}
		if repr == repr[0..split_length_candidate].repeat(sl/split_length_candidate) {
			return true
		}
		if only_two {
			// stop iterating, they asked for only a split into two and this is not it
			return false
		}
	}

	return false
}
fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    buffer = buffer.trim().to_string();
    println!("Read {} bytes", buffer.len());
    // we can just calculate the total while iterating on the elements
    // but I wanted to put them in a vector to try it out
    let mut problem: Vec<(u64, u64)> = Vec::new();
    for interval in buffer.split(",") {
        let extremes: Vec<&str> = interval.split("-").collect();

        problem.push(
            (
                extremes[0].parse::<u64>().unwrap(),
                extremes[1].parse::<u64>().unwrap()
            )
        );
    }
    let mut sol1: u64 = 0;
    let mut sol2: u64 = 0;

    for (f, t) in problem{
        for x in f..=t{

            if is_invalid(x, true){
                sol1 += x;
                sol2 += x;
            }
            else{

                if is_invalid(x, false){
                    sol2 += x;
                }
            }
        }
    }
    println!("SOLUTION PART 1: {} ", sol1);
    println!("SOLUTION PART 2: {} ", sol2);

}