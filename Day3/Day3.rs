use std::fs;

fn main() {
	let input = fs::read_to_string("Input.txt").unwrap();
	let mut half1;
    let mut half2;
    let mut matches = vec![];

	for line in input.lines() {
        half1 = &line[..line.len()/2];
        half2 = &line[line.len()/2..];

        for c in half2.chars() {
            if half1.find(c).is_some(){
                matches.push(c);
                break;
            }
        }
	}

    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut total = 0;

    for x in matches.iter() {
        total = total + alphabet.find(*x).unwrap() + 1;
        println!("Total {}", total);
    }





}

