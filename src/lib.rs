#![no_std]

use rand::prelude::*;
use wasm_bindgen::prelude::*;

mod color_scale;
use color_scale::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const WIDTH: usize = 256;
const HEIGHT: usize = 112;
const LEN: usize = WIDTH * HEIGHT;

#[wasm_bindgen]
pub struct Fire {
    data: [usize; LEN],
    texture: [Pixel; LEN],
    rng: SmallRng,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum ColorMode {
    Red,
    YellowGreen,
    BlueGreen,
    Blue,
    Purple,
    Pink,
}

#[allow(clippy::len_without_is_empty, clippy::new_without_default)]
#[wasm_bindgen]
impl Fire {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut data = [0; LEN];
        data[LEN - WIDTH..].fill(FIRE_PROGRESS.len() - 1);

        let mut texture = [FIRE_PROGRESS[0]; LEN];
        texture[LEN - WIDTH..].fill(*FIRE_PROGRESS.last().unwrap());

        Self {
            data,
            texture,
            rng: SmallRng::from_entropy(),
        }
    }

    pub fn tick(&mut self, spread_factor: f64, height_factor: f64, color: ColorMode) {
        for row in 1..HEIGHT {
            for col in 0..WIDTH {
                let spread_rand = if self.rng.gen_bool(spread_factor) {
                    *[0_usize, 2, 3].choose(&mut self.rng).unwrap_or(&1)
                } else {
                    1
                };

                let decrease_rand = if self.rng.gen_bool(height_factor) {
                    1
                } else {
                    0
                };

                let get_index = |row, column| row * WIDTH + column;

                let src = get_index(row, col);
                let dst = get_index(
                    row - 1,
                    core::cmp::min(WIDTH - 1, (col + spread_rand).saturating_sub(1)),
                );

                self.data[dst] = self.data[src].saturating_sub(decrease_rand);
            }
        }

        self.compute_texture(color);
    }

    fn compute_texture(&mut self, color: ColorMode) {
        for i in 0..self.data.len() {
            let mut val = FIRE_PROGRESS[self.data[i]];

            use core::mem::swap;
            match color {
                ColorMode::Red => {}
                ColorMode::BlueGreen => {
                    let tmp = val.red;
                    val.red = val.blue;
                    val.blue = val.green;
                    val.green = tmp;
                }
                ColorMode::Purple => {
                    let tmp = val.red;
                    val.red = val.green;
                    val.green = val.blue;
                    val.blue = tmp;
                }
                ColorMode::Blue => {
                    swap(&mut val.red, &mut val.blue);
                }
                ColorMode::YellowGreen => {
                    swap(&mut val.red, &mut val.green);
                }
                ColorMode::Pink => {
                    swap(&mut val.blue, &mut val.green);
                }
            };

            self.texture[i] = val;
        }
    }

    #[wasm_bindgen(getter)]
    pub fn texture(&self) -> *const Pixel {
        self.texture.as_ptr()
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> usize {
        WIDTH
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> usize {
        HEIGHT
    }

    #[wasm_bindgen(getter)]
    pub fn len(&self) -> usize {
        LEN
    }
}
