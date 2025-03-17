use nalgebra::Vector2;
use rand::{Rng, rngs::OsRng};
use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

type Position = Vector2<i32>;

#[wasm_bindgen]
pub struct Grid {
    width: i32,
    height: i32,
    grid: Vec<Vec<bool>>,
    grid_update: Vec<Vec<bool>>,
    // rng: OsRng,
}

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(constructor)]
    pub fn new(width: i32, height: i32) -> Grid {
        Grid {
            width,
            height,
            grid: vec![vec![false; width as usize]; height as usize],
            grid_update: vec![vec![false; width as usize]; height as usize],
            // rng: OsRng,
        }
    }

    fn all_positions(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                positions.push(Position::new(x, y));
            }
        }
        return positions;
    }

    fn get_cell(&self, position: &Position) -> Option<bool> {
        // if position.x < 0 || position.y < 0 || position.x >= self.width || position.y >= self.height
        // {
        //     return None;
        // }
        let x = (position.x + self.width) % self.width;
        let y = (position.y + self.height) % self.height;
        Some(self.grid[y as usize][x as usize])
    }

    fn set_cell(&mut self, position: &Position, value: bool) {
        // if position.x < 0 || position.y < 0 || position.x >= self.width || position.y >= self.height
        // {
        //     return;
        // }
        let x = (position.x + self.width) % self.width;
        let y = (position.y + self.height) % self.height;
        self.grid_update[y as usize][x as usize] = value;
    }

    fn push_update(&mut self) {
        self.grid = self.grid_update.clone();
    }

    #[wasm_bindgen]
    pub fn click_at(&mut self, x: i32, y: i32) {
        for y_offset in -1..=1 {
            for x_offset in -1..=1 {
                self.set_cell(&Position::new(x_offset + x, y_offset + y), true);
            }
        }
        self.push_update();
    }

    #[wasm_bindgen]
    pub fn initialise(&mut self) {
        for position in self.all_positions() {
            // let value = self.rng.gen_range(0..=1) == 1;
            self.set_cell(&position, false);
        }
        self.push_update();
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {
        for position in self.all_positions() {
            let mut total = 0;
            let cell_value = self.get_cell(&position).unwrap_or(false);

            for y in -1..=1 {
                for x in -1..=1 {
                    if y == 0 && x == 0 {
                        continue;
                    }
                    let value = self
                        .get_cell(&Position::new(position.x + x, position.y + y))
                        .unwrap_or(false);
                    total += if value { 1 } else { 0 }
                }
            }
            // total += self.rng.gen_range(-4..=5);
            if cell_value {
                self.set_cell(&position, total == 2 || total == 3);
            } else {
                self.set_cell(&position, total == 3);
            }
        }
        self.push_update();
    }

    // #[wasm_bindgen]
    // pub fn draw(&self, canvas: HtmlCanvasElement, pixel_size: f64) -> Result<(), JsValue> {
    //     let context = canvas
    //         .get_context("2d")?
    //         .ok_or("Failed to get canvas context")?
    //         .dyn_into::<CanvasRenderingContext2d>()?;

    //     context.clear_rect(
    //         0.0,
    //         0.0,
    //         pixel_size * self.width as f64,
    //         pixel_size * self.height as f64,
    //     );

    //     for position in self.all_positions() {
    //         context.set_fill_style_str(if self.get_cell(&position).unwrap_or(false) {
    //             "black"
    //         } else {
    //             "white"
    //         });
    //         context.fill_rect(
    //             position.x as f64 * pixel_size,
    //             position.y as f64 * pixel_size,
    //             pixel_size,
    //             pixel_size,
    //         );
    //     }

    //     Ok(())
    // }

    #[wasm_bindgen]
    pub fn draw(&self, canvas: HtmlCanvasElement) -> Result<(), JsValue> {
        let context = canvas
            .get_context("2d")?
            .ok_or("Failed to get canvas context")?
            .dyn_into::<CanvasRenderingContext2d>()?;

        let width = self.width as u32;
        let height = self.height as u32;

        let mut buffer = vec![255; (width * height * 4) as usize];

        for position in self.all_positions() {
            let is_black = self.get_cell(&position).unwrap_or(false);
            let color = if is_black { 0 } else { 255 };

            let pixel_index = (((position.y * self.width) + position.x) * 4) as usize;
            buffer[pixel_index] = color;
            buffer[pixel_index + 1] = color;
            buffer[pixel_index + 2] = color;
            buffer[pixel_index + 3] = 255;
        }

        let image_data =
            ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut buffer), width, height)?;
        context.put_image_data(&image_data, 0.0, 0.0)
    }
}
