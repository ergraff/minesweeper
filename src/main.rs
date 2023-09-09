use console::Term;
use rand::Rng;

const SIZE: usize = 20;
const DIFFICULTY: usize = 1;
const DIFFICULTY_OFFSET: usize = 8;

#[derive(Clone)]
enum State {
    Unrevealed,
    Revealed(u8),
    Flagged,
    Mined,
}

struct Board {
    mines: Vec<Vec<bool>>,
    cells: Vec<Vec<State>>,
    position: (usize, usize),
}

impl Board {
    fn new() -> Self {
        Board {
            mines: vec![vec![false; SIZE]; SIZE],
            cells: vec![vec![State::Unrevealed; SIZE]; SIZE],
            position: (0, 0),
        }
    }

    fn randomize(mut self) -> Self {
        let mut rng = rand::thread_rng();
        for i in 0..SIZE {
            for j in 0..SIZE {
                let r = rng.gen_range(0..(DIFFICULTY_OFFSET - DIFFICULTY));
                if r == 0 {
                    self.mines[i][j] = true;
                }
            }
        }
        self
    }

    fn print(&self) {
        fn line() {
            for _ in 0..SIZE {
                print!("+ - ");
            }
            print!("+\n");
        }

        print!("\x1B[2J\x1B[1;1H");
        for i in 0..SIZE {
            line();
            for j in 0..SIZE {
                // Left border
                match (i, j) == self.position {
                    true => print!("||"),
                    false => print!("| "),
                }
                // Cell content
                match self.cells[i][j] {
                    State::Unrevealed => print!("."),
                    State::Revealed(v) => print!("{v}"),
                    State::Flagged => print!("F"),
                    State::Mined => print!("x"),
                }
                // Right border
                match (i, j) == self.position {
                    true => print!("|"),
                    false => print!(" "),
                }
            }
            print!("|\n");
        }
        line();
        // Print info to user
        println!("Arrow keys to move, <a> to reveal cell, <f> to place flag, <Esc> to quit.");
    }

    fn count_neighbors(&self) -> u8 {
        let mut sum: u8 = 0;
        for i in [-1, 0, 1].into_iter() {
            for j in [-1, 0, 1].into_iter() {
                if i == 0 && j == 0 {
                    continue;
                }
                let new_i = self.position.0 as i32 + i;
                let new_j = self.position.1 as i32 + j;
                let bounded =
                    (0..SIZE as i32).contains(&new_i) && (0..SIZE as i32).contains(&new_j);
                let is_mine = self.mines[new_i as usize][new_j as usize] == true;
                if bounded && is_mine {
                    sum += 1;
                }
            }
        }
        sum
    }

    fn flood_empty(&mut self) {}

    fn evaluate_cell(&mut self) -> Option<bool> {
        let mut result = Some(true);
        let mined = self.mines[self.position.0][self.position.1];
        let count = self.count_neighbors();
        match (mined, count) {
            // Mined, end game
            (true, _) => {
                self.cells[self.position.0][self.position.1] = State::Mined;
                result = None;
            }
            // Not mined, continue
            (false, 0) => {
                self.flood_empty();
                // Do something else?
                // todo!();
            }
            (false, _) => {
                self.cells[self.position.0][self.position.1] = State::Revealed(count);
            }
        }
        result
    }

    fn player_action(&mut self, key: console::Key) -> Option<bool> {
        match key {
            console::Key::ArrowUp => {
                if self.position.0 > 0 {
                    self.position.0 -= 1;
                }
                Some(true)
            }
            console::Key::ArrowDown => {
                if self.position.0 < SIZE - 1 {
                    self.position.0 += 1;
                }
                Some(true)
            }
            console::Key::ArrowLeft => {
                if self.position.1 > 0 {
                    self.position.1 -= 1;
                }
                Some(true)
            }
            console::Key::ArrowRight => {
                if self.position.1 < SIZE - 1 {
                    self.position.1 += 1;
                }
                Some(true)
            }
            console::Key::Char('a') => {
                let game_continue = self.evaluate_cell();
                game_continue
            }
            console::Key::Char('f') => {
                self.cells[self.position.0][self.position.1] = State::Flagged;
                Some(true)
            }
            console::Key::Escape => None,
            _ => Some(false),
        }
    }
}

fn main() {
    let stdout = Term::buffered_stdout();
    let mut board = Board::new().randomize();

    // Main game loop
    let mut running = true;
    while running {
        // Print map
        board.print();

        // Read key
        loop {
            if let Ok(key) = stdout.read_key() {
                let legal = board.player_action(key);
                match legal {
                    Some(true) => break,
                    Some(false) => {}
                    None => {
                        running = false;
                        break;
                    }
                }
            }
        }
    }
    board.print();
    println!("Game ended.");
}
