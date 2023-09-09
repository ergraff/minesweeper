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
                match self.mines[i][j] {
                    true => print!("x"),
                    false => print!(" "),
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
                println!("Select key");
                Some(true)
            }
            console::Key::Char('f') => {
                println!("Flag key");
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
    println!("Game ended.");
}
