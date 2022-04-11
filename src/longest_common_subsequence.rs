use crate::grid::Grid;
use std::cmp::max;

/// Returns the length of the longest common subsequence of two string files.
/// # Example
///
/// Basic usage:
/// ```
/// let vec_file_one = vec!["a", "b", "c", "d", "e"];
/// let vec_file_two = vec!["a", "b", "c", "d", "e", "f"];
/// let grid = create_lcs_grid(&vec_file_one, &vec_file_two);
/// ```
pub fn create_lcs_grid(a: &[String], b: &[String]) -> Grid {
    let a_len = a.len();
    let b_len = b.len();
    let grid = Grid::new(a_len + 1, b_len + 1);

    for i in 0..a_len {
        grid.set(i, 0, 0);
    }

    for j in 0..b_len {
        grid.set(0, j, 0);
    }

    for (i, element) in a.iter().enumerate().take(a_len - 1) {
        for (j, element2) in b.iter().enumerate().take(b_len - 1) {
            if element == element2 {
                grid.set(i + 1, j + 1, grid.get(i, j) + 1);
            } else {
                grid.set(i + 1, j + 1, max(grid.get(i + 1, j), grid.get(i, j + 1)));
            }
        }
    }

    grid
}

/// Prints the longest common subsequence of two string files.
/// The LCS Grid is needed to print the LCS.
///
/// # Example
///
/// Basic usage:
/// ```
/// let vec_file_one = vec!["a", "b", "c", "d", "e"];
/// let vec_file_two = vec!["a", "b", "c", "d", "e", "f"];
/// let grid = create_lcs_grid(&vec_file_one, &vec_file_two);
/// print_lcs_grid(grid, vec_file_one, vec_file_two, vec_file_one.len(), vec_file_two.len());
/// ```
pub fn print_lcs_grid(grid: Grid, a: &[String], b: &[String], i: usize, j: usize) {
    if i > 0 && j > 0 && (a[i - 1] == b[j - 1]) {
        print_lcs_grid(grid, a, b, i - 1, j - 1);
        print!(" {}", a[i - 1]);
        println!();
    } else if j > 0 && (grid.get(i, j - 1) >= grid.get(i - 1, j)) {
        print_lcs_grid(grid, a, b, i, j - 1);
        print!("> {}", b[j - 1]);
        println!();
    } else if i > 0 && (j == 0 || grid.get(i, j - 1) < grid.get(i - 1, j)) {
        print_lcs_grid(grid, a, b, i - 1, j);
        print!("< {}", a[i - 1]);
        println!();
    } else {
        print!("");
        println!();
    }
}

#[test]
fn test_lcs_creation_size_successful(){
    let vec_file_one: &Vec<String> = &vec!["a".to_string(), "b".to_string()];
    let vec_file_two: &Vec<String> = &vec!["a".to_string()];
    let grid = create_lcs_grid(vec_file_one, vec_file_two);
    assert_eq!(grid.get_width(), 3);
    assert_eq!(grid.get_height(), 2);
}