use std::collections::{HashMap, HashSet};

#[derive(PartialEq)]
pub enum Mode {
    Drawing,
    Simulating,
    None,
}

pub struct App {
    pub size: u32,
    pub board: HashMap<(u16, u16), bool>,
    pub generation: u32,
    pub live_cells: u64,
    pub mode: Mode,
}

impl App {
    pub fn new() -> Self {
        let size = 15;
        let board = HashMap::new();
        App { size, board, generation: 0, live_cells: 0, mode: Mode::Drawing }
    }   

    pub fn get_points(&self) -> Vec<(u16, u16)> {
        self.board
            .iter()
            .filter(|&(_, &v)| v) // Only keep entries where the value is true
            .map(|(&k, _)| k)     // Collect just the keys
            .collect()
    }

    pub fn draw(&mut self, x: u16, y: u16, val: bool) {
        if self.mode == Mode::Drawing {
            self.board.insert((x, y), val);
        }
    }
    
    pub fn end_simulation(&mut self) {
        self.mode = Mode::None;
    }

    pub fn clear(&mut self) {
        self.board.clear();
        self.generation = 0;
        self.live_cells = 0;
        self.end_simulation();
    }

    pub fn toggle_drawing(&mut self) {
        if self.mode == Mode::Drawing {
            self.mode = Mode::None;
        } else {
            self.mode = Mode::Drawing;
        }
    }

    pub fn simulate(&mut self) {
        // simulate one generation - called each tick
        if self.mode != Mode::Simulating {
            return;
        }

        let mut new_board: HashMap<(u16, u16), bool> = HashMap::new();
        let mut neighbor_counts: HashMap<(u16, u16), u16> = HashMap::new();
        let mut cells_to_check: HashSet<(u16, u16)> = HashSet::new();

        // vector of live cells
        let live_cells: Vec<(u16, u16)> = self.board
            .iter()
            .filter(|&(_, &v)| v)   // keep only entries where value is true (live cells)
            .map(|(&k, _)| k)     // extract the key (coordinates)
            .collect();

        for &(x, y) in &live_cells {
            // add live and neighboring cells to cells_to_check
            cells_to_check.insert((x, y));
            App::update_neighbor_counts(self, x, y, &mut neighbor_counts, &mut cells_to_check);
        }
    
        for cell in &cells_to_check {
            // get loive neighbors count
            let live_neighbors = if let Some(&count) = neighbor_counts.get(cell) {
                count // already calculated - use it
            } else {
                // not calculated - calculate it
                let count = self.calculate_live_neighbors(*cell);
                neighbor_counts.insert(*cell, count); // Store the count
                count
            };
    
            let is_live = self.board.get(cell).copied().unwrap_or(false);
    
            // apply rules
            if is_live {
                // cell survives if it has 2 or 3 live neighbors
                new_board.insert(*cell, live_neighbors == 2 || live_neighbors == 3);
            } else {
                // dead cell comes to life if it has exactly 3 live neighbors
                new_board.insert(*cell, live_neighbors == 3);
            }
        }

        // update board
        self.board = new_board;
        self.generation += 1;
        self.live_cells = self.board.values().filter(|&&v| v).count() as u64;
    }

    fn update_neighbor_counts(
        &self,
        x: u16,
        y: u16,
        neighbor_counts: &mut HashMap<(u16, u16), u16>,
        cells_to_check: &mut HashSet<(u16, u16)>,
    ) {
        let deltas = [-1, 0, 1];
    
        for dx in deltas.iter() {
            for dy in deltas.iter() {
                if *dx == 0 && *dy == 0 {
                    continue; // skip cell itself
                }
    
                let nx = (x as i32 + dx) as u16;
                let ny = (y as i32 + dy) as u16;
    
                // insert neighboring cell
                cells_to_check.insert((nx, ny));
    
                // if live increment neighbor count
                if let Some(&is_live) = self.board.get(&(nx, ny)) {
                    if is_live {
                        *neighbor_counts.entry((nx, ny)).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    
    fn calculate_live_neighbors(&self, cell: (u16, u16)) -> u16 {
        let (x, y) = cell;
        let deltas = [-1, 0, 1];
        let mut live_neighbors = 0;
    
        for dx in deltas.iter() {
            for dy in deltas.iter() {
                if *dx == 0 && *dy == 0 {
                    continue; // skip cell itself
                }
    
                let nx = (x as i32 + dx) as u16;
                let ny = (y as i32 + dy) as u16;
    
                // if live increment neighbor count
                if let Some(&is_live) = self.board.get(&(nx, ny)) {
                    if is_live {
                        live_neighbors += 1;
                    }
                }
            }
        }
    
        live_neighbors
    }
}
