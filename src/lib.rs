use wasm_bindgen::prelude::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

const ALIVE_COLOR: Cell = Cell {
    red: 0,
    green: 0,
    blue: 0,
    alpha: 255,
};

const DEAD_COLOR: Cell = Cell {
    red: 255,
    green: 255,
    blue: 255,
    alpha: 255,
};

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    buffer: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Self {
        let cells: Vec<Cell> = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    ALIVE_COLOR
                } else {
                    DEAD_COLOR
                }
            })
            .collect();
        let buffer = cells.clone();
        Universe {
            width,
            height,
            cells,
            buffer,
        }
    }

    pub fn tick(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (ALIVE_COLOR, x) if x < 2 => DEAD_COLOR,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (ALIVE_COLOR, 2) | (ALIVE_COLOR, 3) => ALIVE_COLOR,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (ALIVE_COLOR, x) if x > 3 => DEAD_COLOR,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (DEAD_COLOR, 3) => ALIVE_COLOR,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                self.buffer[idx] = next_cell;
            }
        }

        std::mem::swap(&mut self.buffer, &mut self.cells);
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += if self.cells[idx] == ALIVE_COLOR { 1 } else { 0 };
            }
        }
        count
    }
}
