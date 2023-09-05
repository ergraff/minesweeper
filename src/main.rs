const SIZE: usize = 20;

struct Map {
    map: Vec<Vec<bool>>,
}

impl Map {
    fn new() -> Self {
        Map {
            map: vec![vec![false; SIZE]; SIZE],
        }
    }
    fn print(&self) {
        fn line() {
            for _ in 0..SIZE {
                print!("+ - ");
            }
            print!("+\n");
        }

        line();
        for i in 0..SIZE {
            for j in 0..SIZE {
                match self.map[i][j] {
                    false => print!("|   "),
                    true => print!("| x "),
                }
            }
            print!("|\n");
            line();
        }
    }
}

fn main() {
    let map = Map::new();
    map.print();
}
