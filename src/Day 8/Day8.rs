use std::fs;

fn main() {

    let input = fs::read_to_string("input.txt").unwrap();

    println!("{}",part1(&input));
    println!("{}",part2(&input));


}

pub fn part1(input: &str) -> usize {
    solve(input).0
}

pub fn part2(input: &str) -> usize {
    solve(input).1
}

fn solve(input: &str) -> (usize, usize) {
    std::iter::once(parse_trees(input))
        .map(|grid| (grid.len(), grid[0].len(), grid))
        .fold((0, 0), |_, (rows, cols, grid)|
            (0..rows).flat_map(|row| std::iter::repeat(row).zip(0..cols))
                .map(|(row, col)| {
                    match (
                        scan_above(&grid, row, col, grid[row][col]),
                        scan_below(&grid, row, col, grid[row][col]),
                        scan_left(&grid, row, col, grid[row][col]),
                        scan_right(&grid, row, col, grid[row][col]),
                    ) {
                        (
                            (1.., a),
                            (1.., b),
                            (1.., c),
                            (1.., d),
                        ) => (false, a * b * c * d),
                        (
                            (_, a),
                            (_, b),
                            (_, c),
                            (_, d),
                        ) => (true, a * b * c * d),
                    }
                })
                .fold((0, 0), |(visible, highest), (clear, score)| {
                    (visible + clear as usize, highest.max(score))
                })
        )
}

fn parse_trees(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|l| l.as_bytes()
            .iter()
            .map(|d| d - b'0')
            .collect())
        .collect()
}

fn score_tree(obstructions: usize, score: usize, height: u8, ceiling: u8) -> (usize, usize) {
    (obstructions - (height < ceiling) as usize, score + 1)
}

fn scan_above(grid: &[Vec<u8>], row: usize, col: usize, height: u8) -> (usize, usize) {
    match row {
        0 => (0, 0),
        _ => std::iter::once(grid.iter()
                .take(row)
                .rev()
                .map(|r| r[col])
                .collect::<Vec<_>>())
            .map(|c| (c.iter().position(|&c| c >= height), c))
            .flat_map(|(i, c)| c[..=i.unwrap_or(row - 1)].to_vec())
            .fold((row, 0), |(o, s), h| score_tree(o, s, h, height))
    }
}

fn scan_below(grid: &[Vec<u8>], row: usize, col: usize, height: u8) -> (usize, usize) {
    match (row, grid.len() - 1) {
        (_, len) if row == len => (0, 0),
        (_, len) => std::iter::once(grid.iter()
                .skip(row + 1)
                .map(|r| r[col])
                .collect::<Vec<_>>())
            .map(|c| (c.iter().position(|&c| c >= height), c))
            .flat_map(|(i, c)| match i {
                Some(i) => c[..=i].to_vec(),
                None => c,
            })
            .fold((len - row, 0), |(o, s), h| score_tree(o, s, h, height))
    }
}

fn scan_left(grid: &[Vec<u8>], row: usize, col: usize, height: u8) -> (usize, usize) {
    match col {
        0 => (0, 0),
        _ => std::iter::once(grid[row].iter()
                .take(col)
                .rev()
                .collect::<Vec<_>>())
            .map(|c| (c.iter().position(|&&c| c >= height), c))
            .flat_map(|(i, c)| c[..=i.unwrap_or(col - 1)].to_vec())
            .fold((col, 0), |(o, s), &h| score_tree(o, s, h, height))
    }
}

fn scan_right(grid: &[Vec<u8>], row: usize, col: usize, height: u8) -> (usize, usize) {
    match (col, grid[0].len() - 1) {
        (_, len) if col == len => (0, 0),
        (_, len) => std::iter::once(grid[row].iter()
                .skip(col + 1)
                .collect::<Vec<_>>())
            .map(|c| (c.iter().position(|&&c| c >= height), c))
            .flat_map(|(i, c)| match i {
                Some(i) => c[..=i].to_vec(),
                None => c,
            })
            .fold((len - col, 0), |(o, s), &h| score_tree(o, s, h, height))
    }
}