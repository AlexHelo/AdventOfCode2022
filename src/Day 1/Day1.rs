use std::fs;

fn main() {
	let input = fs::read_to_string("Input1.txt").unwrap();
	let mut current = 0;
    let mut top = vec![];

	for line in input.lines() {
        if line.len() == 0 {
            top.push(current);
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
	}

    top.sort_by(|a, b| a.cmp(b));
	println!("No. 1 {}", top[0]);
    println!("Top3 {}", top[0]+top[1]+top[2]);
}

