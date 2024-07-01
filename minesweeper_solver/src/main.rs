use rand::Rng;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;
const MINES: usize = 10;

#[derive(Debug)]
enum Cell {
    Mine,
    Empty(u8),
}

#[derive(Debug)]
struct Board {
    cells: [[Cell; WIDTH]; HEIGHT],
}

impl Board {
    fn new() -> Self {
        let empty_cell = Cell::Empty(0);
        let cells = vec![vec![empty_cell.clone(); WIDTH]; HEIGHT];
        let mut board = Board {
            cells
        };
        board.place_mines();
        println!("{:?}", board);
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

}

fn main() {
    let _board = Board::new();
}