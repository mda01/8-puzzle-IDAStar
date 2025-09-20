use std::fs;
fn main() {
    let puzzle_name = "test_puzzles/puzzle00.txt";
    let puzzle_str = fs::read_to_string(puzzle_name).expect("Could not read file");
    let mut puzzle_in = puzzle_str.splitn(2, '\n');
    let n = puzzle_in.next().expect("Empty file?");
    println!("N = {n}");
    let puzzle = puzzle_in.next().expect("Nothing after the size?");
    println!("{puzzle}");
}
