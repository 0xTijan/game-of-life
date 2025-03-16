use std::collections::HashMap;

pub struct App {
    pub size: u32,
    pub board: HashMap<(u16, u16), bool>,
    pub is_drawing: bool,
    pub generation: u32,
    pub live_cells: u64,
}

impl App {
    pub fn new() -> Self {
        let size = 15;
        let mut board = HashMap::new();
        board.insert((0, 0), true);
        App { size, board, is_drawing: true, generation: 0, live_cells: 0 }
    }   

    pub fn get_points(&self) -> Vec<(u16, u16)> {
        self.board
            .iter()
            .filter(|&(_, &v)| v) // Only keep entries where the value is true
            .map(|(&k, _)| k)     // Collect just the keys
            .collect()
    }

    pub fn draw(&mut self, x: u16, y: u16) {
        if self.is_drawing {
            self.board.insert((x, y), true);
        }
    }

    // fn run simulation
    // fn stop simulation
    // fn clear board
}