use console::{style, Term};
use rand::Rng;

const SIZE: usize = 20;
const DIFFICULTY: usize = 1;
const DIFFICULTY_OFFSET: usize = 8;

type Pos = (usize, usize);

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
    position: Pos,
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
                print!("{}", style("+ - ").dim());
            }
            println!("{}", style("+").dim());
        }

        print!("\x1B[2J\x1B[1;1H");
        for i in 0..SIZE {
            line();
            for j in 0..SIZE {
                // Left border
                match (i, j) == self.position {
                    true => print!("{}|", style("|").dim()),
                    false => print!("{} ", style("|").dim()),
                }
                // Cell content
                match self.cells[i][j] {
                    State::Unrevealed => print!("▒"),
                    State::Revealed(0) => print!(" "),
                    State::Revealed(v) => match v {
                        1 => print!("{}", style("1").cyan()),
                        2 => print!("{}", style("2").green()),
                        3 => print!("{}", style("3").red()),
                        4 => print!("{}", style("4").blue()),
                        5 => print!("{}", style("5").red()),
                        6 => print!("{}", style("6").blue()),
                        7 => print!("{}", style("7").black()),
                        8 => print!("{}", style("8").white()),
                        _ => {}
                    },
                    State::Flagged => print!("{}", style("F").red().dim()),
                    State::Mined => print!("x"),
                }
                // Right border
                match (i, j) == self.position {
                    true => print!("|"),
                    false => print!(" "),
                }
            }
            println!("|");
        }
        line();
        // Print info to user
        println!("Arrow keys to move, <a> to reveal cell, <f> to place flag, <Esc> to quit.");
    }

    fn count_neighbors_at(&self, pos: Pos) -> u8 {
        let mut sum: u8 = 0;
        for i in [-1, 0, 1].into_iter() {
            for j in [-1, 0, 1].into_iter() {
                if i == 0 && j == 0 {
                    continue;
                }
                let new_i = pos.0 as i32 + i;
                let new_j = pos.1 as i32 + j;
                let bounded =
                    (0..SIZE as i32).contains(&new_i) && (0..SIZE as i32).contains(&new_j);
                if bounded && self.mines[new_i as usize][new_j as usize] == true {
                    sum += 1;
                }
            }
        }
        sum
    }

    fn flood_empty(&mut self) {
        fn get_neighbor_positions(pos: Pos) -> Vec<Pos> {
            let mut result = vec![];
            for i in [-1, 0, 1].into_iter() {
                for j in [-1, 0, 1].into_iter() {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    let new_i = pos.0 as i32 + i;
                    let new_j = pos.1 as i32 + j;
                    let bounded =
                        (0..SIZE as i32).contains(&new_i) && (0..SIZE as i32).contains(&new_j);
                    if bounded {
                        result.push((new_i as usize, new_j as usize));
                    }
                }
            }
            result
        }

        // Initial positions
        let mut to_be_checked = get_neighbor_positions(self.position);
        let mut visited = vec![self.position];

        while !to_be_checked.is_empty() {
            let mut next_neighbors = vec![];
            for pos in &to_be_checked {
                let count = self.count_neighbors_at(*pos);
                let mined = self.mines[pos.0][pos.1];
                match (count, mined) {
                    (_, true) => continue,
                    (0, false) => {
                        // New neighbors to be checked that have not been visited
                        let mut neighbors = get_neighbor_positions(*pos);
                        neighbors.retain(|p| !visited.contains(p));
                        next_neighbors.append(&mut neighbors);
                    }
                    _ => {}
                }
                self.cells[pos.0][pos.1] = State::Revealed(count);
                visited.push(*pos);
            }
            // Remove visited positions
            to_be_checked.retain(|p| !visited.contains(p));
            // Append positions to be checked next
            to_be_checked.append(&mut next_neighbors);
        }
    }

    fn evaluate_cell(&mut self) -> Option<bool> {
        let mut result = Some(true);
        let mined = self.mines[self.position.0][self.position.1];
        let count = self.count_neighbors_at(self.position);
        let cell = &mut self.cells[self.position.0][self.position.1];
        match mined {
            true => {
                *cell = State::Mined;
                result = None;
            }
            false => {
                *cell = State::Revealed(count);
                if count == 0 {
                    self.flood_empty();
                }
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
            console::Key::Char('a') => self.evaluate_cell(),
            console::Key::Char('f') => {
                let cell = &mut self.cells[self.position.0][self.position.1];
                match *cell {
                    State::Unrevealed => *cell = State::Flagged,
                    State::Flagged => *cell = State::Unrevealed,
                    _ => {}
                }
                Some(true)
            }
            console::Key::Escape => None,
            _ => Some(false),
        }
    }

    fn game_finished(&self) -> bool {
        for i in 0..SIZE {
            for j in 0..SIZE {
                match (self.mines[i][j], &self.cells[i][j]) {
                    (false, State::Unrevealed) => return false,
                    _ => {}
                }
            }
        }
        true
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

        // Check if game is over
        if board.game_finished() {
            break;
        }
    }
    board.print();
    println!("Game ended.");
}
