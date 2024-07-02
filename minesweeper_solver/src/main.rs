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

}

impl Solver{
    // Solves a board given to it
    fn solver(board: Board) -> Self {
        let new_board = board.clone();
        // new_board.calculate_numbers();
        Solver {
            cells: new_board.cells
        }
    }
}

fn main() {
    let _board = Board::new_board();
    _board.print_board();
}