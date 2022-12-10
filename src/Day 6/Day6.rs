use std::fs;
use std::convert::TryInto;


fn main() {
	let input = fs::read_to_string("input.txt").unwrap();

    println!("{}",find_unique_window(&input, 4));
    println!("{}",find_unique_window(&input, 14));

}

fn find_unique_window(input: &str, window_size: usize) -> usize {
    input
        .as_bytes()
        .windows(window_size)
        .enumerate()
        .find_map(|(index, win)| {
            let set: u32 = win.iter().fold(0, |acc, &c| acc | 1 << (c - b'a'));
            if set.count_ones() == window_size.try_into().unwrap() {
                return Some(index + window_size);
            }
            return None;
        })
        .unwrap()
}

