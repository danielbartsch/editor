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
                self.right();
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
                    self.left();
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
        pub fn home(&mut self) {
            if &self.start == &self.end {
                if self.start.column == 0
                    && self.line_lengths[self.start.line] >= self.start.column_offset
                {
                    self.start.column = self.start.column_offset;
                    self.end.column = self.end.column_offset;
                } else {
                    self.start.column = 0;
                    self.end.column = 0;
                }
            }
        }
        pub fn end(&mut self) {
            if &self.start == &self.end {
                let current_line_max_column = self.line_lengths[self.start.line];
                if self.start.column == current_line_max_column
                    && current_line_max_column >= self.start.column_offset
                {
                    self.start.column = self.start.column_offset;
                    self.end.column = self.end.column_offset;
                } else {
                    self.start.column = current_line_max_column;
                    self.end.column = current_line_max_column;
                }
            }
        }
        pub fn left(&mut self) {
            if &self.start == &self.end {
                if &self.start.column == &0 {
                    if &self.start.line != &0 {
                        self.start.line -= 1;
                        self.end.line -= 1;
                        self.start.column = *self.line_lengths.get(self.start.line).unwrap();
                        self.end.column = *self.line_lengths.get(self.end.line).unwrap();
                    }
                } else {
                    self.start.column -= 1;
                    self.end.column -= 1;
                }
                self.start.column_offset = self.start.column;
                self.end.column_offset = self.end.column;
            }
        }
        pub fn right(&mut self) {
            if &self.start == &self.end {
                let current_line_max_column = self.line_lengths.get(self.start.line).unwrap();
                let has_next_line = self.start.line < self.line_lengths.len() - 1;
                if &self.start.column == current_line_max_column && has_next_line {
                    self.start.line += 1;
                    self.end.line += 1;
                    self.start.column = 0;
                    self.end.column = 0;
                } else if &self.start.column < current_line_max_column {
                    self.start.column += 1;
                    self.end.column += 1;
                }
                self.start.column_offset = self.start.column;
                self.end.column_offset = self.end.column;
            }
        }
        pub fn up(&mut self) {
            self.vertical(-1);
        }
        pub fn down(&mut self) {
            self.vertical(1);
        }

        fn vertical(&mut self, direction: isize) {
            if &self.start == &self.end {
                self.vertical_movement_line(direction);
                self.vertical_movement_column();
            }
        }
        fn vertical_movement_line(&mut self, direction: isize) {
            let next_line = self.start.line as isize + direction;
            if next_line > (self.line_lengths.len() - 1) as isize {
                self.start.line = self.line_lengths.len() - 1;
                self.end.line = self.line_lengths.len() - 1;
            } else if next_line <= 0 {
                self.start.line = 0;
                self.end.line = 0;
            } else {
                self.start.line = ((self.start.line as isize) + direction) as usize;
                self.end.line = ((self.end.line as isize) + direction) as usize;
            }
        }

        fn vertical_movement_column(&mut self) {
            let next_line_max_column = self.line_lengths.get(self.start.line).unwrap();
            if &self.start.column > next_line_max_column
                || &self.start.column < &self.start.column_offset
                    && &self.start.column < next_line_max_column
                    && &self.start.column_offset > next_line_max_column
            {
                self.start.column = *next_line_max_column;
                self.end.column = *next_line_max_column;
            } else if &self.start.column < &self.start.column_offset
                && &self.start.column < next_line_max_column
            {
                self.start.column = self.start.column_offset;
                self.end.column = self.end.column_offset;
            }
        }
    }

    #[test]
    fn right() {
        let mut empty = Cursor::new(vec![0]);
        empty.right();
        assert_eq!(empty.start.column, 0);

        let mut cursor = Cursor::new(vec![3, 0, 2]);
        cursor.right();
        cursor.right();
        cursor.right();
        assert_eq!(cursor.start.column, 3);
        cursor.right();
        assert_eq!(cursor.start.column, 0);
        assert_eq!(cursor.start.line, 1);
        cursor.right();
        assert_eq!(cursor.start.column, 0);
        assert_eq!(cursor.start.line, 2);
        cursor.right();
        cursor.right();
        assert_eq!(cursor.start.column, 2);
        cursor.right();
        assert_eq!(cursor.start.column, 2);
    }

    #[test]
    fn left() {
        let mut empty = Cursor::new(vec![0]);
        empty.left();
        assert_eq!(empty.start.column, 0);

        let mut cursor = Cursor::new(vec![3, 0, 2]);
        assert_eq!(cursor.start.column, 0);
        cursor.left();
        assert_eq!(cursor.start.column, 0);
        cursor.down();
        cursor.down();
        cursor.right();
        assert_eq!(cursor.start.column, 1);
        assert_eq!(cursor.start.line, 2);
        cursor.left();
        assert_eq!(cursor.start.column, 0);
        cursor.left();
        assert_eq!(cursor.start.column, 0);
        cursor.left();
        assert_eq!(cursor.start.column, 3);
    }

    #[test]
    fn down_up_remembering_sideways_flat() {
        let mut cursor = Cursor::new(vec![3, 1, 1]);
        cursor.right();
        cursor.right();
        assert_eq!(cursor.start.column, 2, "beginning");
        cursor.down();
        assert_eq!(cursor.start.column, 1, "first");
        cursor.down();
        assert_eq!(cursor.start.column, 1, "second");
        cursor.up();
        assert_eq!(cursor.start.column, 1, "third");
        cursor.up();
        assert_eq!(cursor.start.column, 2, "fourth");
    }

    #[test]
    fn down_up_remembering_sideways_hilly() {
        let mut cursor = Cursor::new(vec![5, 1, 2]);
        cursor.right();
        cursor.right();
        cursor.right();
        cursor.right();
        assert_eq!(cursor.start.column, 4, "beginning");
        cursor.down();
        assert_eq!(cursor.start.column, 1, "first");
        cursor.down();
        assert_eq!(cursor.start.column, 2, "second");
        cursor.up();
        assert_eq!(cursor.start.column, 1, "third");
        cursor.up();
        assert_eq!(cursor.start.column, 4, "fourth");
    }

    #[test]
    fn down_up_staying_on_same_column() {
        let mut cursor = Cursor::new(vec![5, 5, 5]);
        cursor.right();
        cursor.right();
        cursor.right();
        assert_eq!(cursor.start.column, 3, "beginning");
        cursor.down();
        assert_eq!(cursor.start.column, 3, "first");
        cursor.down();
        assert_eq!(cursor.start.column, 3, "second");
        cursor.up();
        assert_eq!(cursor.start.column, 3, "third");
        cursor.up();
        assert_eq!(cursor.start.column, 3, "fourth");
    }

    #[test]
    fn wrap_around() {
        let mut cursor = Cursor::new(vec![1, 1]);
        cursor.down();
        assert_eq!(cursor.start.line, 1, "last line");
        cursor.down();
        assert_eq!(cursor.start.line, 1, "should stay on last line");
        cursor.up();
        assert_eq!(cursor.start.line, 0, "first line");
        cursor.up();
        assert_eq!(cursor.start.line, 0, "should stay on first line");
    }

    #[test]
    fn delete() {
        let mut cursor = Cursor::new(vec![2, 2, 2]);
        cursor.right();
        cursor.right();
        assert_eq!(cursor.start.column, 2);
        cursor.delete();
        assert_eq!(cursor.start.column, 2);
        assert_eq!(cursor.line_lengths, vec![4, 2]);
        cursor.down();
        cursor.delete();
        assert_eq!(cursor.line_lengths, vec![4, 2]);
    }

    #[test]
    fn backspace() {
        let mut cursor = Cursor::new(vec![2, 2, 2]);
        cursor.down();
        cursor.right();
        assert_eq!(cursor.start.column, 1);
        assert_eq!(cursor.start.line, 1);
        cursor.backspace();
        assert_eq!(cursor.start.column, 0);
        assert_eq!(cursor.line_lengths, vec![2, 1, 2]);
        cursor.backspace();
        assert_eq!(cursor.start.line, 0);
        assert_eq!(cursor.start.column, 2);
        assert_eq!(cursor.line_lengths, vec![3, 2]);
    }

    #[test]
    fn add() {
        let mut cursor = Cursor::new(vec![5]);
        assert_eq!(cursor.start.column, 0, "initial column");
        cursor.add();
        assert_eq!(cursor.start.column, 1, "column after adding");
        assert_eq!(cursor.line_lengths, vec![6], "line lengths after adding");

        let mut empty = Cursor::new(vec![0]);
        empty.add();
        assert_eq!(empty.line_lengths, vec![1]);
    }

    #[test]
    fn new_line() {
        let mut cursor = Cursor::new(vec![5, 10]);
        cursor.right();
        cursor.right();
        cursor.right();
        cursor.right();
        cursor.right();
        assert_eq!(cursor.start.column, 5, "initial column");
        assert_eq!(cursor.start.line, 0, "initial line");
        cursor.new_line();
        assert_eq!(cursor.start.column, 0, "column after new line");
        assert_eq!(cursor.start.line, 1, "line after new line");
        assert_eq!(cursor.line_lengths, vec![5, 0, 10]);
        cursor.down();
        assert_eq!(cursor.start.column, 5, "column after coming from new line");
        cursor.new_line();
        assert_eq!(
            cursor.start.column, 0,
            "column after new line from line split"
        );
        assert_eq!(cursor.line_lengths, vec![5, 0, 5, 5]);
    }

    #[test]
    fn home() {
        let mut cursor = Cursor::new(vec![10, 0, 4]);
        cursor.right();
        cursor.right();
        cursor.right();
        cursor.right();
        assert_eq!(cursor.start.column, 4, "initial column");
        cursor.home();
        assert_eq!(cursor.start.column, 0, "home column");
        cursor.home();
        assert_eq!(
            cursor.start.column, 4,
            "home again: go back to where you were"
        );
        cursor.down();
        assert_eq!(cursor.start.column, 0);
        cursor.home();
        assert_eq!(cursor.start.column, 0, "empty line: stay there");
        cursor.home();
        assert_eq!(
            cursor.start.column, 0,
            "stay there even after twice home-ing"
        );
    }

    #[test]
    fn end() {
        let mut cursor = Cursor::new(vec![10, 0, 4]);
        cursor.right();
        cursor.right();
        cursor.right();
        cursor.right();
        assert_eq!(cursor.start.column, 4, "initial column");
        cursor.end();
        assert_eq!(cursor.start.column, 10, "end column");
        cursor.end();
        assert_eq!(
            cursor.start.column, 4,
            "end again: go back to where you were"
        );
        cursor.down();
        assert_eq!(cursor.start.column, 0);
        cursor.end();
        assert_eq!(cursor.start.column, 0, "empty line: stay there");
        cursor.end();
        assert_eq!(
            cursor.start.column, 0,
            "stay there even after twice end-ing"
        );
    }
}
