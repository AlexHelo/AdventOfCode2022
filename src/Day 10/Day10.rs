use std::fs;

fn draw(cycle: i32, x: i32, t: &mut i32) {
    let col = cycle - 1;
    // Draw cell
    if col % 40  >= x - 1 && col % 40 <= x + 1 {
        print!("#");
    } else {
        print!(".");
    }
    // Start new row.
    if col % 40 == 39 {
        print!("\n");
    }
    // Claculate signal strength.
    if (cycle - 20) % 40 == 0 {
        let strength = x * cycle;
        *t += strength;
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let commands = input.split_terminator("\n").collect::<Vec<&str>>();

    let mut x = 1;
    let mut cycle = 0;
    let mut t = 0;

    for command in commands.iter() {
        if *command == "noop" {
            cycle += 1;
            draw(cycle, x, &mut t);
        } else {
            cycle += 1;
            draw(cycle, x, &mut t);
            cycle += 1;
            draw(cycle, x, &mut t);
            // addx
            let v = command.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
            x += v;
        }
    }

    println!("Part 1: {}", t);
}

