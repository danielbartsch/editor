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
        pub line_lengths: Vec<usize>,
    }

    impl Cursor {
        pub fn new(line_lengths: Vec<usize>) -> Cursor {
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
                line_lengths: line_lengths,
            }
        }
        pub fn add(&mut self) {
            if &self.current == &self.extender {
                self.line_lengths[self.current.line] += 1;
                self.right(false);
            }
        }
        pub fn delete(&mut self) {
            if &self.current == &self.extender {
                if &self.current.column == &self.line_lengths[self.current.line] {
                    if self.current.line + 1 < self.line_lengths.len() {
                        self.line_lengths[self.current.line] +=
                            self.line_lengths[self.current.line + 1];
                        self.line_lengths.remove(self.current.line + 1);
                    }
                } else {
                    self.line_lengths[self.current.line] -= 1;
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
                let remaining_length = self.line_lengths[self.current.line] - self.current.column;
                self.line_lengths[self.current.line] -= remaining_length;
                self.line_lengths
                    .insert(self.current.line + 1, remaining_length);
                self.current.line += 1;
                self.extender.line += 1;
                self.current.column = 0;
                self.extender.column = 0;
            }
        }
        pub fn home(&mut self, select: bool) {
            if self.current.column == 0
                && self.line_lengths[self.current.line] >= self.current.column_offset
            {
                self.current.column = self.current.column_offset;
                if !select {
                    self.extender.column = self.extender.column_offset;
                }
            } else {
                self.current.column = 0;
                if !select {
                    self.extender.column = 0;
                }
            }
        }
        pub fn end(&mut self, select: bool) {
            let current_line_max_column = self.line_lengths[self.extender.line];
            if self.extender.column == current_line_max_column
                && current_line_max_column >= self.extender.column_offset
            {
                if !select {
                    self.current.column = self.current.column_offset;
                }
                self.extender.column = self.extender.column_offset;
            } else {
                if !select {
                    self.current.column = current_line_max_column;
                }
                self.extender.column = current_line_max_column;
            }
        }
        pub fn left(&mut self, select: bool) {
            let move_end = !select;
            let (new_start_line, new_start_column) = if self.current.column == 0 {
                if self.current.line != 0 {
                    let previous_line = self.current.line - 1;
                    (
                        previous_line,
                        *self.line_lengths.get(previous_line).unwrap(),
                    )
                } else {
                    (self.current.line, self.current.column)
                }
            } else {
                (self.current.line, self.current.column - 1)
            };
            self.current.column = new_start_column;
            self.current.line = new_start_line;
            self.current.column_offset = self.current.column;
            if move_end {
                self.extender.column = new_start_column;
                self.extender.line = new_start_line;
                self.extender.column_offset = self.extender.column;
            }
        }
        pub fn right(&mut self, select: bool) {
            let move_start = !select;
            let current_line_max_column = self.line_lengths.get(self.extender.line).unwrap();
            let has_next_line = self.extender.line < self.line_lengths.len() - 1;
            let (new_end_line, new_end_column) =
                if &self.extender.column == current_line_max_column && has_next_line {
                    (self.extender.line + 1, 0)
                } else if &self.extender.column < current_line_max_column {
                    (self.extender.line, self.extender.column + 1)
                } else {
                    (self.extender.line, self.extender.column)
                };
            if move_start {
                self.current.line = new_end_line;
                self.current.column = new_end_column;
                self.current.column_offset = new_end_column;
            }
            self.extender.line = new_end_line;
            self.extender.column = new_end_column;
            self.extender.column_offset = new_end_column;
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
                self.current.line as isize
            } else {
                self.extender.line as isize
            } + direction;
            let (new_start_line, new_end_line) =
                if next_move_to_line > (self.line_lengths.len() - 1) as isize {
                    (self.line_lengths.len() - 1, self.line_lengths.len() - 1)
                } else if next_move_to_line <= 0 {
                    (0, 0)
                } else if !select && self.current.line != self.extender.line {
                    (next_move_to_line as usize, next_move_to_line as usize)
                } else {
                    (
                        ((self.current.line as isize) + direction) as usize,
                        ((self.extender.line as isize) + direction) as usize,
                    )
                };
            if move_start {
                self.current.line = new_start_line;
            }
            if move_end {
                self.extender.line = new_end_line;
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
                &self.current
            } else {
                &self.extender
            };
            let next_line_max_column = self.line_lengths.get(moving_cursor_part.line).unwrap();
            let (new_start_column, new_end_column) =
                if !select && self.current.column != self.extender.column {
                    let actual_new_column = if &moving_cursor_part.column > next_line_max_column {
                        next_line_max_column
                    } else {
                        &moving_cursor_part.column
                    };
                    (*actual_new_column, *actual_new_column)
                } else if &self.current.column > next_line_max_column
                    || &self.current.column < &self.current.column_offset
                        && &self.current.column < next_line_max_column
                        && &self.current.column_offset > next_line_max_column
                {
                    (*next_line_max_column, *next_line_max_column)
                } else if &self.current.column < &self.current.column_offset
                    && &self.current.column < next_line_max_column
                {
                    (self.current.column_offset, self.extender.column_offset)
                } else {
                    ((self.current.column, self.extender.column))
                };
            if move_start {
                self.current.column = new_start_column;
            }
            if move_end {
                self.extender.column = new_end_column;
            }
        }
    }
}
