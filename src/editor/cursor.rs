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
                        self.start.column = self.line_lengths.get(self.start.line).unwrap() - 1;
                        self.end.column = self.line_lengths.get(self.end.line).unwrap() - 1;
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
                let current_line_max_column = self.line_lengths.get(self.start.line).unwrap() - 1;
                let has_next_line = self.start.line < self.line_lengths.len() - 1;
                if &self.start.column == &current_line_max_column && has_next_line {
                    self.start.line += 1;
                    self.end.line += 1;
                    self.start.column = 0;
                    self.end.column = 0;
                } else if &self.start.column < &current_line_max_column {
                    self.start.column += 1;
                    self.end.column += 1;
                }
                self.start.column_offset = self.start.column;
                self.end.column_offset = self.end.column;
            }
        }
        pub fn up(&mut self) {
            if &self.start == &self.end {
                if &self.start.line > &0 {
                    self.start.line -= 1;
                    self.end.line -= 1;
                    let upper_line_max_column = self.line_lengths.get(self.start.line).unwrap() - 1;
                    if &self.start.column != &self.start.column_offset
                        && &self.start.column <= &upper_line_max_column
                    {
                        self.start.column = self.start.column_offset;
                        self.end.column = self.end.column_offset;
                    } else if &self.start.column > &upper_line_max_column {
                        self.start.column = upper_line_max_column;
                        self.end.column = upper_line_max_column;
                    }
                }
            }
        }
        pub fn down(&mut self) {
            if &self.start == &self.end {
                if &self.start.line < &(self.line_lengths.len() - 1) {
                    self.start.line += 1;
                    self.end.line += 1;
                    let lower_line_max_column = self.line_lengths.get(self.start.line).unwrap() - 1;
                    if &self.start.column != &self.start.column_offset
                        && &self.start.column <= &lower_line_max_column
                    {
                        self.start.column = self.start.column_offset;
                        self.end.column = self.end.column_offset;
                    } else if &self.start.column > &lower_line_max_column {
                        self.start.column = lower_line_max_column;
                        self.end.column = lower_line_max_column;
                    }
                }
            }
        }
    }

    #[test]
    fn left_1() {
        let mut cursor = Cursor::new(vec![1]);
        cursor.left();
        assert_eq!(cursor, Cursor)
    }
}
