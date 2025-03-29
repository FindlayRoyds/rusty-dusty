use crate::Game;
use fastrand;
use vector2d::Vector2D;
use wasm_bindgen::prelude::*;

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
pub type Vector = Vector2D<i32>;

#[derive(Clone)]
pub struct Cell {
    pub kind: Kind,
    pub color: Color,
    pub data: u8,
}

impl Cell {
    pub fn new(kind: Kind) -> Self {
        Cell {
            kind,
            color: Color::new(0, 0, 0),
            data: 0,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Air,
    Wall,
    Sand,
    Water,
}

impl Kind {
    pub fn new(&self, time: Option<i32>) -> Cell {
        match self {
            Kind::Sand => create_sand(time.unwrap()),
            Kind::Air => create_air(),
            Kind::Water => create_water(),
            _ => Cell::new(self.clone()),
        }
    }

    pub fn update(&self, cell: &Cell, position: &Vector, grid: &mut Game) {
        match self {
            Kind::Sand => update_sand(cell, position, grid),
            Kind::Water => update_water(cell, position, grid),
            _ => {}
        }
    }
}

// ---------- AIR

fn create_air() -> Cell {
    let mut cell = Cell::new(Kind::Air);
    cell.color = Color::new(229, 243, 253);
    return cell;
}

// ---------- SAND

fn create_sand(time: i32) -> Cell {
    let base_color = 70.0;
    let color_range = 7.0;
    let lightness =
        ((time as f64 / 15.0).rem_euclid(color_range * 2.0) - color_range).abs() + base_color;
    let color = Color::from_hsl(42.0, 50.0, lightness);
    let mut cell = Cell::new(Kind::Sand);
    cell.color = color;
    return cell;
}

fn update_sand(_: &Cell, position: &Vector, grid: &mut Game) {
    if fastrand::u8(0..15) > 0 {
        let below_position = position + &Vector::new(0, -1);
        let should_fall = || -> bool {
            if grid.is_type(&below_position, Kind::Air) {
                return true;
            }
            if grid.is_type(&below_position, Kind::Water) {
                return fastrand::bool();
            }
            false
        };
        if should_fall() {
            grid.swap_cells(position, &below_position);
            return;
        }
    }

    let direction = if fastrand::bool() { -1 } else { 1 };
    let side_position = position + &Vector::new(direction, 0);
    let below_position = position + &Vector::new(direction, -1);
    let should_move = || -> bool {
        if grid.is_type(&below_position, Kind::Air) && grid.is_type(&side_position, Kind::Air) {
            return true;
        }
        if grid.is_type(&below_position, Kind::Water)
            || grid.is_type(&below_position, Kind::Air) && grid.is_type(&side_position, Kind::Water)
            || grid.is_type(&below_position, Kind::Water)
        {
            return fastrand::bool();
        }
        false
    };
    if should_move() {
        grid.swap_cells(position, &below_position);
        return;
    }
}

// ---------- WATER

fn create_water() -> Cell {
    let mut cell = Cell::new(Kind::Water);
    cell.color = Color::new(0, 100, fastrand::u8(220..=255));
    cell.data = fastrand::u8(0..2);
    return cell;
}

fn update_water(cell: &Cell, position: &Vector, grid: &mut Game) {
    let mut new_cell = cell.clone();
    new_cell.color.b = (new_cell.color.b - 220 + 1) % (255 - 220) + 220;

    let below_position = position + &Vector::new(0, -1);
    if grid.is_type(&below_position, Kind::Air) {
        grid.swap_cells(position, &below_position);
        return;
    }

    let direction = if cell.data == 0 { -1 } else { 1 };
    let side_position = position + &Vector::new(direction, 0);
    if grid.is_type(&side_position, Kind::Air) {
        grid.swap_cells(position, &side_position);
        return;
    } else {
        new_cell.data ^= 1;
        grid.set_cell(position, new_cell);
    }
}
