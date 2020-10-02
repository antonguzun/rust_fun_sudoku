use std::char;
extern crate ncurses;
use ncurses::*;

struct SudokuTable {
    array: [[u8; 9]; 9],
}

impl SudokuTable {
    fn new() -> SudokuTable {
        SudokuTable {
            array: [[0u8; 9]; 9],
        }
    }
    fn display_view(&self, cursor: &Position) -> String {
        let mut string_data = String::new();

        for (i, row) in self.array.iter().enumerate() {
            if i == 0 || i % 3 == 0 {
                string_data += "_____________________\n"
            }
            for (j, column_element) in row.iter().enumerate() {
                if j == 0 || j % 3 == 0 {
                    string_data += "|";
                }
                string_data += " ";
                if cursor.x == i && cursor.y == j {
                    string_data += "X";
                } else {
                    string_data += &column_element.to_string();
                }
            }
            string_data += "\n";
        }
        string_data
    }
    fn set_value(&mut self, value: u8, position: &Position) {
        self.array[position.x][position.y] = value;
    }
}

struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new() -> Position {
        Position { x: 0, y: 0 }
    }
    fn move_left(&mut self) {
        if self.y != 0 {
            self.y -= 1;
        }
    }
    fn move_right(&mut self) {
        if self.y != 8 {
            self.y += 1;
        }
    }
    fn move_top(&mut self) {
        if self.x != 0 {
            self.x -= 1;
        }
    }
    fn move_bottom(&mut self) {
        if self.x != 8 {
            self.x += 1;
        }
    }
    fn display_position(&self) -> String {
        format!("current position {}, {}\n", self.x, self.y)
    }
}

fn main() {
    initscr();
    let mut table: SudokuTable = SudokuTable::new();
    let mut position: Position = Position::new();
    loop {
        addstr(&table.display_view(&position));
        refresh();
        addstr(&position.display_position());
        let ch = getch();
        match ch {
            113 => break,
            97 => &position.move_left(),
            100 => &position.move_right(),
            119 => &position.move_top(),
            115 => &position.move_bottom(),
            10 => &{
                let char_code = getch();
                println!("char_code {}", char_code);
                println!("char {:?}", char::from_digit(char_code as u32, 10));
                &table.set_value(9 as u8, &position);
            },
            _ => &{},
        };
        clear();
    }
    endwin();
}
