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
    Fire,
    Wood,
    Steam,
}

impl Kind {
    pub fn new(&self, time: Option<i32>) -> Cell {
        match self {
            Kind::Sand => create_sand(time.unwrap()),
            Kind::Air => create_air(),
            Kind::Water => create_water(),
            Kind::Fire => create_fire(),
            Kind::Wood => create_wood(),
            Kind::Steam => create_steam(),
            _ => Cell::new(self.clone()),
        }
    }

    pub fn update(&self, cell: &Cell, position: &Vector, grid: &mut Game) {
        match self {
            Kind::Sand => update_sand(cell, position, grid),
            Kind::Water => update_water(cell, position, grid),
            Kind::Fire => update_fire(cell, position, grid),
            Kind::Wood => {}
            Kind::Steam => update_steam(cell, position, grid),
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
            if grid.is_type(&below_position, [Kind::Air]) {
                return true;
            }
            if grid.is_type(&below_position, [Kind::Water]) {
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
        if grid.is_type(&below_position, [Kind::Air]) && grid.is_type(&side_position, [Kind::Air]) {
            return true;
        }
        if grid.is_type(&below_position, [Kind::Water, Kind::Air])
            && grid.is_type(&side_position, [Kind::Water, Kind::Air])
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
    grid.set_cell(position, new_cell);

    let below_position = position + &Vector::new(0, -1);
    if grid.is_type(&below_position, [Kind::Air, Kind::Steam]) {
        let mut new_cell = cell.clone();
        new_cell.data ^= 1;
        grid.set_cell(position, new_cell);
        grid.swap_cells(position, &below_position);
        return;
    }

    let direction = if cell.data == 0 { -1 } else { 1 };
    let side_position = position + &Vector::new(direction, 0);
    if grid.is_type(&side_position, [Kind::Air, Kind::Water]) {
        grid.swap_cells(position, &side_position);
        let below_position = &side_position + &Vector::new(0, -1);
        if grid.is_type(&below_position, [Kind::Air]) {
            grid.swap_cells(&side_position, &below_position);
            return;
        }
        return;
    } else {
        let mut new_cell = cell.clone();
        new_cell.data ^= 1;
        grid.set_cell(position, new_cell);
    }
}

// ---------- WOOD

fn create_wood() -> Cell {
    let mut cell = Cell::new(Kind::Wood);

    let hue = 25.0 + fastrand::f64() * 5.0;
    let saturation = 45.0 + fastrand::f64() * 10.0;
    let lightness = 18.0 + fastrand::f64() * 7.0;
    cell.color = Color::from_hsl(hue, saturation, lightness);

    return cell;
}

// ---------- FIRE

fn create_fire() -> Cell {
    let mut cell = Cell::new(Kind::Fire);

    let hue = fastrand::f64() * 20.0 + 5.0;
    let saturation = 85.0 + fastrand::f64() * 15.0;
    let lightness = 45.0 + fastrand::f64() * 25.0;
    cell.color = Color::from_hsl(hue, saturation, lightness);

    cell.data = fastrand::u8(60..100);
    return cell;
}

fn update_fire(cell: &Cell, position: &Vector, grid: &mut Game) {
    let mut new_cell = cell.clone();
    let mut mut_position = &position.clone();

    let decay_rate = 1 + (fastrand::u8(0..5) < 3) as u8 * 4;
    if new_cell.data > decay_rate {
        new_cell.data -= decay_rate;
    } else {
        grid.set_cell(mut_position, create_air());
        return;
    }

    let lifespan_factor = new_cell.data as f64 / 100.0;
    let hue = 5.0 + lifespan_factor * 20.0;
    let saturation = 85.0 + lifespan_factor * 15.0;
    let lightness = 50.0 + lifespan_factor * 20.0;
    new_cell.color = Color::from_hsl(hue, saturation, lightness);
    grid.set_cell(mut_position, new_cell);

    let above_position = mut_position + &Vector::new(0, 1);
    let direction = if fastrand::bool() { -1 } else { 1 };
    let diagonal_position = mut_position + &Vector::new(direction, 0);

    if grid.is_type(&above_position, [Kind::Air]) {
        grid.swap_cells(mut_position, &above_position);
        mut_position = &above_position;
    } else {
        if grid.is_type(&diagonal_position, [Kind::Air]) {
            grid.swap_cells(mut_position, &diagonal_position);
            mut_position = &diagonal_position;
        }
    }

    burn_cell(grid, &(mut_position + &random_direction()), mut_position);
}

fn random_direction() -> Vector {
    let x = fastrand::i32(-1..=1);
    let y = fastrand::i32(-1..=1);
    Vector::new(x, y)
}

fn burn_cell(grid: &mut Game, position: &Vector, fire_position: &Vector) {
    let cell = grid.get_cell(position);
    match cell.kind {
        Kind::Water => {
            if fastrand::u8(0..5) == 0 {
                grid.set_cell(position, create_steam());
            }
            grid.set_cell(fire_position, create_air());
        }
        Kind::Wood => {
            if fastrand::u8(0..20) == 0 {
                grid.set_cell(position, create_fire());
            }
            for _ in 0..3 {
                let fire_position = position + &random_direction();
                if grid.is_type(&fire_position, [Kind::Air]) {
                    grid.set_cell(&fire_position, create_fire());
                }
            }
        }
        Kind::Steam => {
            grid.set_cell(position, create_steam());
        }
        _ => {}
    }
}

// ---------- STEAM

fn create_steam() -> Cell {
    let mut cell = Cell::new(Kind::Steam);

    let hue = 195.0 + fastrand::f64() * 10.0;
    let saturation = 10.0 + fastrand::f64() * 15.0;
    let lightness = 75.0 + fastrand::f64() * 15.0;
    cell.color = Color::from_hsl(hue, saturation, lightness);

    cell.data = fastrand::u8(60..100);
    return cell;
}

fn update_steam(cell: &Cell, position: &Vector, grid: &mut Game) {
    let mut new_cell = cell.clone();
    let position = &position.clone();

    let decay_rate = if fastrand::u8(0..20) == 0 { 1 } else { 0 };
    if new_cell.data > decay_rate {
        new_cell.data -= decay_rate;
    } else {
        grid.set_cell(position, create_water());
        return;
    }

    let lifespan_factor = new_cell.data as f64 / 100.0;
    let hue = 195.0 + lifespan_factor * 10.0;
    let saturation = 10.0 + (1.0 - lifespan_factor) * 15.0;
    let lightness = 75.0 + lifespan_factor * 15.0;
    new_cell.color = Color::from_hsl(hue, saturation, lightness);
    grid.set_cell(position, new_cell);

    let above_position = position + &Vector::new(0, 1);
    let below_position = position + &Vector::new(0, -1);

    if (grid.is_type(&above_position, [Kind::Steam, Kind::Wall]))
        && grid.is_type(&below_position, [Kind::Air])
    {
        grid.swap_cells(position, &below_position);
        return;
    }

    if fastrand::u8(0..10) < 8 && grid.is_type(&above_position, [Kind::Air, Kind::Fire]) {
        grid.swap_cells(position, &above_position);
    } else {
        let direction = if fastrand::bool() { -1 } else { 1 };
        let side_position = position + &Vector::new(direction, 0);

        if grid.is_type(&side_position, [Kind::Air]) {
            grid.swap_cells(position, &side_position);
        } else {
            let diagonal_up = position + &Vector::new(direction, 1);
            if grid.is_type(&diagonal_up, [Kind::Air]) {
                grid.swap_cells(position, &diagonal_up);
            }
        }
    }
}
