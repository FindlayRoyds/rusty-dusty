use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

mod cells;
use cells::*;

#[wasm_bindgen]
pub struct Grid {
    width: i32,
    height: i32,
    grid: Vec<Vec<Cell>>,
    grid_update: Vec<Vec<Cell>>,
    // rng: OsRng,
}

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(constructor)]
    pub fn new(width: i32, height: i32) -> Grid {
        Grid {
            width,
            height,
            grid: vec![vec![Cell::new(Kind::Air); width as usize]; height as usize],
            grid_update: vec![vec![Cell::new(Kind::Air); width as usize]; height as usize],
            // rng: OsRng,
        }
    }

    fn all_positions(&self) -> Vec<Vector> {
        // Could probably be simplified but I'm not good enough at rust yet
        let mut positions = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                positions.push(Vector::new(x, y));
            }
        }
        return positions;
    }

    fn is_in_bounds(&self, position: &Vector) -> bool {
        // I could do (0..self.width).contains..., but I'm not sure it'll get optimised
        position.x >= 0 && position.y >= 0 && position.x < self.width && position.y < self.height
    }

    fn get_cell(&self, position: &Vector) -> Cell {
        if !self.is_in_bounds(position) {
            return Cell::new(Kind::Wall);
        }
        let x = (position.x + self.width) % self.width;
        let y = (position.y + self.height) % self.height;
        self.grid[y as usize][x as usize].clone()
    }

    fn set_cell(&mut self, position: &Vector, value: Cell) {
        if !self.is_in_bounds(position) {
            return;
        }
        let x = (position.x + self.width) % self.width;
        let y = (position.y + self.height) % self.height;
        self.grid_update[y as usize][x as usize] = value;
    }

    fn swap_cells(&mut self, position1: &Vector, position2: &Vector) {
        if !self.is_in_bounds(position1) || !self.is_in_bounds(position2) {
            return;
        }
        let cell1 = self.get_cell(position1);
        let cell2 = self.get_cell(position2);
        self.set_cell(position1, cell2);
        self.set_cell(position2, cell1);
    }

    fn is_type(&self, position: &Vector, kind: Kind) -> bool {
        self.get_cell(position).kind == kind
    }

    fn push_update(&mut self) {
        self.grid = self.grid_update.clone();
    }

    #[wasm_bindgen]
    pub fn initialise(&mut self) {
        for position in self.all_positions() {
            // let value = self.rng.gen_range(0..=1) == 1;
            self.set_cell(&position, Cell::new(Kind::Air));
        }
        self.push_update();
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {
        for position in self.all_positions() {
            let cell = self.get_cell(&position);
            cell.kind.update(&cell, &position, self);
        }
        self.push_update();
    }

    #[wasm_bindgen]
    pub fn click_at(&mut self, x: i32, y: i32, radius: i32, time: i32) {
        for y_offset in -radius..=radius {
            for x_offset in -radius..=radius {
                let offset = Vector::new(x_offset, y_offset);
                if offset.length_squared() > radius * radius {
                    continue;
                }
                let position = Vector::new(x_offset + x, y_offset + (self.height - y - 1));
                if !self.is_type(&position, Kind::Air) {
                    continue;
                }

                self.set_cell(&position, Kind::Sand.new(time));
            }
        }

        self.push_update();
    }

    #[wasm_bindgen]
    pub fn draw(&self, canvas: HtmlCanvasElement, pixel_size: u32) -> Result<(), JsValue> {
        let context = canvas
            .get_context("2d")?
            .ok_or("Failed to get canvas context :(")?
            .dyn_into::<CanvasRenderingContext2d>()?;

        let width = self.width as u32;
        let height = self.height as u32;

        let mut data = vec![255; (width * height * pixel_size * pixel_size * 4) as usize];

        for position in self.all_positions() {
            let cell = self.get_cell(&position);
            if cell.kind == Kind::Sand {
                let x_start = position.x as u32 * pixel_size;
                let y_start = (self.height - position.y - 1) as u32 * pixel_size;

                for y in 0..pixel_size {
                    for x in 0..pixel_size {
                        let index = ((y_start + y) * width * pixel_size + (x_start + x)) * 4;
                        data[index as usize] = cell.color.r; // R
                        data[index as usize + 1] = cell.color.g; // G
                        data[index as usize + 2] = cell.color.b; // B
                        data[index as usize + 3] = 255; // A
                    }
                }
            }
        }

        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut data),
            width * pixel_size,
            height * pixel_size,
        )?;
        context.put_image_data(&image_data, 0.0, 0.0)?;

        Ok(())
    }
}
