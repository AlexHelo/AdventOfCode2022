use std::fs;



fn main() {
	let input = fs::read_to_string("input.txt").unwrap();

    let mut tmp_dir: Vec<u32> = Vec::new();
    let mut directories: Vec<u32> = Vec::new();
    
    for line in input.lines() {
        match line.split(' ').collect::<Vec<&str>>().as_slice() {
            ["$", "cd", ".."] => directories.push(tmp_dir.pop().unwrap()),
            ["$", "cd", _] => tmp_dir.push(0),
            [size, _] => {
                // Getting rid of "dir ..." and "$ ls" here
                if let Ok(num) = size.parse::<u32>() {
                    tmp_dir.iter_mut().for_each(|n| *n += num)
                }
            }
            [..] => unreachable!(),
        }
    }
    directories.extend(tmp_dir);

    let result = directories
    .iter()
    .filter(|&&size| size < 100_000)
    .sum::<u32>();

    println!("{}",result.to_string());

    let root = directories.iter().max().unwrap();  // Biggest directory
    let required = root + 30_000_000 - 70_000_000;  // Required size for dir
    let result = directories
        .iter()
        .filter(|&&dir| dir >= required)
        .min()
        .unwrap();

    println!("{}",result.to_string());
}

