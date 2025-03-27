use rand::prelude::*;  // Importiert alles aus dem Prelude

mod sudoku {
    pub struct Sudoku {
        grid: [[u8; 9]; 9],
    }

    impl Sudoku {
        pub fn new() -> Sudoku {
            Sudoku {
                grid: [[0; 9]; 9],
            }
        }

        pub fn generate_puzzle(&mut self) {
            self.fill_grid();
            self.remove_numbers(40);
        }

        pub fn fill_grid(&mut self) -> bool {
            for row in 0..9 {
                for col in 0..9 {
                    if self.grid[row][col] == 0 {
                        let mut numbers: Vec<u8> = (1..=9).collect();
                        numbers.shuffle(&mut rand::thread_rng());
                        for &num in &numbers {
                            if self.is_valid(row, col, num) {
                                self.grid[row][col] = num;
                                if self.fill_grid() {
                                    return true;
                                }
                                self.grid[row][col] = 0;
                            }
                        }
                        return false;
                    }
                }
            }
            true
        }

        pub fn is_valid(&self, row: usize, col: usize, num: u8) -> bool {
            for x in 0..9 {
                if self.grid[row][x] == num {
                    return false;
                }
            }
            for x in 0..9 {
                if self.grid[x][col] == num {
                    return false;
                }
            }
            let start_row = row - row % 3;
            let start_col = col - col % 3;
            for i in 0..3 {
                for j in 0..3 {
                    if self.grid[i + start_row][j + start_col] == num {
                        return false;
                    }
                }
            }
            true
        }

        pub fn remove_numbers(&mut self, count: usize) {
            let mut cells: Vec<(usize, usize)> = (0..9)
                .flat_map(|i| (0..9).map(move |j| (i, j)))
                .collect();
            cells.shuffle(&mut rand::thread_rng());
            for &(row, col) in cells.iter().take(count) {
                self.grid[row][col] = 0;
            }
        }

        pub fn get_grid(&self) -> &[[u8; 9]; 9] {
            &self.grid
        }
    }

    pub fn print_grid(grid: &[[u8; 9]; 9]) {
        for i in 0..9 {
            if i % 3 == 0 && i != 0 {
                println!("- - - - - - - - - - - -");
            }
            for j in 0..9 {
                if j % 3 == 0 && j != 0 {
                    print!("| ");
                }
                if grid[i][j] == 0 {
                    print!(". ");
                } else {
                    print!("{} ", grid[i][j]);
                }
            }
            println!();
        }
    }
}

fn main() {
    let mut puzzle = sudoku::Sudoku::new();
    println!("Generiere Sudoku-Rätsel...");
    puzzle.generate_puzzle();
    println!("Sudoku-Rätsel:");
    sudoku::print_grid(puzzle.get_grid());
}