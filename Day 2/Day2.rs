use std::fs;

fn main() {
	let input = fs::read_to_string("Input2.txt").unwrap();
	let mut score = 0;

	for line in input.lines() {
        let elf_play = line.chars().nth(0).unwrap();
        let my_play = line.chars().nth(2).unwrap();

        match my_play{
            'X'=> score += 1,
            'Y'=> score += 2,
            'Z'=> score += 3,
            _=> println!("Unmatched!"),
        }   

        if elf_play == 'A'{
            match my_play{
                'X'=> score += 3,
                'Y'=> score += 6,
                'Z'=> score += 0,
                _=> println!("Unmatched!"),
            }  
        }
        else if elf_play == 'B'{
            match my_play{
                'X'=> score += 0,
                'Y'=> score += 3,
                'Z'=> score += 6,
                _=> println!("Unmatched!"),
            }  

        }
        else {
            match my_play{
                'X'=> score += 6,
                'Y'=> score += 0,
                'Z'=> score += 3,
                _=> println!("Unmatched!"),
            }  

        }


        println!("Elf Play:{}", elf_play);
        println!("My Play:{}", my_play);
        println!("Score is:{}", score);

	}

 
}

