use std::cell::Cell;

#[derive(Debug)]

/// A grid of cells.
/// width and height are the dimensions of the grid.
/// cells is a vector of cells, where the index of the cell in the vector is the index of the cell in the grid.
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell<i32>>,
}

impl Grid {
    /// Creates a new grid of the given width and height.
    pub fn new(width: usize, height: usize) -> Grid {
        let cells = (0..width * height).map(|_| Cell::new(0)).collect();
        Grid {
            width,
            height,
            cells,
        }
    }

    /// Returns the value of the cell at the given coordinates.
    pub fn get(&self, x: usize, y: usize) -> i32 {
        self.cells[y * self.width + x].get()
    }

    /// Sets the value of the cell at the given coordinates to the given value.
    pub fn set(&self, x: usize, y: usize, value: i32) {
        self.cells[y * self.width + x].set(value);
    }

    /// Returns the width of the grid.
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Returns the height of the grid.
    pub fn get_height(&self) -> usize {
        self.height
    }
}

#[test]
fn test_grid_creation_height_successful() {
    let grid = Grid::new(3, 3);
    assert_eq!(grid.get_height(), 3);
}

#[test]
fn test_grid_creation_width_successful() {
    let grid = Grid::new(3, 3);
    assert_eq!(grid.get_width(), 3);
}

#[test]
fn test_grid_set_value_successful() {
    let grid = Grid::new(3, 3);
    grid.set(1, 1, 1);
    assert_eq!(grid.get(1, 1), 1);
}
