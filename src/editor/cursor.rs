pub mod cursor {
    #[derive(Debug)]
    pub struct CursorPosition {
        pub line: usize,
        pub column: usize,
        column_offset: usize,
    }

    impl PartialEq for CursorPosition {
        fn eq(&self, other: &Self) -> bool {
            self.line == other.line && self.column == other.column
        }
    }

    impl Eq for CursorPosition {}

    use std::cmp::Ordering;

    impl PartialOrd for CursorPosition {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if self.line > other.line {
                Some(Ordering::Greater)
            } else if self.line < other.line {
                Some(Ordering::Less)
            } else if self.column > other.column {
                Some(Ordering::Greater)
            } else if self.column < other.column {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Equal)
            }
        }
    }

    #[derive(Debug)]
    pub struct Cursor {
        pub current: CursorPosition,
        pub extender: CursorPosition,
        pub lines: Vec<String>,
    }
    use std::error::Error;
    use std::fs;
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    use std::path::Path;
    impl Cursor {
        pub fn new(lines: Vec<String>) -> Cursor {
            Cursor {
                current: CursorPosition {
                    line: 0,
                    column: 0,
                    column_offset: 0,
                },
                extender: CursorPosition {
                    line: 0,
                    column: 0,
                    column_offset: 0,
                },
                lines: lines,
            }
        }
        pub fn from_file(file_name: &str) -> Cursor {
            let string_file = fs::read_to_string(file_name);
            if let Ok(actual_content) = string_file {
                Cursor::new(
                    actual_content
                        .split('\n')
                        .map(|string| String::from(string))
                        .collect::<Vec<String>>(),
                )
            } else {
                panic!("Could not read file: {}", file_name);
            }
        }
        pub fn to_file(&mut self, file_name: &str) -> Result<(), io::Error> {
            let path = Path::new(file_name);
            let display = path.display();

            let mut file = match File::create(&path) {
                Err(why) => panic!("couldn't create {}: {}", display, why.description()),
                Ok(file) => file,
            };

            file.write_all(
                self.lines
                    .clone()
                    .into_iter()
                    .collect::<Vec<String>>()
                    .join("\n")
                    .as_bytes(),
            )
        }
        pub fn add(&mut self, character: char) {
            if self.current != self.extender {
                self.delete();
            }
            if self.current.column == self.lines[self.current.line].chars().count() {
                self.lines[self.current.line].push(character);
            } else {
                self.lines[self.current.line] = self.lines[self.current.line]
                    .chars()
                    .enumerate()
                    .fold("".to_string(), |acc, (index, current_character)| {
                        if self.current.column == index {
                            format!("{}{}{}", acc, character, current_character)
                        } else {
                            format!("{}{}", acc, current_character)
                        }
                    });
            }
            self.right(false);
        }
        fn filter_character(line: &String, condition: impl Fn(usize) -> bool) -> String {
            line.chars()
                .enumerate()
                .fold("".to_string(), |acc, (index, current_character)| {
                    if condition(index) {
                        format!("{}{}", acc, current_character)
                    } else {
                        format!("{}", acc)
                    }
                })
        }
        pub fn delete(&mut self) {
            if &self.current == &self.extender {
                if &self.current.column == &self.lines[self.current.line].chars().count() {
                    if self.current.line + 1 < self.lines.len() {
                        let next_line = self.lines[self.current.line + 1].to_string();
                        self.lines[self.current.line].push_str(&next_line);
                        self.lines.remove(self.current.line + 1);
                    }
                } else {
                    self.lines[self.current.line] =
                        Cursor::filter_character(&self.lines[self.current.line], |index| {
                            index != self.current.column
                        });
                }
            } else {
                if self.current.line == self.extender.line {
                    let (lesser_column, greater_column) =
                        if self.current.column > self.extender.column {
                            (self.extender.column, self.current.column)
                        } else {
                            (self.current.column, self.extender.column)
                        };
                    self.lines[self.current.line] =
                        Cursor::filter_character(&self.lines[self.current.line], |index| {
                            index < lesser_column || index >= greater_column
                        });
                } else {
                    let (end_of_line_delete, beginning_of_line_delete) =
                        if self.current < self.extender {
                            (&self.current, &self.extender)
                        } else {
                            (&self.extender, &self.current)
                        };
                    self.lines[end_of_line_delete.line] =
                        Cursor::filter_character(&self.lines[end_of_line_delete.line], |index| {
                            index < end_of_line_delete.column
                        });

                    self.lines[beginning_of_line_delete.line] = Cursor::filter_character(
                        &self.lines[beginning_of_line_delete.line],
                        |index| index >= beginning_of_line_delete.column,
                    );

                    let next_line = self.lines[beginning_of_line_delete.line].to_string();

                    self.lines[end_of_line_delete.line].push_str(&next_line);

                    for _line_index in (end_of_line_delete.line + 1)..=beginning_of_line_delete.line
                    {
                        self.lines.remove(end_of_line_delete.line + 1);
                    }
                }
                if self.current < self.extender {
                    self.extender.column = self.current.column;
                    self.extender.line = self.current.line;
                } else {
                    self.current.column = self.extender.column;
                    self.current.line = self.extender.line;
                }
            }
        }
        pub fn backspace(&mut self) {
            if &self.current == &self.extender {
                if self.current.column != 0 || self.current.line != 0 {
                    self.left(false);
                    self.delete();
                }
            } else {
                self.delete();
            }
        }
        pub fn new_line(&mut self) {
            if self.current != self.extender {
                self.delete();
            }
            let (remaining_current_line, new_next_line) =
                self.lines[self.current.line].chars().enumerate().fold(
                    ("".to_string(), "".to_string()),
                    |acc, (index, current_character)| {
                        if index >= self.current.column {
                            (acc.0, format!("{}{}", acc.1, current_character).to_string())
                        } else {
                            (format!("{}{}", acc.0, current_character).to_string(), acc.1)
                        }
                    },
                );
            let (remaining_current_line_string, new_next_line_string) = (
                remaining_current_line.to_string(),
                new_next_line.to_string(),
            );

            self.lines
                .insert(self.current.line, remaining_current_line_string);
            self.lines
                .insert(self.current.line + 2, new_next_line_string);
            self.lines.remove(self.current.line + 1);

            self.current.line += 1;
            self.extender.line += 1;
            self.current.column = 0;
            self.extender.column = 0;
        }
        pub fn home(&mut self, select: bool) {
            let moving_cursor = self.get_moving_cursor(select);
            let move_to = if moving_cursor.column == 0
                && self.lines[moving_cursor.line].chars().count() >= moving_cursor.column_offset
            {
                moving_cursor.column_offset
            } else {
                0
            };
            self.extender.column = move_to;
            if self.cursors_need_sync(select) {
                self.current.column = move_to;
            }
        }
        pub fn end(&mut self, select: bool) {
            let moving_cursor = self.get_moving_cursor(select);
            let current_line_max_column = self.lines[moving_cursor.line].chars().count();
            let move_to = if moving_cursor.column == current_line_max_column
                && current_line_max_column >= moving_cursor.column_offset
            {
                moving_cursor.column_offset
            } else {
                current_line_max_column
            };

            self.extender.column = move_to;
            if self.cursors_need_sync(select) {
                self.current.column = move_to;
            }
        }
        pub fn left(&mut self, select: bool) {
            let moving_cursor = self.get_moving_cursor(select);
            let (new_start_line, new_start_column) = if moving_cursor.column == 0 {
                if moving_cursor.line != 0 {
                    let previous_line = moving_cursor.line - 1;
                    (previous_line, self.lines[previous_line].chars().count())
                } else {
                    (moving_cursor.line, moving_cursor.column)
                }
            } else {
                (moving_cursor.line, moving_cursor.column - 1)
            };
            self.extender.column = new_start_column;
            self.extender.line = new_start_line;
            self.extender.column_offset = new_start_column;
            if self.cursors_need_sync(select) {
                self.current.column = new_start_column;
                self.current.line = new_start_line;
                self.current.column_offset = new_start_column;
            }
        }
        pub fn right(&mut self, select: bool) {
            let moving_cursor = self.get_moving_cursor(select);
            let current_line_max_column = self.lines[moving_cursor.line].chars().count();
            let has_next_line = moving_cursor.line < self.lines.len() - 1;
            let (new_end_line, new_end_column) =
                if moving_cursor.column == current_line_max_column && has_next_line {
                    (moving_cursor.line + 1, 0)
                } else if moving_cursor.column < current_line_max_column {
                    (moving_cursor.line, moving_cursor.column + 1)
                } else {
                    (moving_cursor.line, moving_cursor.column)
                };
            self.extender.line = new_end_line;
            self.extender.column = new_end_column;
            self.extender.column_offset = new_end_column;
            if self.cursors_need_sync(select) {
                self.current.line = new_end_line;
                self.current.column = new_end_column;
                self.current.column_offset = new_end_column;
            }
        }
        pub fn up(&mut self, select: bool) {
            self.vertical(-1, select);
        }
        pub fn down(&mut self, select: bool) {
            self.vertical(1, select);
        }

        fn vertical(&mut self, direction: isize, select: bool) {
            self.vertical_movement_line(direction, select);
            self.vertical_movement_column(select);
        }
        fn vertical_movement_line(&mut self, direction: isize, select: bool) {
            let moving_cursor = self.get_moving_cursor(select);

            let next_move_to_line = moving_cursor.line as isize + direction;
            let new_line = if next_move_to_line > (self.lines.len() - 1) as isize {
                self.lines.len() - 1
            } else if next_move_to_line <= 0 {
                0
            } else {
                ((moving_cursor.line as isize) + direction) as usize
            };
            self.extender.line = new_line;
            if self.cursors_need_sync(select) {
                self.current.line = new_line;
            }
        }

        fn vertical_movement_column(&mut self, select: bool) {
            let moving_cursor = self.get_moving_cursor(select);

            let next_line_max_column = self.lines[moving_cursor.line].chars().count();
            let new_column = if moving_cursor.column > next_line_max_column
                || &moving_cursor.column < &moving_cursor.column_offset
                    && moving_cursor.column < next_line_max_column
                    && moving_cursor.column_offset > next_line_max_column
            {
                next_line_max_column
            } else if &moving_cursor.column < &moving_cursor.column_offset
                && moving_cursor.column < next_line_max_column
            {
                moving_cursor.column_offset
            } else {
                moving_cursor.column
            };
            self.extender.column = new_column;
            if self.cursors_need_sync(select) {
                self.current.column = new_column;
            }
        }

        fn cursors_need_sync(&self, select: bool) -> bool {
            !select && self.current != self.extender
        }
        fn get_moving_cursor(&self, select: bool) -> &CursorPosition {
            if !select || self.current != self.extender {
                &self.extender
            } else {
                &self.current
            }
        }
    }
}
