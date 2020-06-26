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
        for row in 0..self.height - 1 {
            for col in 0..self.width {
                let i = self.get_index(row, col);
                let r = if self.rng.gen_ratio(3, 4) { 1 } else { 0 };
                self.data[i] = self.data[self.get_index(row + 1, col)].saturating_sub(r);
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
