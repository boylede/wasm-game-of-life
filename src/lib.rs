#![no_std]

mod utils;

use wasm_bindgen::prelude::*;

macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    }
}

const SIZE: usize = 64;

static mut static_universe: Universe = Universe {
    width: SIZE as u32,
    height: SIZE as u32,
    tick_rate: 1,
    cells: [Cell::Dead; SIZE*SIZE],
    double: [Cell::Dead; SIZE*SIZE],
    state: BufferState::First,
};

// #[wasm_bindgen]
// pub struct StaticUniverse;

// #[wasm_bindgen]
// impl StaticUniverse {
    #[wasm_bindgen]
    pub fn static_tick() {
        // log!("tick");
        unsafe {
            static_universe.tick();
        }
    }
    #[wasm_bindgen]
    pub fn static_width() -> u32 {
        // log!("get width");
        SIZE as u32
    }
    #[wasm_bindgen]
    pub fn static_height() -> u32 {
        // log!("get height");
        SIZE as u32
    }
    #[wasm_bindgen]
    pub fn toggle_cell(row: u32, column: u32) {
        // log!("toggle cell");
        unsafe {
            static_universe.toggle_cell(row, column);
        }
    }
    #[wasm_bindgen]
    pub fn cells_ptr() -> *const Cell {
        // log!("cell ptr");
        unsafe {
            static_universe.cells_ptr()
        }
    }
    #[wasm_bindgen]
    pub fn reset() {
        // log!("reset");
        unsafe {
            static_universe.reset();
        }
    }
// }


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
    cells: [Cell; SIZE*SIZE],
    double: [Cell; SIZE*SIZE],
    state: BufferState,
}

impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        // log!("getting index");
        let idx = (row * self.width + col) as usize;
        // log!("got index: {}", idx);
        idx
    }
    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        // log!("start counting neighbors");
        let cells: &[Cell] = match self.state {
            BufferState::First => &self.cells,
            BufferState::Second => &self.double,
        };
        let north = if row == 0 { self.height - 1 } else { row - 1 };
        let south = if row == self.height - 1 { 0 } else { row + 1 };
        let west = if col == 0 { self.width - 1 } else { col - 1 };
        let east = if col == self.width - 1 { 0 } else { col + 1 };
        // log!("ready to add");
        let count = {
            let nw = self.get_index(north, west);
            // log!("1, {}", cells.len());
            cells[nw] as u8
        } + {
            let n = self.get_index(north, col);
            // log!("2");
            cells[n] as u8
        } + {
            let ne = self.get_index(north, east);
            // log!("3");
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
        // log!("done counting neighbors");
        return count;
    }
    pub fn reset_dead(&mut self) {
        (0..self.width * self.height).for_each(|i| self.cells[i as usize] = Cell::Dead);
        self.double = self.cells.clone();
    }
    pub fn reset_random(&mut self) {
        (0..self.width * self.height).for_each(|i| {
            if js_sys::Math::random() < 0.5 {
                self.cells[i as usize] =Cell::Alive
            } else {
                self.cells[i as usize] =Cell::Dead
            }
        });
        self.double = self.cells.clone();
    }
    pub fn reset_grid(&mut self) {
        (0..self.width * self.height).for_each(|i| {
            if i % 2 == 0 || i % 7 == 0 {
                self.cells[i as usize] = Cell::Alive
            } else {
                self.cells[i as usize] =Cell::Dead
            }
        });
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
            let cells: &mut [Cell] = match self.state {
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
        // let (last, next): (&Vec<Cell>, &mut Vec<Cell>) = match self.state {
        //     BufferState::First => (&self.cells, &mut self.double),
        //     BufferState::Second => (&self.double, &mut self.cells),
        // };

        // let mut next = self.cells.clone();
        // log!("tick inner");
        for _ in 0..self.tick_rate {
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    // let cell = last[idx];
                    let cell = match self.state {
                        BufferState::First => self.cells[idx],
                        BufferState::Second => self.double[idx],
                    };
                    let live_neighbors = self.live_neighbor_count(row, col);
                    // log!(
                    //     "cell[{}],{}] is {:?} and has {} live neighbors",
                    //     row,
                    //     col,
                    //     cell,
                    //     live_neighbors,
                    // );
                    let next_cell = match (cell, live_neighbors) {
                        (Cell::Alive, x) if x < 2 => Cell::Dead,
                        (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                        (Cell::Alive, x) if x > 3 => Cell::Dead,
                        (Cell::Dead, 3) => Cell::Alive,
                        (otherwise, _) => otherwise,
                    };
                    // log!("      it becomes {:?}", next_cell);
                    match self.state {
                        BufferState::First => self.double[idx] = next_cell,
                        BufferState::Second => self.cells[idx] = next_cell,
                    }
                    // log!("cell done");
                }
                // log!("row done");
            } 
        }
        self.state = match self.state {
            BufferState::First => BufferState::Second,
            BufferState::Second => BufferState::First,
        };
        // log!("tick done");
    }
    // pub fn new() -> Self {
    //     // log!("test");
    //     utils::set_panic_hook();
    //     // panic!("test");
    //     let width = 64;
    //     let height = 64;
    //     let mut new = Universe {
    //         width,
    //         height,
    //         tick_rate: 1,
    //         cells: vec![],
    //         double: vec![],
    //         state: BufferState::First,
    //     };
    //     new.reset_random();
    //     new
    // }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn cells_ptr(&self) -> *const Cell {
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

// use std::fmt;

// impl fmt::Display for Universe {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         for line in self.cells.as_slice().chunks(self.width as usize) {
//             for &cell in line {
//                 let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
//                 write!(f, "{}", symbol)?;
//             }
//             write!(f, "\n")?;
//         }
//         Ok(())
//     }
// }
