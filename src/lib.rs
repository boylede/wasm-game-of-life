mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        }
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let north = if row == 0 {self.height - 1} else {row - 1};
        let south = if row == self.height - 1 { 0 } else { row + 1 };
        let west = if column == 0 { self.width - 1 } else { column - 1 };
        let east = if column == self.width - 1 { 0 } else { column + 1 };

        return {
            let nw = self.get_index(north, west);
            self.cells[nw] as u8
        } + {
            let n = self.get_index(north, column);
            self.cells[n] as u8
        } + {
            let ne = self.get_index(north, east);
            self.cells[ne] as u8
        } + {
            let w = self.get_index(row, west);
            self.cells[w] as u8
        } + {
            let e = self.get_index(row, east);
            self.cells[e] as u8
        } + {
            let sw = self.get_index(south, west);
            self.cells[sw] as u8
        } + {
            let s = self.get_index(south, column);
            self.cells[s] as u8
        } + {
            let se = self.get_index(south, east);
            self.cells[se] as u8
        };
    }
    pub fn reset_dead(&mut self) {
        self.cells = (0..self.width * self.height)
            .map(|_| {
                Cell::Dead
            })
            .collect();
    }
    pub fn reset_random(&mut self) {
        self.cells = (0..self.width * self.height)
            .map(|_| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
    }
    pub fn reset_grid(&mut self) {
        self.cells = (0..self.width * self.height)
            .map(|i| {
                if i % 2 == 0 || i % 7 ==0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
    }
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }
    pub fn set_cells_alive(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);
                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }
    pub fn new() -> Self {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;
        let mut new = Universe {
            width,
            height,
            cells: vec![],
        };
        new.reset_random();
        new
    }
    pub fn render(&self) -> String {
        self.to_string()
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.reset_dead();
    }
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.reset_dead();
    }
    pub fn reset(&mut self) {
        self.reset_grid();
    }
    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
