use rand::prelude::*;
use wasm_bindgen::prelude::*;

mod color_scale;
use color_scale::*;

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    data: Vec<usize>,
    texture: Vec<Cell>,
    rng: SmallRng,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: usize, height: usize) -> Self {
        let len = (width * height) as usize;

        use std::iter::repeat;
        let data: Vec<_> = repeat(0)
            .take(len - width)
            .chain(repeat(FIRE_PROGRESS.len() - 1).take(width))
            .collect();

        let texture = data.iter().map(|&x| FIRE_PROGRESS[x]).collect();

        Universe {
            width,
            height,
            data,
            texture,
            rng: SmallRng::from_entropy(),
        }
    }

    pub fn tick(&mut self) {
        for row in 1..self.height {
            for col in 0..self.width {
                let spread_rand = [0usize, 1, 2, 3].choose(&mut self.rng).unwrap();
                let decrease_rand = if self.rng.gen() { 1 } else { 0 };

                let src = self.get_index(row, col);
                let dst = self.get_index(
                    row - 1,
                    std::cmp::min(self.width - 1, (col + spread_rand).saturating_sub(1)),
                );

                self.data[dst] = self.data[src].saturating_sub(decrease_rand);
            }
        }

        self.compute_texture();
    }

    fn compute_texture(&mut self) {
        for i in 0..self.data.len() {
            self.texture[i] = FIRE_PROGRESS[self.data[i]];
        }
    }

    pub fn texture(&self) -> *const Cell {
        self.texture.as_ptr()
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        (row * self.width + column) as usize
    }
}
