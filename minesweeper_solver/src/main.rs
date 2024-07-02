use rand::Rng;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;
const MINES: usize = 10;

#[derive(Debug, Clone)]
enum Cell {
    Mine,
    Empty(u8),
}

#[derive(Debug, Clone)]
struct Board {
    cells: Vec<Vec<Cell>>,
}

#[derive(Debug, Clone)]
struct Solver {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    // Create a new board with empty cells and place mines
    fn new_board() -> Self {
        let empty_cell = Cell::Empty(0);
        let cells = vec![vec![empty_cell.clone(); WIDTH]; HEIGHT];
        let mut board = Board {
            cells
        };
        board.place_mines();
        
        // board.calculate_numbers();
        board
    }

    fn place_mines(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..MINES {
            let x = rng.gen_range(0..WIDTH);
            let y = rng.gen_range(0..HEIGHT);
            self.cells[y][x] = Cell::Mine;
        }
    }

    // Prints the
    fn print_board(&self) {
        for row in &self.cells {
            for cell in row {
                match cell {
                    Cell::Mine => print!("X"),
                    Cell::Empty(n) => print!("{}", n),
                }
            }
            println!();
        }
    }

    fn calculate_numbers(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let mut count = 0;
                for i in -1..2 {
                    for j in -1..2 {
                        let new_x = x as i32 + i;
                        let new_y = y as i32 + j;
                        if new_x >= 0 && new_x < WIDTH as i32 && new_y >= 0 && new_y < HEIGHT as i32 {
                            if let Cell::Mine = self.cells[new_y as usize][new_x as usize] {
                                count += 1;
                            }
                        }
                    }
                }
                if let Cell::Empty(n) = self.cells[y][x] {
                    self.cells[y][x] = Cell::Empty(count);
                }
            }
        }
    }
}

impl Solver{
    // Solves a board given to it
    fn solver(board: Board) -> Self {
        let empty_cell = Cell::Empty(0);
        let solved_board = board.clone();
        let cells = vec![vec![empty_cell.clone(); WIDTH]; HEIGHT];
        let mut board = Solver {
            cells
        };
        board
    }

    fn solve_board(&self, board: Board) {
        while !self.is_solved(board.clone()) {
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    if let Cell::Empty(n) = board.cells[y][x] {
                        if n == 0 {
                            self.click(x, y);
                        }
                }
            }
        }
    }

    fn is_solved(&self, board: Board) -> bool {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if let Cell::Empty(n) = board.cells[y][x] {
                    if n == 0 {
                        println!("{}", n);
                    }
                }
            }
        }
        true
    }

    fn click(&self, x: usize, y: usize) {
        println!("Clicking at ({}, {})", x, y);
        self.cells[y][x] = self.solve_board[y][x];
    }
}

fn main() {
    let _board = Board::new_board();
    _board.print_board();
    let _solver = Solver::solver(_board);
    _solver.is_solved(_board);
}