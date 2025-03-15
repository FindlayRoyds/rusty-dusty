use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct Grid {
    width: usize,
    height: usize,
    grid: Vec<bool>,
}

struct Position {
    x: usize,
    y: usize,
}

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            grid: vec![false; width * height],
        }
    }

    fn all_positions(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                positions.push(Position { x: x, y: y });
            }
        }
        return positions;
    }

    fn get_cell(&self, x: usize, y: usize) -> bool {
        let index = y * self.width + x;
        self.grid[index]
    }

    fn set_cell(&self, position: Position, value: bool) {
        let index = position.y + self.width * position.x;
        self.grid[index] = value;
    }

    #[wasm_bindgen]
    pub fn update(&self) {
        for position in self.all_positions() {
            self.set_cell(x, y, value);
        }
    }

    #[wasm_bindgen]
    pub fn draw_on_canvas(&self, canvas: HtmlCanvasElement) -> Result<(), JsValue> {
        let context = canvas
            .get_context("2d")?
            .ok_or("Failed to get canvas context")?
            .dyn_into::<CanvasRenderingContext2d>()?;

        for position in self.all_positions() {
            context.set_fill_style_str(if self.get_cell(position.x, position.y) {
                "blue"
            } else {
                "red"
            });
            context.fill_rect((position.x * 5) as f64, (position.y * 5) as f64, 5.0, 5.0);
        }

        Ok(())
    }
}
