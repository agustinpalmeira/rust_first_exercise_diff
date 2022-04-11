mod file_reader;
mod grid;
mod longest_common_subsequence;
use crate::longest_common_subsequence::{create_lcs_grid, print_lcs_grid};

/// The current project is about the implementation of the following exercise:
/// `<https://taller-1-fiuba-rust.github.io/guia_2.html#ejercicio-2>`
/// - The goal is to implement a function that returns the difference between two text files.

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Usage: cargo run <first_file_path> <second_file_path>");
        return;
    }

    let first_file_path = &args[1];
    let second_file_path = &args[2];

    println!("1st file path: {}", first_file_path);
    println!("2nd file path: {}", second_file_path);

    let vec_file_one: &Vec<String> = &file_reader::read_file_lines(first_file_path);
    let vec_file_two: &Vec<String> = &file_reader::read_file_lines(second_file_path);
    let grid = create_lcs_grid(vec_file_one, vec_file_two);

    println!(
        "A grid of {} lines and {} columns was created",
        grid.get_width(),
        grid.get_height()
    );

    print_lcs_grid(
        grid,
        vec_file_one,
        vec_file_two,
        vec_file_one.len(),
        vec_file_two.len(),
    );
}
