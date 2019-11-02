#[path = "cursor.rs"]
mod cursor;

#[cfg(test)]
mod tests {
    use super::cursor::cursor::Cursor;

    #[test]
    fn right() {
        let mut empty = Cursor::new(vec![0]);
        empty.right(false);
        assert_eq!(empty.start.column, 0);

        let mut cursor = Cursor::new(vec![3, 0, 2]);
        cursor.right(false);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.start.column, 3);
        cursor.right(false);
        assert_eq!(cursor.start.column, 0);
        assert_eq!(cursor.start.line, 1);
        cursor.right(false);
        assert_eq!(cursor.start.column, 0);
        assert_eq!(cursor.start.line, 2);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.start.column, 2);
        cursor.right(false);
        assert_eq!(cursor.start.column, 2);
    }

    #[test]
    fn right_select() {
        let mut cursor = Cursor::new(vec![2, 5]);
        cursor.right(true);
        cursor.right(true);
        assert_eq!(cursor.start.column, 0, "start column");
        assert_eq!(cursor.end.column, 2, "end column");
        cursor.right(true);
        assert_eq!(cursor.start.column, 0, "new line start column");
        assert_eq!(cursor.end.column, 0, "new line end column");
        assert_eq!(cursor.start.line, 0, "new line start line");
        assert_eq!(cursor.end.line, 1, "new line end line");
        cursor.right(false);
        assert_eq!(cursor.start.column, 1, "line cursor start column");
        assert_eq!(cursor.end.column, 1, "line cursor end column");
        assert_eq!(cursor.start.line, 1, "line cursor start line");
        assert_eq!(cursor.end.line, 1, "line cursor end line");
        cursor.right(true);
        cursor.right(true);
        cursor.right(false);
        assert_eq!(cursor.start.column, 4, "line cursor start column");
        assert_eq!(cursor.end.column, 4, "line cursor end column");
        assert_eq!(cursor.start.line, 1, "line cursor start line");
        assert_eq!(cursor.end.line, 1, "line cursor end line");
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
        cursor.down(false);
        cursor.down(false);
        cursor.right(false);
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
    fn up_select() {
        let mut cursor = Cursor::new(vec![1, 5, 5, 5]);
        cursor.down(false);
        cursor.down(false);
        cursor.down(false);
        cursor.up(true);
        assert_eq!(cursor.start.line, 2, "select start.line");
        assert_eq!(cursor.end.line, 3, "select end.line");
        cursor.up(false);
        assert_eq!(cursor.start.line, 1, "move start.line");
        assert_eq!(cursor.end.line, 1, "move end.line");
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.start.column, 2);
        assert_eq!(cursor.end.column, 2);
        cursor.up(true);
        assert_eq!(cursor.start.column, 1);
        assert_eq!(cursor.end.column, 2);
        assert_eq!(cursor.start.line, 0, "move start.line");
        assert_eq!(cursor.end.line, 1, "move end.line");
    }

    #[test]
    fn down_select() {
        let mut cursor = Cursor::new(vec![5, 5, 5, 1]);
        cursor.down(true);
        assert_eq!(cursor.start.line, 0, "select start.line");
        assert_eq!(cursor.end.line, 1, "select end.line");
        cursor.down(false);
        assert_eq!(cursor.start.line, 2, "move start.line");
        assert_eq!(cursor.end.line, 2, "move end.line");
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.start.column, 2);
        assert_eq!(cursor.end.column, 2);
        cursor.down(true);
        assert_eq!(cursor.start.column, 2, "left behind column");
        assert_eq!(cursor.end.column, 1, "next column");
        assert_eq!(cursor.start.line, 2, "left behind line");
        assert_eq!(cursor.end.line, 3, "next line");
    }

    #[test]
    fn down_up_remembering_sideways_flat() {
        let mut cursor = Cursor::new(vec![3, 1, 1]);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.start.column, 2, "beginning");
        cursor.down(false);
        assert_eq!(cursor.start.column, 1, "first");
        cursor.down(false);
        assert_eq!(cursor.start.column, 1, "second");
        cursor.up(false);
        assert_eq!(cursor.start.column, 1, "third");
        cursor.up(false);
        assert_eq!(cursor.start.column, 2, "fourth");
    }

    #[test]
    fn down_up_remembering_sideways_hilly() {
        let mut cursor = Cursor::new(vec![5, 1, 2]);
        cursor.right(false);
        cursor.right(false);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.start.column, 4, "beginning");
        cursor.down(false);
        assert_eq!(cursor.start.column, 1, "first");
        cursor.down(false);
        assert_eq!(cursor.start.column, 2, "second");
        cursor.up(false);
        assert_eq!(cursor.start.column, 1, "third");
        cursor.up(false);
        assert_eq!(cursor.start.column, 4, "fourth");
    }

    #[test]
    fn down_up_staying_on_same_column() {
        let mut cursor = Cursor::new(vec![5, 5, 5]);
        cursor.right(false);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.start.column, 3, "beginning");
        cursor.down(false);
        assert_eq!(cursor.start.column, 3, "first");
        cursor.down(false);
        assert_eq!(cursor.start.column, 3, "second");
        cursor.up(false);
        assert_eq!(cursor.start.column, 3, "third");
        cursor.up(false);
        assert_eq!(cursor.start.column, 3, "fourth");
    }

    #[test]
    fn wrap_around() {
        let mut cursor = Cursor::new(vec![1, 1]);
        cursor.down(false);
        assert_eq!(cursor.start.line, 1, "last line");
        cursor.down(false);
        assert_eq!(cursor.start.line, 1, "should stay on last line");
        cursor.up(false);
        assert_eq!(cursor.start.line, 0, "first line");
        cursor.up(false);
        assert_eq!(cursor.start.line, 0, "should stay on first line");
    }

    #[test]
    fn delete() {
        let mut cursor = Cursor::new(vec![2, 2, 2]);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.start.column, 2);
        cursor.delete();
        assert_eq!(cursor.start.column, 2);
        assert_eq!(cursor.line_lengths, vec![4, 2]);
        cursor.down(false);
        cursor.delete();
        assert_eq!(cursor.line_lengths, vec![4, 2]);
    }

    #[test]
    fn backspace() {
        let mut cursor = Cursor::new(vec![2, 2, 2]);
        cursor.down(false);
        cursor.right(false);
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
        cursor.right(false);
        cursor.right(false);
        cursor.right(false);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.start.column, 5, "initial column");
        assert_eq!(cursor.start.line, 0, "initial line");
        cursor.new_line();
        assert_eq!(cursor.start.column, 0, "column after new line");
        assert_eq!(cursor.start.line, 1, "line after new line");
        assert_eq!(cursor.line_lengths, vec![5, 0, 10]);
        cursor.down(false);
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
        cursor.right(false);
        cursor.right(false);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.start.column, 4, "initial column");
        cursor.home(false);
        assert_eq!(cursor.start.column, 0, "home column");
        cursor.home(false);
        assert_eq!(
            cursor.start.column, 4,
            "home again: go back to where you were"
        );
        cursor.down(false);
        assert_eq!(cursor.start.column, 0);
        cursor.home(false);
        assert_eq!(cursor.start.column, 0, "empty line: stay there");
        cursor.home(false);
        assert_eq!(
            cursor.start.column, 0,
            "stay there even after twice home-ing"
        );
    }

    #[test]
    fn home_select() {
        let mut cursor = Cursor::new(vec![10, 0, 4]);
        cursor.right(false);
        cursor.right(false);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.start.column, 4, "initial start column");
        assert_eq!(cursor.end.column, 4, "initial end column");
        cursor.home(true);
        assert_eq!(cursor.start.column, 0, "start home column");
        assert_eq!(cursor.end.column, 4, "same end column");
        cursor.home(true);
        assert_eq!(
            cursor.start.column, 4,
            "home again: go back to where you were"
        );
        assert_eq!(
            cursor.end.column, 4,
            "home again: end column still the same"
        );
        cursor.down(false);
        assert_eq!(cursor.start.column, 0);
        cursor.home(true);
        assert_eq!(cursor.start.column, 0, "empty line: stay there");
        cursor.home(true);
        assert_eq!(
            cursor.start.column, 0,
            "stay there even after twice home-ing"
        );
        assert_eq!(cursor.end.column, 0, "stay there even after twice home-ing");
    }

    #[test]
    fn end() {
        let mut cursor = Cursor::new(vec![10, 0, 4]);
        cursor.right(false);
        cursor.right(false);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.start.column, 4, "initial column");
        cursor.end(false);
        assert_eq!(cursor.start.column, 10, "end column");
        cursor.end(false);
        assert_eq!(
            cursor.start.column, 4,
            "end again: go back to where you were"
        );
        cursor.down(false);
        assert_eq!(cursor.start.column, 0);
        cursor.end(false);
        assert_eq!(cursor.start.column, 0, "empty line: stay there");
        cursor.end(false);
        assert_eq!(
            cursor.start.column, 0,
            "stay there even after twice end-ing"
        );
    }

    #[test]
    fn end_select() {
        let mut cursor = Cursor::new(vec![10, 0, 4]);
        cursor.right(false);
        cursor.right(false);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.start.column, 4, "initial start column");
        assert_eq!(cursor.end.column, 4, "initial end column");
        cursor.end(true);
        assert_eq!(cursor.start.column, 4, "end column");
        assert_eq!(cursor.end.column, 10, "end column");
        cursor.end(true);
        assert_eq!(cursor.end.column, 4, "end again: go back to where you were");
        assert_eq!(cursor.start.column, 4, "end again: start is still the same");
        cursor.down(false);
        assert_eq!(cursor.start.column, 0);
        assert_eq!(cursor.end.column, 0);
        cursor.end(true);
        assert_eq!(cursor.start.column, 0, "empty line: stay there");
        assert_eq!(cursor.end.column, 0, "empty line: stay there");
        cursor.end(true);
        assert_eq!(
            cursor.start.column, 0,
            "stay there even after twice end-ing"
        );
        assert_eq!(cursor.end.column, 0, "stay there even after twice end-ing");
    }
}
