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
pub enum ColorMode {
    Red,
    Green,
    Blue,
    Purple,
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

    pub fn tick(&mut self, spread_factor: f64, height_factor: f64, color: ColorMode) {
        for row in 1..self.height {
            for col in 0..self.width {
                let spread_rand = if self.rng.gen_bool(spread_factor) {
                    *[0usize, 2, 3].choose(&mut self.rng).unwrap()
                } else {
                    1
                };

                let decrease_rand = if self.rng.gen_bool(height_factor) {
                    1
                } else {
                    0
                };

                let src = self.get_index(row, col);
                let dst = self.get_index(
                    row - 1,
                    std::cmp::min(self.width - 1, (col + spread_rand).saturating_sub(1)),
                );

                self.data[dst] = self.data[src].saturating_sub(decrease_rand);
            }
        }

        self.compute_texture(color);
    }

    fn compute_texture(&mut self, color: ColorMode) {
        for i in 0..self.data.len() {
            let mut val = FIRE_PROGRESS[self.data[i]];

            use std::mem::swap;
            match color {
                ColorMode::Red => {}
                ColorMode::Green => {
                    let tmp = val.red;
                    val.red = val.blue;
                    val.blue = val.green;
                    val.green = tmp;
                }
                ColorMode::Blue => {
                    swap(&mut val.red, &mut val.blue);
                }
                ColorMode::Purple => {
                    let tmp = val.red;
                    val.red = val.green;
                    val.green = val.blue;
                    val.blue = tmp;
                }
            };
            self.texture[i] = val;
        }
    }

    pub fn texture(&self) -> *const Cell {
        self.texture.as_ptr()
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        (row * self.width + column) as usize
    }
}
