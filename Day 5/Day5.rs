use std::fs;

fn main() {
	let input = fs::read_to_string("Input.txt").unwrap();

	let parts: Vec<_> = input.split("\n\n").collect();
	let init: Vec<_> = parts[0].lines().collect();

	let mut stacks: Vec<Vec<_>> = Vec::new();

	for _ in 0..((init[0].len() + 1) / 4) {
		stacks.push(Vec::new());
	}

	for i in (0..(init.len() - 1)).rev() {
		let line: Vec<_> = init[i].chars().collect();
		let mut place = 0;
		for part in line.chunks(4) {
			if part[1] != ' ' {
				stacks[place].push(part[1]);
			}
			place += 1;
		}
	}
    let mut iterate = 1;

	for line in parts[1].lines() {

        let wordz: Vec<_> = parts[1].split('\n').collect();

        if (iterate == 502) {
            break;
        }

   
       
		let words: Vec<_> = wordz[iterate].split(' ').collect();
        
		let num = words[1].parse::<u32>().unwrap();
		let from = words[3].parse::<usize>().unwrap() - 1;
		let to = words[5].parse::<usize>().unwrap() - 1;
		
		for _ in 0..num {
			if stacks[from].len() > 0 {
				let item = stacks[from].pop().unwrap();
				stacks[to].push(item);
			}
		}
        iterate +=1;
	}		
	for mut vec in stacks {
		if vec.len() > 0 {
			print!("{}", vec.pop().unwrap());
		}
	}
	println!();
}