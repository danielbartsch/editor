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
        pub start: CursorPosition,
        pub end: CursorPosition,
        pub line_lengths: Vec<usize>,
    }

    impl Cursor {
        pub fn new(line_lengths: Vec<usize>) -> Cursor {
            Cursor {
                start: CursorPosition {
                    line: 0,
                    column: 0,
                    column_offset: 0,
                },
                end: CursorPosition {
                    line: 0,
                    column: 0,
                    column_offset: 0,
                },
                line_lengths: line_lengths,
            }
        }
        pub fn add(&mut self) {
            if &self.start == &self.end {
                self.line_lengths[self.start.line] += 1;
                self.right(false);
            }
        }
        pub fn delete(&mut self) {
            if &self.start == &self.end {
                if &self.start.column == &self.line_lengths[self.start.line] {
                    if self.start.line + 1 < self.line_lengths.len() {
                        self.line_lengths[self.start.line] +=
                            self.line_lengths[self.start.line + 1];
                        self.line_lengths.remove(self.start.line + 1);
                    }
                } else {
                    self.line_lengths[self.start.line] -= 1;
                }
            }
        }
        pub fn backspace(&mut self) {
            if &self.start == &self.end {
                if self.start.column != 0 || self.start.line != 0 {
                    self.left(false);
                    self.delete();
                }
            }
        }
        pub fn new_line(&mut self) {
            if &self.start == &self.end {
                let remaining_length = self.line_lengths[self.start.line] - self.start.column;
                self.line_lengths[self.start.line] -= remaining_length;
                self.line_lengths
                    .insert(self.start.line + 1, remaining_length);
                self.start.line += 1;
                self.end.line += 1;
                self.start.column = 0;
                self.end.column = 0;
            }
        }
        pub fn home(&mut self, select: bool) {
            if self.start.column == 0
                && self.line_lengths[self.start.line] >= self.start.column_offset
            {
                self.start.column = self.start.column_offset;
                if !select {
                    self.end.column = self.end.column_offset;
                }
            } else {
                self.start.column = 0;
                if !select {
                    self.end.column = 0;
                }
            }
        }
        pub fn end(&mut self, select: bool) {
            let current_line_max_column = self.line_lengths[self.end.line];
            if self.end.column == current_line_max_column
                && current_line_max_column >= self.end.column_offset
            {
                if !select {
                    self.start.column = self.start.column_offset;
                }
                self.end.column = self.end.column_offset;
            } else {
                if !select {
                    self.start.column = current_line_max_column;
                }
                self.end.column = current_line_max_column;
            }
        }
        pub fn left(&mut self, select: bool) {
            let move_end = !select;
            let (new_start_line, new_start_column) = if self.start.column == 0 {
                if self.start.line != 0 {
                    let previous_line = self.start.line - 1;
                    (
                        previous_line,
                        *self.line_lengths.get(previous_line).unwrap(),
                    )
                } else {
                    (self.start.line, self.start.column)
                }
            } else {
                (self.start.line, self.start.column - 1)
            };
            self.start.column = new_start_column;
            self.start.line = new_start_line;
            self.start.column_offset = self.start.column;
            if move_end {
                self.end.column = new_start_column;
                self.end.line = new_start_line;
                self.end.column_offset = self.end.column;
            }
        }
        pub fn right(&mut self, select: bool) {
            let move_start = !select;
            let current_line_max_column = self.line_lengths.get(self.end.line).unwrap();
            let has_next_line = self.end.line < self.line_lengths.len() - 1;
            let (new_end_line, new_end_column) =
                if &self.end.column == current_line_max_column && has_next_line {
                    (self.end.line + 1, 0)
                } else if &self.end.column < current_line_max_column {
                    (self.end.line, self.end.column + 1)
                } else {
                    (self.end.line, self.end.column)
                };
            if move_start {
                self.start.line = new_end_line;
                self.start.column = new_end_column;
                self.start.column_offset = new_end_column;
            }
            self.end.line = new_end_line;
            self.end.column = new_end_column;
            self.end.column_offset = new_end_column;
        }
        pub fn up(&mut self, select: bool) {
            self.vertical(-1, select);
        }
        pub fn down(&mut self, select: bool) {
            self.vertical(1, select);
        }

        fn vertical(&mut self, direction: isize, select: bool) {
            let move_start = !select || direction < 0;
            let move_end = !select || direction > 0;
            self.vertical_movement_line(direction, select, move_start, move_end);
            self.vertical_movement_column(direction, select, move_start, move_end);
        }
        fn vertical_movement_line(
            &mut self,
            direction: isize,
            select: bool,
            move_start: bool,
            move_end: bool,
        ) {
            let next_move_to_line = if direction < 0 {
                self.start.line as isize
            } else {
                self.end.line as isize
            } + direction;
            let (new_start_line, new_end_line) =
                if next_move_to_line > (self.line_lengths.len() - 1) as isize {
                    (self.line_lengths.len() - 1, self.line_lengths.len() - 1)
                } else if next_move_to_line <= 0 {
                    (0, 0)
                } else if !select && self.start.line != self.end.line {
                    (next_move_to_line as usize, next_move_to_line as usize)
                } else {
                    (
                        ((self.start.line as isize) + direction) as usize,
                        ((self.end.line as isize) + direction) as usize,
                    )
                };
            if move_start {
                self.start.line = new_start_line;
            }
            if move_end {
                self.end.line = new_end_line;
            }
        }

        fn vertical_movement_column(
            &mut self,
            direction: isize,
            select: bool,
            move_start: bool,
            move_end: bool,
        ) {
            let moving_cursor_part = if direction < 0 {
                &self.start
            } else {
                &self.end
            };
            let next_line_max_column = self.line_lengths.get(moving_cursor_part.line).unwrap();
            let (new_start_column, new_end_column) =
                if !select && self.start.column != self.end.column {
                    let actual_new_column = if &moving_cursor_part.column > next_line_max_column {
                        next_line_max_column
                    } else {
                        &moving_cursor_part.column
                    };
                    (*actual_new_column, *actual_new_column)
                } else if &self.start.column > next_line_max_column
                    || &self.start.column < &self.start.column_offset
                        && &self.start.column < next_line_max_column
                        && &self.start.column_offset > next_line_max_column
                {
                    (*next_line_max_column, *next_line_max_column)
                } else if &self.start.column < &self.start.column_offset
                    && &self.start.column < next_line_max_column
                {
                    (self.start.column_offset, self.end.column_offset)
                } else {
                    ((self.start.column, self.end.column))
                };
            if move_start {
                self.start.column = new_start_column;
            }
            if move_end {
                self.end.column = new_end_column;
            }
        }
    }
}
