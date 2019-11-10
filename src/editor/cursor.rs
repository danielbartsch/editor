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

    #[derive(Debug)]
    pub struct Cursor {
        pub current: CursorPosition,
        pub extender: CursorPosition,
        pub lines: Vec<Vec<char>>,
    }

    impl Cursor {
        pub fn new(lines: Vec<Vec<char>>) -> Cursor {
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
        pub fn add(&mut self, character: char) {
            if &self.current == &self.extender {
                self.lines[self.current.line].insert(self.current.column, character);
                self.right(false);
            }
        }
        pub fn delete(&mut self) {
            if &self.current == &self.extender {
                if &self.current.column == &self.lines[self.current.line].len() {
                    if self.current.line + 1 < self.lines.len() {
                        for character in self.lines[self.current.line + 1].clone().iter() {
                            self.lines[self.current.line].push(*character);
                        }
                        self.lines.remove(self.current.line + 1);
                    }
                } else {
                    self.lines[self.current.line].remove(self.current.column);
                }
            }
        }
        pub fn backspace(&mut self) {
            if &self.current == &self.extender {
                if self.current.column != 0 || self.current.line != 0 {
                    self.left(false);
                    self.delete();
                }
            }
        }
        pub fn new_line(&mut self) {
            if &self.current == &self.extender {
                let (remaining_current_line, new_next_line) =
                    self.lines[self.current.line].split_at_mut(self.current.column);
                let (remaining_current_line_vec, new_next_line_vec) =
                    (remaining_current_line.to_vec(), new_next_line.to_vec());

                self.lines
                    .insert(self.current.line, remaining_current_line_vec);
                self.lines.insert(self.current.line + 2, new_next_line_vec);
                self.lines.remove(self.current.line + 1);

                self.current.line += 1;
                self.extender.line += 1;
                self.current.column = 0;
                self.extender.column = 0;
            }
        }
        pub fn home(&mut self, select: bool) {
            let moving_cursor = self.get_moving_cursor(select);
            let move_to = if moving_cursor.column == 0
                && self.lines[moving_cursor.line].len() >= moving_cursor.column_offset
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
            let current_line_max_column = self.lines[moving_cursor.line].len();
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
                    (previous_line, self.lines[previous_line].len())
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
            let current_line_max_column = self.lines[moving_cursor.line].len();
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

            let next_line_max_column = self.lines[moving_cursor.line].len();
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
