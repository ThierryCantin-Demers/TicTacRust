use std::fmt;
use std::io::{self};

#[derive(Copy)]
enum CellType {
    X,
    O,
    Empty,
}

impl CellType {
    fn get_character(&self) -> char {
        match self {
            CellType::X => 'X',
            CellType::O => 'O',
            CellType::Empty => ' ',
        }
    }
}

impl Clone for CellType {
    fn clone(&self) -> Self {
        match self {
            CellType::X => CellType::X,
            CellType::O => CellType::O,
            CellType::Empty => CellType::Empty,
        }
    }
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_character())
    }
}

struct Grid {
    cells: [[CellType; 3]; 3],
}

impl Grid {
    fn new() -> Grid {
        Grid {
            cells: [[CellType::Empty; 3]; 3],
        }
    }

    fn insert(&mut self, row: usize, column: usize, cell_type: CellType) {
        self.cells[row][column] = cell_type;
    }

    fn is_cell_empty(&self, row: usize, column: usize) -> bool {
        match self.cells[row][column] {
            CellType::Empty => true,
            _ => false,
        }
    }

    fn index_from_str(&self, index: &str) -> Option<i32> {
        let index: i32 = if !index.is_empty() && is_string_numeric(index) {
            index
                .parse::<i32>()
                .expect("The index value is not a number.")
        } else {
            println!("The index value is invalid.");
            -1
        };

        match index {
            0 | 1 | 2 => Some(index),
            _ => {
                println!("The index value is invalid.");
                None
            }
        }
    }

    // Determines if there is a winner in the 3x3 grid.
    fn get_winner(&self) -> Option<CellType> {
        // Check rows
        for row in 0..=2 {
            if self.cells[row][0].get_character() == self.cells[row][1].get_character()
                && self.cells[row][1].get_character() == self.cells[row][2].get_character()
                && self.cells[row][0].get_character() != ' '
            {
                return Some(self.cells[row][0]);
            }
        }

        // Check columns
        for column in 0..=2 {
            if self.cells[0][column].get_character() == self.cells[1][column].get_character()
                && self.cells[1][column].get_character() == self.cells[2][column].get_character()
                && self.cells[0][column].get_character() != ' '
            {
                return Some(self.cells[0][column]);
            }
        }

        // Check diagonals

        if (self.cells[0][0].get_character() == self.cells[1][1].get_character()
            && self.cells[1][1].get_character() == self.cells[2][2].get_character()
            && self.cells[0][0].get_character() != ' ')
            || (self.cells[0][2].get_character() == self.cells[1][1].get_character()
                && self.cells[1][1].get_character() == self.cells[2][0].get_character()
                && self.cells[0][2].get_character() != ' ')
        {
            return Some(self.cells[0][0]);
        }

        return None;
    }

    fn is_tie(&self) -> bool {
        for row in 0..=2 {
            for column in 0..=2 {
                if self.cells[row][column].get_character() == ' ' {
                    return false;
                }
            }
        }

        return true;
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        output.push_str(" -------------\n");
        for row in self.cells {
            for cell in row {
                output.push_str(" | ");
                output.push_str(&format!("{cell}")[..]);
            }
            output.push_str(" |\n -------------\n");
        }

        write!(f, "{}", output)
    }
}

fn get_from_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    String::from(input.trim())
}

fn is_string_numeric(str: &str) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut grid: Grid = Grid::new();
    let player_cell_type = loop {
        println!("Do you want to play as X or O?");

        let cell_type: String = get_from_input();

        match cell_type.as_str() {
            "X" | "x" => break CellType::X,
            "O" | "o" => break CellType::O,
            _ => (),
        };
    };
    let bot_cell_type = match player_cell_type {
        CellType::X => CellType::O,
        CellType::O => CellType::X,
        _ => CellType::Empty,
    };

    loop {
        let position: (i32, i32) = loop {
            println!("\n\nWhere do you want to place your {}?\nWrite your answer in this format \"row,column\" (each between 0 and 2)", player_cell_type);
            println!("{grid}");

            let input = get_from_input();
            if let Some((x, y)) = input.split_once(',') {
                let x = grid.index_from_str(x);
                let y = grid.index_from_str(y);

                if x == None || y == None {
                    continue;
                }
                let x = x.unwrap();
                let y = y.unwrap();

                if grid.is_cell_empty(x as usize, y as usize) {
                    break (x, y);
                } else {
                    println!("The position you chose already contains a value. Try again.");
                    continue;
                }
            } else {
                println!("Your answer is not written in the right format.");
            }
        };

        grid.insert(position.0 as usize, position.1 as usize, player_cell_type);

        let winner = grid.get_winner();

        println!("{grid}");

        if !winner.is_none() {
            println!("The game is over!!!");
            println!("You won!!!! Damn bruh");
            break;
        } else if grid.is_tie() {
            println!("The game is over!!!");
            println!("It's a tie, you're both cringe");
            break;
        }

        // Bot's turn
        loop {
            let pos: (i32, i32) = (
                (rand::random::<u32>() % 3) as i32,
                (rand::random::<u32>() % 3) as i32,
            );
            if grid.is_cell_empty(pos.0 as usize, pos.1 as usize) {
                grid.insert(pos.0 as usize, pos.1 as usize, bot_cell_type);
                break;
            }
        }

        let winner = grid.get_winner();

        println!("{grid}");

        if !winner.is_none() {
            println!("The game is over!!!");
            println!("bruh bruh bruh, you lost to a bot, you're cringe");
            break;
        } else if grid.is_tie() {
            println!("The game is over!!!");
            println!("It's a tie, you're both cringe");
            break;
        }
    }
}
