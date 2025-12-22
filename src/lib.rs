#![no_std]

extern crate alloc;

use alloc::rc::Rc;
use core::cell::Cell;

use rand::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

mod color_scale;
use color_scale::*;

#[global_allocator]
static ALLOC: lol_alloc::AssumeSingleThreaded<lol_alloc::FreeListAllocator> =
    unsafe { lol_alloc::AssumeSingleThreaded::new(lol_alloc::FreeListAllocator::new()) };

#[cfg_attr(not(test), panic_handler)]
#[cfg_attr(test, expect(dead_code))]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

const WIDTH: usize = 256;
const HEIGHT: usize = 112;
const LEN: usize = WIDTH * HEIGHT;

#[wasm_bindgen]
pub struct Fire {
    data: [u8; LEN],
    texture: [Pixel; LEN],
    rng: SmallRng,

    height: Rc<Cell<f64>>,
    spread: Rc<Cell<f64>>,
}

const _: () = {
    assert!(FIRE_PROGRESS.len() <= u8::MAX as usize);
};

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

#[expect(clippy::len_without_is_empty, clippy::new_without_default)]
#[wasm_bindgen]
impl Fire {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut data = [0; LEN];
        data[LEN - WIDTH..].fill(FIRE_PROGRESS.len() as u8 - 1);

        let mut texture = [FIRE_PROGRESS[0]; LEN];
        texture[LEN - WIDTH..].fill(*FIRE_PROGRESS.last().unwrap());

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let height = Rc::new(Cell::new(0.0));
        let height_elem: HtmlInputElement = document
            .get_element_by_id("height_param")
            .unwrap()
            .dyn_into()
            .unwrap();
        let height_event = height.clone();
        let height_elem_event = height_elem.clone();
        let height_closure =
            Closure::<dyn Fn()>::new(move || height_event.set(height_elem_event.value_as_number()));
        height_elem.set_oninput(Some(height_closure.as_ref().unchecked_ref()));

        let spread = Rc::new(Cell::new(0.0));
        let spread_elem: HtmlInputElement = document
            .get_element_by_id("spread_param")
            .unwrap()
            .dyn_into()
            .unwrap();
        let spread_event = spread.clone();
        let spread_elem_event = spread_elem.clone();
        let spread_closure =
            Closure::<dyn Fn()>::new(move || spread_event.set(spread_elem_event.value_as_number()));
        spread_elem.set_oninput(Some(spread_closure.as_ref().unchecked_ref()));

        Self {
            data,
            texture,
            rng: SmallRng::from_os_rng(),

            height,
            spread,
        }
    }

    pub fn tick(&mut self, color: ColorMode) {
        for row in 1..HEIGHT {
            for col in 0..WIDTH {
                let spread_rand = if self.rng.random_bool(self.spread.get()) {
                    *[0_usize, 2, 3].choose(&mut self.rng).unwrap_or(&1)
                } else {
                    1
                };

                let decrease_rand = if self.rng.random_bool(self.height.get()) {
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
        for i in 0..LEN {
            let mut val = FIRE_PROGRESS[self.data[i] as usize];

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
