use crate::Game;
use fastrand;
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
pub type Vector = Vector2D<i32>;

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

    // pub fn new_with_color(kind: Kind, color: Color) -> Self {
    //     Cell { kind, color }
    // }
}

// #[wasm_bindgen]
#[derive(PartialEq, Clone)]
pub enum Kind {
    Air,
    Wall,
    Sand,
}

impl Kind {
    pub fn new(&self, time: i32) -> Cell {
        match self {
            Kind::Sand => create_sand(time),
            _ => Cell::new(self.clone()),
        }
    }

    pub fn update(&self, cell: &Cell, position: &Vector, grid: &mut Game) {
        match self {
            Kind::Sand => update_sand(cell, position, grid),
            _ => {}
        }
    }
}

fn update_sand(_: &Cell, position: &Vector, grid: &mut Game) {
    if fastrand::u8(0..15) > 0 {
        let below_position = position + &Vector::new(0, -1);
        if grid.is_type(&below_position, Kind::Air) {
            grid.swap_cells(position, &below_position);
            return;
        }
    }

    let direction = if fastrand::bool() { -1 } else { 1 };
    let side_position = position + &Vector::new(direction, 0);
    let below_position = position + &Vector::new(direction, -1);
    if grid.is_type(&below_position, Kind::Air) && grid.is_type(&side_position, Kind::Air) {
        grid.swap_cells(position, &side_position);
        return;
    }
}

fn create_sand(time: i32) -> Cell {
    let base_color = 45.0;
    let color_range = 7.0;
    let lightness =
        ((time as f64 / 15.0).rem_euclid(color_range * 2.0) - color_range).abs() + base_color;
    let color = Color::from_hsl(42.0, 50.0, lightness);
    let mut cell = Cell::new(Kind::Sand);
    cell.color = color;
    return cell;
}
