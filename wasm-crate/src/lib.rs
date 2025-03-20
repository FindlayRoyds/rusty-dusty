use hashbrown::HashSet; // explained in Cargo.toml
use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

mod cells;
use cells::{Cell, Kind, Vector};

const USE_HASHBROWN: bool = false;

mod utils;
use utils::remove_random;

#[wasm_bindgen]
pub struct Game {
    grid_width: i32,
    grid_height: i32,
    grid: Vec<Vec<Cell>>,
    cells_to_update: HashSet<Vector>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(grid_width: i32, grid_height: i32) -> Game {
        Game {
            grid_width,
            grid_height,
            grid: vec![vec![Cell::new(Kind::Air); grid_width as usize]; grid_height as usize],
            cells_to_update: HashSet::new(),
        }
    }

    fn all_positions(&self) -> Vec<Vector> {
        // Could probably be simplified but I'm not good enough at rust yet
        let mut positions = Vec::new();
        for y in 0..self.grid_height {
            for x in 0..self.grid_width {
                positions.push(Vector::new(x, y));
            }
        }
        return positions;
    }

    fn is_in_bounds(&self, position: &Vector) -> bool {
        // I could do (0..self.width).contains..., but I'm not sure it'll get optimised
        position.x >= 0
            && position.y >= 0
            && position.x < self.grid_width
            && position.y < self.grid_height
    }

    fn get_cell(&self, position: &Vector) -> Cell {
        // Should only be called during update step
        if !self.is_in_bounds(position) {
            return Cell::new(Kind::Wall);
        }
        let x = (position.x + self.grid_width) % self.grid_width;
        let y = (position.y + self.grid_height) % self.grid_height;
        self.grid[y as usize][x as usize].clone()
    }

    fn set_cell(&mut self, position: &Vector, value: Cell) {
        if !self.is_in_bounds(position) {
            return;
        }
        let x = (position.x + self.grid_width) % self.grid_width;
        let y = (position.y + self.grid_height) % self.grid_height;
        self.grid[y as usize][x as usize] = value;
    }

    fn swap_cells(&mut self, position1: &Vector, position2: &Vector) {
        // Should only be called during update step
        if !self.is_in_bounds(position1) || !self.is_in_bounds(position2) {
            return;
        }
        let cell1 = self.get_cell(position1);
        let cell2 = self.get_cell(position2);
        self.set_cell(position1, cell2);
        self.set_cell(position2, cell1);

        if !USE_HASHBROWN {
            return;
        }
        if self.cells_to_update.contains(position1) {
            self.cells_to_update.remove(position1);
            self.cells_to_update.insert(*position2);
        } else if self.cells_to_update.contains(position2) {
            self.cells_to_update.remove(position2);
            self.cells_to_update.insert(*position1);
        }
    }

    fn is_type(&self, position: &Vector, kind: Kind) -> bool {
        self.get_cell(position).kind == kind
    }

    #[wasm_bindgen]
    pub fn initialise(&mut self) {
        for position in self.all_positions() {
            self.set_cell(&position, Kind::Air.new(None));
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {
        let mut positions = self.all_positions();
        if USE_HASHBROWN {
            self.cells_to_update.clear(); // shouldn't be needed but is a nice safe-guard
            self.cells_to_update.extend(positions.clone());
            for _ in 0..self.cells_to_update.len() {
                if let Some(position) = remove_random(&mut self.cells_to_update) {
                    let cell = self.get_cell(&position);
                    cell.kind.update(&cell, &position, self);
                }
            }
        } else {
            fastrand::shuffle(&mut positions);
            for position in positions {
                let cell = self.get_cell(&position);
                cell.kind.update(&cell, &position, self);
            }
        }
    }

    #[wasm_bindgen]
    pub fn click_at(&mut self, x: i32, y: i32, radius: i32, time: i32) {
        for y_offset in -radius..=radius {
            for x_offset in -radius..=radius {
                let offset = Vector::new(x_offset, y_offset);
                if offset.length_squared() > radius * radius {
                    continue;
                }
                let position = Vector::new(x_offset + x, y_offset + (self.grid_height - y - 1));
                if !self.is_type(&position, Kind::Air) {
                    continue;
                }

                self.set_cell(&position, Kind::Sand.new(Some(time)));
            }
        }
    }

    #[wasm_bindgen]
    pub fn draw(&self, canvas: HtmlCanvasElement, pixel_size: u32) -> Result<(), JsValue> {
        let context = canvas
            .get_context("2d")?
            .ok_or("Failed to get canvas context :(")?
            .dyn_into::<CanvasRenderingContext2d>()?;

        let width = self.grid_width as u32;
        let height = self.grid_height as u32;

        let mut data = vec![255; (width * height * pixel_size * pixel_size * 4) as usize];

        for position in self.all_positions() {
            let cell = self.get_cell(&position);
            let x_start = position.x as u32 * pixel_size;
            let y_start = (self.grid_height - position.y - 1) as u32 * pixel_size;

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

        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut data),
            width * pixel_size,
            height * pixel_size,
        )?;
        context.put_image_data(&image_data, 0.0, 0.0)?;

        Ok(())
    }
}
