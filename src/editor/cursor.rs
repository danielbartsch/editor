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
            if (self.start.line as isize + direction) as usize > self.line_lengths.len() - 1 {
                self.start.line = self.line_lengths.len() - 1;
                self.end.line = self.line_lengths.len() - 1;
            } else if self.start.line as isize + direction <= 0 {
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
    fn remembering_sideways_flat() {
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
    fn remembering_sideways_hilly() {
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
    fn staying_on_same_column() {
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
}
