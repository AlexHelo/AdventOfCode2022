use std::fs;

fn main() {
	let input = fs::read_to_string("Input.txt").unwrap();
    let lines: Vec<_> = input.lines().collect();
    let mut matches = vec![];

    for i in (0..lines.len()).into_iter().step_by(3) {
        
        let line1: Vec<char> = lines[i].chars().collect();
		let line2: Vec<char> = lines[i + 1].chars().collect();
		let line3: Vec<char> = lines[i + 2].chars().collect();

        for ch in line1 {
            if line2.contains(&ch) && line3.contains(&ch) {
				matches.push(ch);
                break;
			}
        }
	}

    println!("Matches {:?}", matches);

    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut total = 0;

    for x in matches.iter() {
        total = total + alphabet.find(*x).unwrap() + 1;
        println!("Total {}", total);
    }


}

