mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}

#[wasm_bindgen]
pub fn greet_by_name(name: &str){
    alert(&format!("Howdy {}", name));
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>
}

//this implementation block is private by default
impl Universe {

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        //there's some interesting syntax below that I'm not sure what
        //it is called.
        for delta_row in [self.height -1, 0, 1].iter().cloned() {
            for delta_col in [self.width -1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbor_row,neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}


//these are the public implementation details.
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row,col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row,col);

                let next_cell = match (cell, live_neighbors) {
                    //Any live cell with < 2 live neighbors dies of loneliness
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    //any live cell with 2-3 neighbors is happy and survives
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    //any live cell with >3 live neighbors is crowded to death
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    //any dead cell with 3 live neighbors gets life born in it
                    (Cell::Dead, 3) => Cell::Alive,
                    //otherwise no change
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        let width=64;
        let height=64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        //okay, the above is kinda neat.  We're setting each cell's value to
        //it's own index.  Then we have a Vector with values which we can run
        //map on, which takes each value and runs a function.  This function is
        //that if the cell's idx is evenly divisible by 2 or 7 it will be alive
        //otherwise dead.  

        //At the moment, I'm not sure where map is implemented, nor collect.

        Universe {
            width,
            height,
            cells
        }
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

}

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
