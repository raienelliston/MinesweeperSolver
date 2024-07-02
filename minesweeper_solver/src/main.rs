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
        
        board
    }

    fn place_mines(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..MINES {
            let x = rng.gen_range(0..WIDTH);
            let y = rng.gen_range(0..HEIGHT);
            self.cells[y][x] = Cell::Mine;
        }
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.cells[y][x] == Cell::Mine {
                    continue;
                }
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
                self.cells[y][x] = Cell::Empty(count);
            }
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
                        self.print_board("Actual");
                        std::process::exit(0);
                    }

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
                board = self.update_zeros(board.clone());
            }
        }
        board
    }

    fn update_zeros(&self, mut board: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if let Cell::Empty(0) = board[y][x] {
                    for i in -1..2 {
                        for j in -1..2 {
                            if x as i32 + i >= 0 && x as i32 + i < WIDTH as i32 && y as i32 + j >= 0 && y as i32 + j < HEIGHT as i32 {
                                if board[(y as i32 + j) as usize][(x as i32 + i) as usize] == Cell::Unkown {
                                    let mut count = 0;
                                    for k in -1..2 {
                                        for l in -1..2 {
                                            if x as i32 + i + k >= 0 && x as i32 + i + k < WIDTH as i32 && y as i32 + j + l >= 0 && y as i32 + j + l < HEIGHT as i32 {
                                                if self.cells[(y as i32 + j + l) as usize][(x as i32 + i + k) as usize] == Cell::Mine {
                                                    count += 1;
                                                }
                                            }
                                        }
                                    }

                                    board[(y as i32 + j) as usize][(x as i32 + i) as usize] = Cell::Empty(count);
                                    println!("Clicked ({}, {})", x as i32 + i, y as i32 + j);
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

        board[0][0] = Cell::Clicked;
        board[0][1] = Cell::Clicked;
        board[1][0] = Cell::Clicked;

        board
    }
}

impl Solver{
    // Solves a board given to it
    fn new(pre_board: Vec<Vec<Cell>>) -> Self {
        Solver {
            cells: pre_board.clone(),
        };
        let cells = pre_board.clone();
        let board = Solver {
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
        let mut progress = true;
        let mut minus_ones: Vec<Vec<[i32; 2]>> = vec![];
        let mut cells: Vec<[i32; 2]> = vec![];
    
        while progress {
            progress = false;
            cells.clear();
            let board = self.cells.clone();
    
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    if let Cell::Empty(n) = board[y][x] {
                        // Check if all mines are found
                        let mut count = 0;
                        for i in -1..=1 {
                            for j in -1..=1 {
                                let (nx, ny) = (x as i32 + i, y as i32 + j);
                                if nx >= 0 && nx < WIDTH as i32 && ny >= 0 && ny < HEIGHT as i32 {
                                    
                                    if nx == x.try_into().unwrap() && ny == y.try_into().unwrap() {
                                        continue;
                                    }
                                    if board[ny as usize][nx as usize] == Cell::Mine {
                                        count += 1;
                                    }
                                }
                            }
                        }
                        
                        // If all mines are found, click all unknown cells
                        if count == n {
                            for i in -1..=1 {
                                for j in -1..=1 {
                                    let (nx, ny) = (x as i32 + i, y as i32 + j);
                                    
                                    if nx == x.try_into().unwrap() && ny == y.try_into().unwrap() {
                                        continue;
                                    }

                                    if nx >= 0 && nx < WIDTH as i32 && ny >= 0 && ny < HEIGHT as i32 {
                                        if board[ny as usize][nx as usize] == Cell::Unkown {
                                            self.cells[ny as usize][nx as usize] = Cell::Clicked;
                                            self.print_board("Clicked Them");
                                        }
                                    }
                                }
                            }
                        }
                        // Handle the case where n > 0 and n - 1 mines are found
                        if n > 0 && n - 1 == count {
                            let mut minus_one: Vec<[i32; 2]> = vec![];
                            for i in -1..=1 {
                                for j in -1..=1 {
                                    let (nx, ny) = (x as i32 + i, y as i32 + j);
                                    if nx >= 0 && nx < WIDTH as i32 && ny >= 0 && ny < HEIGHT as i32 {
                                        if board[ny as usize][nx as usize] == Cell::Unkown {
                                            minus_one.push([ny, nx]);
                                        }
                                    }
                                }
                            }
                            if minus_one.len() == n as usize{
                                if minus_one.len() - 1 == count.into() {
                                    minus_ones.push(minus_one);
                                }
                            }
                        }
                                      
                    }
                }
            }
    
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    if let Cell::Empty(n) = board[y][x] {

                        let mut difference: i32 = 0;
                        // Track unknown cells for later
                        let mut cells: Vec<[i32; 2]> = Vec::new();
                        // Check if all cells are mines
                        let mut unkown_count = 0;
                        for i in -1..=1 {
                            for j in -1..=1 {
                                let (nx, ny) = (x as i32 + i, y as i32 + j);
                                if nx == x.try_into().unwrap() && ny == y.try_into().unwrap() {
                                    continue;
                                }

                                if nx >= 0 && nx < WIDTH as i32 && ny >= 0 && ny < HEIGHT as i32 {
                                    if board[ny as usize][nx as usize] == Cell::Unkown{
                                        unkown_count += 1;
                                        cells.push([ny, nx]);
                                    }
                                    if board[ny as usize][nx as usize] == Cell::Mine {
                                        unkown_count += 1;
                                    }
                                }
                            }
                        }

                        if unkown_count > n {
                            continue;
                        }

                        difference = (n - unkown_count).try_into().unwrap();
            
                        // If all cells are mines, click all unknown cells
                        if unkown_count == n {
                            for i in -1..=1 {
                                for j in -1..=1 {
                                    let (nx, ny) = (x as i32 + i, y as i32 + j);
                                    if nx == x.try_into().unwrap() && ny == y.try_into().unwrap() {
                                        continue;
                                    }

                                    if nx >= 0 && nx < WIDTH as i32 && ny >= 0 && ny < HEIGHT as i32 {
                                        if board[ny as usize][nx as usize] == Cell::Unkown {
                                            self.print_board("Before");
                                            self.cells[ny as usize][nx as usize] = Cell::Mine;
                                            progress = true;
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Check for sets of n - 1 mines among the unknowns
                        if !progress {

                            for set in minus_ones.iter() {
                                let mut difference = 0;
                                for &[ny, nx] in set {
                                    if board[ny as usize][nx as usize] == Cell::Unkown {
                                        difference += 1;
                                    }
                                }
                                if difference == set.len() as i32 {
                                    for &[ny, nx] in set {
                                        cells.retain(|&cell| cell != [ny, nx]);
                                        progress = true;
                                    }
                                }
                            }   

                            if difference == cells.len() as i32 {
                                for [ny, nx] in cells {
                                    self.cells[ny as usize][nx as usize] = Cell::Clicked;
                                    progress = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let _board = Board::new_board();
    _board.print_board("Initial");
    let mut _solver = Solver::new(_board.create_unkown_board());

    let mut pre_board = _solver.cells.clone();
    while pre_board.iter().any(|row| row.contains(&Cell::Clicked)){
        _solver.solve_board();
        pre_board = _solver.cells.clone();
        _solver.print_board("After update");
        _solver.cells = _board.update_board(_solver.cells.clone()); 
    }
    _solver.print_board("Final");
    _board.print_board("Actual");

}