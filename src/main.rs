extern crate ncurses;
use ncurses::*;

struct SudokuTable {
    array: [[u8; 9]; 9],
}

impl SudokuTable {
    fn new() -> SudokuTable {
        SudokuTable { array: [[0; 9]; 9] }
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
    fn find_first_chunk_index(&self, current_index: &usize) -> usize {
        match current_index {
            0..=2 => 0,
            3..=5 => 3,
            6..=8 => 6,
            _ => panic!("Wrong input index"),
        }
    }
    fn check_rules(&self, value: u8, position: &Position) -> bool {
        if value == 0 {
            return true;
        }
        for row in self.array.iter() {
            if row[position.y] == value {
                return false;
            }
        }
        for column_value in self.array[position.x].iter() {
            if column_value == &value {
                return false;
            }
        }
        let x_chunk_start_index = self.find_first_chunk_index(&position.x);
        let y_chunk_start_index = self.find_first_chunk_index(&position.y);
        for row in self.array[x_chunk_start_index..=x_chunk_start_index + 2].iter() {
            for column_value in row[y_chunk_start_index..=y_chunk_start_index + 2].iter() {
                if column_value == &value {
                    return false;
                }
            }
        }
        true
    }
    fn set_value(&mut self, value: String, position: &Position) {
        let parsing_result = value.trim().parse();
        match parsing_result {
            Ok(0..=9) => {
                let number = parsing_result.unwrap();
                if self.check_rules(number, &position) {
                    self.array[position.x][position.y] = number;
                }
            }
            _ => {}
        }
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
    let help_string = "move cursor: [w, a, s, d]\nset value: enter, press digit, enter\n";
    loop {
        addstr(help_string);
        addstr(&position.display_position());
        addstr(&table.display_view(&position));
        let ch = getch();
        match ch {
            113 => break,
            97 => &position.move_left(),
            100 => &position.move_right(),
            119 => &position.move_top(),
            115 => &position.move_bottom(),
            10 => &{
                let mut input_string = String::new();
                getstr(&mut input_string);
                &table.set_value(input_string, &position);
            },
            _ => &{},
        };
        clear();
    }
    endwin();
}
