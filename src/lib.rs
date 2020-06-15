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

enum BufferState {
    First,
    Second,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    tick_rate: u32,
    cells: Vec<Cell>,
    double: Vec<Cell>,
    state: BufferState,
}

impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }
    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let cells: &Vec<Cell> = match self.state {
            BufferState::First => &self.cells,
            BufferState::Second => &self.double,
        };
        let north = if row == 0 { self.height - 1 } else { row - 1 };
        let south = if row == self.height - 1 { 0 } else { row + 1 };
        let west = if col == 0 { self.width - 1 } else { col - 1 };
        let east = if col == self.width - 1 { 0 } else { col + 1 };

        return {
            let nw = self.get_index(north, west);
            cells[nw] as u8
        } + {
            let n = self.get_index(north, col);
            cells[n] as u8
        } + {
            let ne = self.get_index(north, east);
            cells[ne] as u8
        } + {
            let w = self.get_index(row, west);
            cells[w] as u8
        } + {
            let e = self.get_index(row, east);
            cells[e] as u8
        } + {
            let sw = self.get_index(south, west);
            cells[sw] as u8
        } + {
            let s = self.get_index(south, col);
            cells[s] as u8
        } + {
            let se = self.get_index(south, east);
            cells[se] as u8
        };
    }
    pub fn reset_dead(&mut self) {
        self.cells = (0..self.width * self.height).map(|_| Cell::Dead).collect();
        self.double = self.cells.clone();
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
        self.double = self.cells.clone();
    }
    pub fn reset_grid(&mut self) {
        self.cells = (0..self.width * self.height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        self.double = self.cells.clone();
    }
    pub fn get_cells(&self) -> &[Cell] {
        match self.state {
            BufferState::First => &self.cells,
            BufferState::Second => &self.double,
        }
    }
    pub fn set_cells_alive(&mut self, alive: &[(u32, u32)]) {
        for (row, col) in alive.iter().cloned() {
            let idx = self.get_index(row, col);
            let cells: &mut Vec<Cell> = match self.state {
                BufferState::First => &mut self.cells,
                BufferState::Second => &mut self.double,
            };
            cells[idx] = Cell::Alive;
        }
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        for _ in 0..self.tick_rate {
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = match self.state {
                        BufferState::First => self.cells[idx],
                        BufferState::Second => self.double[idx],
                    };
                    let live_neighbors = self.live_neighbor_count(row, col);
                    let next_cell = match (cell, live_neighbors) {
                        (Cell::Alive, x) if x < 2 => Cell::Dead,
                        (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                        (Cell::Alive, x) if x > 3 => Cell::Dead,
                        (Cell::Dead, 3) => Cell::Alive,
                        (otherwise, _) => otherwise,
                    };
                    match self.state {
                        BufferState::First => self.double[idx] = next_cell,
                        BufferState::Second => self.cells[idx] = next_cell,
                    }
                }
            }
            self.state = match self.state {
                BufferState::First => BufferState::Second,
                BufferState::Second => BufferState::First,
            }
        }
    }
    pub fn new() -> Self {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;
        let mut new = Universe {
            width,
            height,
            tick_rate: 1,
            cells: vec![],
            double: vec![],
            state: BufferState::First,
        };
        new.reset_random();
        new
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn cells(&self) -> *const Cell {
        match self.state {
            BufferState::First => self.cells.as_ptr(),
            BufferState::Second => self.double.as_ptr(),
        }
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
        match self.state {
            BufferState::First => self.cells[idx].toggle(),
            BufferState::Second => self.double[idx].toggle(),
        }
    }
}


