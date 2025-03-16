use std::collections::HashMap;

pub struct App {
    pub size: u32,
    pub board: HashMap<(u32, u32), bool>,
    pub is_drawing: bool,
}

impl App {
    pub fn new() -> Self {
        let size = 15;
        let mut board = HashMap::new();
        board.insert((0, 0), true);
        App { size, board, is_drawing: true }
    }   

    pub fn get_points(&self) -> Vec<(u32, u32)> {
        self.board
            .iter()
            .filter(|&(_, &v)| v) // Only keep entries where the value is true
            .map(|(&k, _)| k)     // Collect just the keys
            .collect()
    }

    // fn run simulation
    // fn stop simulation
    // fn clear board
    // fn draw
}