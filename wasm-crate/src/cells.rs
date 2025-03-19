use crate::Grid;
// use nalgebra::{Vector2, Vector3};
use vector2d::Vector2D;

// pub type Color = Vector3<u8>;
#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_hsl(h: f64, s: f64, l: f64) -> Self {
        // Found on the internet
        let h = h as f32 / 360.0;
        let s = s as f32 / 100.0;
        let l = l as f32 / 100.0;

        let a = s * l.min(1.0 - l);
        let f = |n: f32| {
            let k = (n + h * 12.0) % 12.0;
            l - a * (k - 3.0).min(9.0 - k).max(-1.0)
        };

        let r = (f(0.0) * 255.0).round() as u8;
        let g = (f(8.0) * 255.0).round() as u8;
        let b = (f(4.0) * 255.0).round() as u8;

        Self { r, g, b }
    }
}

// pub type Position = Vector2<i32>;
pub type Position = Vector2D<i32>;

#[derive(Clone)]
pub struct Cell {
    pub kind: Kind,
    pub color: Color,
}

impl Cell {
    pub fn new(kind: Kind) -> Self {
        Cell {
            kind,
            color: Color::new(0, 0, 0),
        }
    }

    pub fn new_with_color(kind: Kind, color: Color) -> Self {
        Cell { kind, color }
    }
}

// #[wasm_bindgen]
#[derive(PartialEq, Clone)]
pub enum Kind {
    Air,
    Wall,
    Sand,
}

impl Kind {
    pub fn update(&self, cell: &Cell, position: &Position, grid: &mut Grid) {
        match self {
            Kind::Air => {}
            Kind::Wall => {}
            Kind::Sand => update_sand(cell, position, grid),
        }
    }
}

fn update_sand(cell: &Cell, position: &Position, grid: &mut Grid) {
    let below_position = position - &Position::new(0, 1);
    if grid.get_cell(&below_position).kind == Kind::Air {
        grid.set_cell(
            &below_position,
            Cell::new_with_color(Kind::Sand, cell.color.clone()),
        );
        grid.set_cell(
            &position,
            Cell::new_with_color(Kind::Air, cell.color.clone()),
        );
        return;
    }

    for direction in &[-1, 1] {
        let random_direction = direction * if position.y % 2 == 0 { -1 } else { 1 };
        let below_position = position + &Position::new(direction * random_direction, -1);
        if grid.get_cell(&below_position).kind != Kind::Air {
            continue;
        }

        grid.set_cell(
            &below_position,
            Cell::new_with_color(Kind::Sand, cell.color.clone()),
        );
        grid.set_cell(
            &position,
            Cell::new_with_color(Kind::Air, cell.color.clone()),
        );
        return;
    }
}
