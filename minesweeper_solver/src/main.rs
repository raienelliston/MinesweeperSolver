use rand::Rng;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;
const MINES: usize = 10;

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Mine,
    Unkown,
    Clicked,
    Empty(u8),
}

#[derive(Debug, Clone, PartialEq)]
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
    fn print_board(&self, text: &str) {
        println!("{}", text);
        for row in &self.cells {
            for cell in row {
                match cell {
                    Cell::Mine => print!("X"),
                    Cell::Empty(n) => print!("{}", n),
                    Cell::Clicked => print!("C"),
                    Cell::Unkown => print!("U"),
                }
            }
            println!();
        }
    }

    fn update_board(&self, mut board: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if board[y][x] == Cell::Clicked {
                    if self.cells[y][x] == Cell::Mine {
                        board[y][x] = Cell::Mine;
                        println!("Mine at ({}, {})", x, y);
                        for row in &board {
                            for cell in row {
                                match cell {
                                    Cell::Mine => print!("X"),
                                    Cell::Empty(n) => print!("{}", n),
                                    Cell::Clicked => print!("C"),
                                    Cell::Unkown => print!("U"),
                                }
                            }
                            println!();
                        }
                        std::process::exit(0);
                    } else {
                        let mut count = 0;
                        for i in -1..2 {
                            for j in -1..2 {
                                if x as i32 + i >= 0 && x as i32 + i < WIDTH as i32 && y as i32 + j >= 0 && y as i32 + j < HEIGHT as i32 {
                                    if self.cells[(y as i32 + j) as usize][(x as i32 + i) as usize] == Cell::Mine {
                                        count += 1;
                                    }
                                }
                            }
                        }
                        board[y][x] = Cell::Empty(count);
                    }
                }
                if let Cell::Empty(n) = board[y][x] {
                    if n == 0 {
                        for i in -1..2 {
                            for j in -1..2 {
                                if x as i32 + i >= 0 && x as i32 + i < WIDTH as i32 && y as i32 + j >= 0 && y as i32 + j < HEIGHT as i32 {
                                    if board[(y as i32 + j) as usize][(x as i32 + i) as usize] == Cell::Unkown {
                                        board[(y as i32 + j) as usize][(x as i32 + i) as usize] = Cell::Clicked;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        board
    }

    fn create_unkown_board(&self) -> Vec<Vec<Cell>> {
        let empty_cell = Cell::Unkown;
        let mut board = vec![vec![empty_cell.clone(); WIDTH]; HEIGHT];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                for i in -1..2 {
                    for j in -1..2 {
                        if x as i32 + 
                    }
                }
            }
        }
        board
    }
}

impl Solver{
    // Solves a board given to it
    fn new(pre_board: Vec<Vec<Cell>>) -> Self {
        let empty_cell = Cell::Unkown;
        let cells = pre_board.clone();
        let mut board = Solver {
            cells
        };
        board
    }

    fn print_board(&self, text: &str) {
        println!("{}", text);
        for row in &self.cells {
            for cell in row {
                match cell {
                    Cell::Mine => print!("X"),
                    Cell::Empty(n) => print!("{}", n),
                    Cell::Clicked => print!("C"),
                    Cell::Unkown => print!("U"),
                }
            }
            println!();
        }
    }

    fn solve_board(&mut self) {
        let mut board = self.cells.clone();
        let mut prev_board = self.cells.clone();
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    println!("Checking ({}, {})", x, y);
                    if let Cell::Empty(n) = board[y][x] {
                    let mut count = 0;
                        for i in -1..2 {
                            for j in -1..2 {
                                if x as i32 + i >= 0 && x as i32 + i < WIDTH as i32 && y as i32 + j >= 0 && y as i32 + j < HEIGHT as i32 {
                                    if board[(y as i32 + j) as usize][(x as i32 + i) as usize] == Cell::Clicked {
                                        count += 1;
                                    }
                                }
                            }
                        }
                        if count == n {
                            for i in -1..2 {
                                for j in -1..2 {
                                    if x as i32 + i >= 0 && x as i32 + i < WIDTH as i32 && y as i32 + j >= 0 && y as i32 + j < HEIGHT as i32 {
                                        if board[(y as i32 + j) as usize][(x as i32 + i) as usize] == Cell::Unkown {
                                            self.cells[(y as i32 + j) as usize][(x as i32 + i) as usize] = Cell::Clicked;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            self.print_board("After first pass");

            // if prev_board == board {
            //     println!("No progress made");
            //     break;
            // }
    }

    fn is_solved(&self, board: Board) -> bool {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if let Cell::Empty(n) = board.cells[y][x] {
                    if n == 0 {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn click_cell(&mut self, x: usize, y: usize) {
        self.cells[y][x] = Cell::Clicked;
    }
}

fn main() {
    let _board = Board::new_board();
    _board.print_board("Initial");
    let mut _solver = Solver::new(_board.create_unkown_board());
    // _solver.solve_board(_board);
    
    _solver.click_cell(4, 4);

    // while !_solver.is_solved(_board.clone()) {
    for _ in 0..10 {
        _solver.solve_board();
        _solver.cells = _board.update_board(_solver.cells.clone());
        _solver.print_board("After update");
    }
    _solver.print_board("Final");
    _board.print_board("Actual");

}