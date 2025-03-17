use std::collections::HashMap;

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

        
    }
}