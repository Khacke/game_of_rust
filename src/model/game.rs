#[derive(Clone,Copy)]
pub enum Cell {
    Dead,
    Alive
}

pub enum TimeFlow {
    Stopped,
    Running
}



pub struct GameState {
    pub board: [[Cell;50]; 50], // Vec<Vec<Cell>>,
    pub time_flow: TimeFlow,
}

impl GameState {
    pub fn new() -> Self {
        GameState { board: make_blank_board() , time_flow: TimeFlow::Stopped}
    }
    
    pub fn handle_click(&mut self, row: usize, col: usize) {
        // println!("Clicked at: {col} {row}")
        if row > 48 || col > 48 {
            return;
        }
        self.board[row][col] = match self.board[row][col] {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead
        };
    }
    
    pub fn change_time_flow(&mut self) {
        self.time_flow = match self.time_flow {
            TimeFlow::Stopped => TimeFlow::Running,
            TimeFlow::Running => TimeFlow::Stopped,
        };
    }
    
    pub fn calculate_next_state(&mut self) {
        let mut next = self.board.clone();

        for row in 0..50 {
            for col in 0..50 {
                let cell = self.board[row][col];
                let live_neighbors = self.live_neighbour_count(row as u32, col as u32);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise
                };

                next[row][col] = next_cell;
            }
        }
        self.board = next;
    }
    
    pub fn live_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut count:u8 = 0;

        match (row, col) {
            (0, 0) => {
                let neighbors = [(0,1), (1,1), (1,0)];
                for neighbor in neighbors {
                    match self.board[neighbor.0][neighbor.1] {
                        Cell::Alive => count+=1,
                        _ => {}
                    }
                }
            },
            (0, x) if x < 49 => {
                let neighbors = [(0, x-1), (0, x+1), (1, x-1), (1, x), (1, x+1)];
                for neighbor in neighbors {
                    match self.board[neighbor.0 as usize][neighbor.1 as usize] {
                        Cell::Alive => count+=1,
                        _ => {}
                    }
                }
            },
            (0, 49) => {
                let neighbors = [(0,48), (1,48), (1,49)];
                for neighbor in neighbors {
                    match self.board[neighbor.0 as usize][neighbor.1 as usize] {
                        Cell::Alive => count+=1,
                        _ => {}
                    }
                }
            },
            (49, 0) => {
                let neighbors = [(49,1), (48,1), (48,0)];
                for neighbor in neighbors {
                    match self.board[neighbor.0 as usize][neighbor.1 as usize] {
                        Cell::Alive => count+=1,
                        _ => {}
                    }
                }
            },
            (49, x) if x < 49  => {
                let neighbors = [(49, x-1), (49, x+1), (48, x-1), (48, x), (48, x+1)];
                for neighbor in neighbors {
                    match self.board[neighbor.0 as usize][neighbor.1 as usize] {
                        Cell::Alive => count+=1,
                        _ => {}
                    }
                }
            },
            (49, 49) => {
                let neighbors = [(49,48), (48,48), (48,49)];
                for neighbor in neighbors {
                    match self.board[neighbor.0 as usize][neighbor.1 as usize] {
                        Cell::Alive => count+=1,
                        _ => {}
                    }
                }
            },
            (x, 0) if x < 49 => {
                let neighbors = [(x+1, 0), (x-1, 0), (x,1), (x-1,1), (x+1,1)];
                for neighbor in neighbors {
                    match self.board[neighbor.0 as usize][neighbor.1 as usize] {
                        Cell::Alive => count+=1,
                        _ => {}
                    }
                }
            },
            (x, 49) if x < 49 => {
                let neighbors = [(x+1, 49), (x-1, 49), (x,48), (x-1,48), (x+1,48)];
                for neighbor in neighbors {
                    match self.board[neighbor.0 as usize][neighbor.1 as usize] {
                        Cell::Alive => count+=1,
                        _ => {}
                    }
                }
            }
            (x,y) => {
                let neighbors = [(x-1, y-1), (x-1, y), (x-1, y+1), (x, y-1), (x, y+1), (x+1, y-1), (x+1, y), (x+1, y+1)];
                for neighbor in neighbors {
                    match self.board[neighbor.0 as usize][neighbor.1 as usize] {
                        Cell::Alive => {
                            // println!("x: {x}, y: {y}, n:{},{}, count:{}",neighbor.0, neighbor.1, count+1);
                            count+=1
                        },
                        _ => {}
                    }
                }
            }
        }
        // println!("row{row}, col{col}, count{count}");
        count
    }
}

fn make_blank_board() -> [[Cell;50]; 50] {
    [[Cell::Dead; 50]; 50]
}