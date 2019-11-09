#[path = "cursor.rs"]
mod cursor;

#[cfg(test)]
mod tests {
    use super::cursor::cursor::Cursor;
    #[test]
    fn right_empty() {
        let mut empty = Cursor::new(vec![0]);
        empty.right(false);
        assert_eq!(empty.current.column, 0);
    }
    #[test]
    fn right_in_line() {
        let mut cursor = Cursor::new(vec![3]);
        cursor.right(false);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.current.column, 3);
    }
    #[test]
    fn right_next_line() {
        let mut cursor = Cursor::new(vec![1, 0]);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.current.line, 1);
    }
    #[test]
    fn right_empty_line_with_next_line() {
        let mut cursor = Cursor::new(vec![0, 2]);
        cursor.right(false);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.current.line, 1);
    }
    #[test]
    fn right_last_line() {
        let mut cursor = Cursor::new(vec![2]);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.current.column, 2);
        cursor.right(false);
        assert_eq!(cursor.current.column, 2);
    }
    #[test]
    fn right_select_empty() {
        let mut cursor = Cursor::new(vec![0]);
        cursor.right(true);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.extender.column, 0);
    }
    #[test]
    fn right_select_same_line() {
        let mut cursor = Cursor::new(vec![10]);
        cursor.right(true);
        cursor.right(true);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.extender.column, 2);
    }
    #[test]
    fn right_select_multiple_lines() {
        let mut cursor = Cursor::new(vec![1, 5]);
        cursor.right(true);
        cursor.right(true);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.extender.column, 0);
        assert_eq!(cursor.extender.line, 1);
    }
    #[test]
    fn right_from_selection_same_line() {
        let mut cursor = Cursor::new(vec![10]);
        cursor.right(true);
        cursor.right(true);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.extender.column, 2);
        cursor.right(false);
        assert_eq!(cursor.current.column, 3);
        assert_eq!(cursor.extender.column, 3);
    }
    #[test]
    fn right_from_selection_multiple_lines() {
        let mut cursor = Cursor::new(vec![3, 3]);
        cursor.right(false);
        cursor.right(true);
        cursor.down(true);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.extender.column, 2);
        assert_eq!(cursor.extender.line, 1);
        cursor.right(false);
        assert_eq!(cursor.current.column, 3);
        assert_eq!(cursor.current.line, 1);
        assert_eq!(cursor.extender.column, 3);
        assert_eq!(cursor.extender.line, 1);
    }
    #[test]
    fn left_empty() {
        let mut cursor = Cursor::new(vec![0]);
        cursor.left(false);
        assert_eq!(cursor.current.column, 0);
    }
    #[test]
    fn left_same_line() {
        let mut cursor = Cursor::new(vec![10]);
        cursor.right(false);
        assert_eq!(cursor.current.column, 1);
        cursor.left(false);
        assert_eq!(cursor.current.column, 0);
        cursor.left(false);
        assert_eq!(cursor.current.column, 0);
    }
    #[test]
    fn left_start_of_line() {
        let mut cursor = Cursor::new(vec![10, 1]);
        cursor.down(false);
        cursor.left(false);
        assert_eq!(cursor.current.column, 10);
    }
    #[test]
    fn left_start_of_line_empty_line() {
        let mut cursor = Cursor::new(vec![0, 5]);
        cursor.down(false);
        cursor.left(false);
        assert_eq!(cursor.current.column, 0);
    }
    #[test]
    fn left_select_empty() {
        let mut cursor = Cursor::new(vec![0]);
        cursor.left(true);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.extender.column, 0);
    }
    #[test]
    fn left_select_same_line() {
        let mut cursor = Cursor::new(vec![10]);
        cursor.right(false);
        cursor.right(false);
        cursor.left(true);
        assert_eq!(cursor.current.column, 2);
        assert_eq!(cursor.extender.column, 1);
    }
    #[test]
    fn left_select_multiple_lines() {
        let mut cursor = Cursor::new(vec![2, 0, 2]);
        cursor.down(false);
        cursor.down(false);
        cursor.right(false);
        cursor.left(true);
        cursor.left(true);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.current.line, 2);
        assert_eq!(cursor.extender.column, 0);
        assert_eq!(cursor.extender.line, 1);
        cursor.left(true);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.current.line, 2);
        assert_eq!(cursor.extender.column, 2);
        assert_eq!(cursor.extender.line, 0);
    }
    #[test]
    fn left_from_selection_same_line() {
        let mut cursor = Cursor::new(vec![10]);
        cursor.right(false);
        cursor.right(false);
        cursor.right(true);
        cursor.right(true);
        assert_eq!(cursor.current.column, 2);
        assert_eq!(cursor.extender.column, 4);
        cursor.left(false);
        assert_eq!(cursor.current.column, 3);
        assert_eq!(cursor.extender.column, 3);
    }
    #[test]
    fn left_from_selection_multiple_lines() {
        let mut cursor = Cursor::new(vec![1, 3]);
        cursor.right(false);
        cursor.right(true);
        cursor.right(true);
        cursor.left(false);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.current.line, 1);
        assert_eq!(cursor.extender.column, 0);
        assert_eq!(cursor.extender.line, 1);
    }
    #[test]
    fn up_select_empty() {
        let mut cursor = Cursor::new(vec![0]);
        cursor.up(true);
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.extender.line, 0);
    }
    #[test]
    fn up_select_same_line_lengths() {
        let mut cursor = Cursor::new(vec![2, 2]);
        cursor.down(false);
        cursor.right(false);
        cursor.up(true);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.current.line, 1);
        assert_eq!(cursor.extender.column, 1);
        assert_eq!(cursor.extender.line, 0);
    }
    #[test]
    fn up_select_different_line_lengths() {
        let mut cursor = Cursor::new(vec![1, 0, 1]);
        cursor.down(false);
        cursor.down(false);
        cursor.right(false);
        cursor.up(true);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.current.line, 2);
        assert_eq!(cursor.extender.column, 0);
        assert_eq!(cursor.extender.line, 1);
        cursor.up(true);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.current.line, 2);
        assert_eq!(cursor.extender.column, 1);
        assert_eq!(cursor.extender.line, 0);
    }
    #[test]
    fn up_from_selection_same_line() {
        let mut cursor = Cursor::new(vec![1, 3]);
        cursor.down(false);
        cursor.right(false);
        cursor.right(false);
        cursor.right(true);
        cursor.up(false);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.extender.column, 1);
        assert_eq!(cursor.extender.line, 0);
    }
    #[test]
    fn up_from_selection_different_line() {
        let mut cursor = Cursor::new(vec![1, 3, 3]);
        cursor.right(true);
        cursor.right(true);
        cursor.right(true);
        cursor.down(true);
        cursor.up(false);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.current.line, 1);
        assert_eq!(cursor.extender.column, 1);
        assert_eq!(cursor.extender.line, 1);
    }
    #[test]
    fn down_select_empty() {
        let mut cursor = Cursor::new(vec![0]);
        cursor.down(true);
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.extender.line, 0);
    }
    #[test]
    fn down_select_same_line_lengths() {
        let mut cursor = Cursor::new(vec![1, 1]);
        cursor.right(false);
        cursor.down(true);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.extender.column, 1);
        assert_eq!(cursor.extender.line, 1);
    }
    #[test]
    fn down_select_different_line_lengths() {
        let mut cursor = Cursor::new(vec![2, 0, 2]);
        cursor.right(false);
        cursor.right(false);
        cursor.down(true);
        assert_eq!(cursor.current.column, 2);
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.extender.column, 0);
        assert_eq!(cursor.extender.line, 1);
        cursor.down(true);
        assert_eq!(cursor.current.column, 2);
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.extender.column, 2);
        assert_eq!(cursor.extender.line, 2);
    }
    #[test]
    fn down_line_selection() {
        let mut cursor = Cursor::new(vec![5, 0, 5]);
        cursor.right(true);
        cursor.down(false);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.extender.column, 0);
        assert_eq!(cursor.current.line, 1);
        assert_eq!(cursor.extender.line, 1);
        cursor.down(false);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.extender.column, 1);
        assert_eq!(cursor.current.line, 2);
        assert_eq!(cursor.extender.line, 2);
    }
    #[test]
    fn down_from_selection_same_line() {
        let mut cursor = Cursor::new(vec![10, 1]);
        cursor.right(false);
        cursor.right(false);
        cursor.right(true);
        cursor.right(true);
        cursor.down(false);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.extender.column, 1);
        assert_eq!(cursor.current.line, 1);
        assert_eq!(cursor.extender.line, 1);
    }
    #[test]
    fn down_from_selection_different_line() {
        let mut cursor = Cursor::new(vec![3, 1, 3]);
        cursor.right(false);
        cursor.right(true);
        cursor.right(true);
        cursor.right(true);
        cursor.right(true);
        cursor.down(false);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.extender.column, 1);
        assert_eq!(cursor.current.line, 2);
        assert_eq!(cursor.extender.line, 2);
    }
    #[test]
    fn up_empty() {
        let mut cursor = Cursor::new(vec![0]);
        cursor.up(false);
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.current.column, 0);
    }
    #[test]
    fn up_in_first_line() {
        let mut cursor = Cursor::new(vec![10]);
        cursor.right(false);
        assert_eq!(cursor.current.column, 1);
        cursor.up(false);
        assert_eq!(cursor.current.column, 1);
    }
    #[test]
    fn up_same_line_length() {
        let mut cursor = Cursor::new(vec![1, 1]);
        cursor.right(false);
        cursor.down(false);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.current.line, 1);
        cursor.up(false);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.current.line, 0);
    }
    #[test]
    fn up_smaller_line_remember_column() {
        let mut cursor = Cursor::new(vec![2, 1, 2]);
        cursor.down(false);
        cursor.down(false);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.current.column, 2);
        cursor.up(false);
        assert_eq!(cursor.current.column, 1);
        cursor.up(false);
        assert_eq!(cursor.current.column, 2);
    }
    #[test]
    fn down_empty() {
        let mut cursor = Cursor::new(vec![0]);
        cursor.down(false);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.current.line, 0);
    }
    #[test]
    fn down_in_last_line() {
        let mut cursor = Cursor::new(vec![10]);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.current.column, 2);
        cursor.down(false);
        assert_eq!(cursor.current.column, 2);
    }
    #[test]
    fn down_same_line_length() {
        let mut cursor = Cursor::new(vec![2, 2]);
        cursor.right(false);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.current.line, 0);
        cursor.down(false);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.current.line, 1);
    }
    #[test]
    fn down_smaller_line_length() {
        let mut cursor = Cursor::new(vec![2, 1, 2]);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.current.column, 2);
        cursor.down(false);
        assert_eq!(cursor.current.column, 1);
        cursor.down(false);
        assert_eq!(cursor.current.column, 2);
    }
    #[test]
    fn delete_empty() {
        let mut cursor = Cursor::new(vec![0]);
        cursor.delete();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.line_lengths, vec![0]);
    }
    #[test]
    fn delete_in_line() {
        let mut cursor = Cursor::new(vec![10]);
        cursor.delete();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.line_lengths, vec![9]);
    }
    #[test]
    fn delete_empty_line() {
        let mut cursor = Cursor::new(vec![0, 10]);
        cursor.delete();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.line_lengths, vec![10]);
    }
    #[test]
    fn delete_at_end_of_line() {
        let mut cursor = Cursor::new(vec![2, 10]);
        cursor.right(false);
        cursor.right(false);
        cursor.delete();
        assert_eq!(cursor.current.column, 2);
        assert_eq!(cursor.line_lengths, vec![12]);
    }
    #[test]
    fn delete_at_end_of_last_line() {
        let mut cursor = Cursor::new(vec![2]);
        cursor.right(false);
        cursor.right(false);
        cursor.delete();
        assert_eq!(cursor.current.column, 2);
        assert_eq!(cursor.line_lengths, vec![2]);
    }
    #[test]
    fn backspace_empty() {
        let mut cursor = Cursor::new(vec![0]);
        cursor.backspace();
        assert_eq!(cursor.line_lengths, vec![0]);
    }
    #[test]
    fn backspace_in_line() {
        let mut cursor = Cursor::new(vec![2]);
        cursor.right(false);
        cursor.backspace();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.line_lengths, vec![1]);
    }
    #[test]
    fn backspace_delete_line() {
        let mut cursor = Cursor::new(vec![2, 0, 2]);
        cursor.down(false);
        cursor.backspace();
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.current.column, 2);
        assert_eq!(cursor.line_lengths, vec![2, 2]);
    }
    #[test]
    fn add_empty() {
        let mut cursor = Cursor::new(vec![0]);
        cursor.add();
        assert_eq!(cursor.line_lengths, vec![1]);
    }
    #[test]
    fn add() {
        let mut cursor = Cursor::new(vec![5]);
        cursor.add();
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.line_lengths, vec![6]);
    }
    #[test]
    fn new_line_empty() {
        let mut cursor = Cursor::new(vec![0]);
        cursor.new_line();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.current.line, 1);
        assert_eq!(cursor.line_lengths, vec![0, 0]);
    }
    #[test]
    fn new_line_end_of_line_no_line_after() {
        let mut cursor = Cursor::new(vec![2]);
        cursor.right(false);
        cursor.right(false);
        cursor.new_line();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.current.line, 1);
        assert_eq!(cursor.line_lengths, vec![2, 0]);
    }
    #[test]
    fn new_line_middle_of_line() {
        let mut cursor = Cursor::new(vec![20]);
        cursor.right(false);
        cursor.right(false);
        cursor.new_line();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.current.line, 1);
        assert_eq!(cursor.line_lengths, vec![2, 18]);
    }
    #[test]
    fn new_line_remember_column() {
        let mut cursor = Cursor::new(vec![10, 10]);
        cursor.right(false);
        cursor.right(false);
        cursor.new_line();
        assert_eq!(cursor.current.column, 0);
        cursor.down(false);
        assert_eq!(cursor.current.column, 2);
    }
    #[test]
    fn new_line_remember_column_line_smaller_than_remembered() {
        let mut cursor = Cursor::new(vec![10, 1]);
        cursor.right(false);
        cursor.right(false);
        cursor.new_line();
        assert_eq!(cursor.current.column, 0);
        cursor.down(false);
        assert_eq!(cursor.current.column, 1);
    }
    #[test]
    fn home_empty() {
        let mut cursor = Cursor::new(vec![0]);
        cursor.home(false);
        assert_eq!(cursor.current.column, 0);
    }
    #[test]
    fn home_line_start() {
        let mut cursor = Cursor::new(vec![10]);
        cursor.home(false);
        assert_eq!(cursor.current.column, 0);
    }
    #[test]
    fn home_middle_of_line() {
        let mut cursor = Cursor::new(vec![10]);
        cursor.right(false);
        cursor.right(false);
        cursor.home(false);
        assert_eq!(cursor.current.column, 0);
        cursor.home(false);
        assert_eq!(cursor.current.column, 2);
    }
    #[test]
    fn home_select_empty() {
        let mut cursor = Cursor::new(vec![0]);
        cursor.home(true);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.extender.column, 0);
    }
    #[test]
    fn home_select_in_same_line() {
        let mut cursor = Cursor::new(vec![10]);
        cursor.right(false);
        cursor.home(true);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.extender.column, 0);
        cursor.home(true);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.extender.column, 1);
    }
    #[test]
    fn home_select_with_multi_line_selection() {
        let mut cursor = Cursor::new(vec![5, 5]);
        cursor.right(false);
        cursor.right(false);
        cursor.down(false);
        cursor.up(true);
        cursor.home(true);
        assert_eq!(cursor.current.column, 2);
        assert_eq!(cursor.current.line, 1);
        assert_eq!(cursor.extender.column, 0);
        assert_eq!(cursor.extender.line, 0);
    }
    #[test]
    fn end_empty() {
        let mut cursor = Cursor::new(vec![0]);
        cursor.end(false);
        assert_eq!(cursor.current.column, 0);
    }
    #[test]
    fn end_line_start() {
        let mut cursor = Cursor::new(vec![10]);
        cursor.end(false);
        assert_eq!(cursor.current.column, 10);
    }
    #[test]
    fn end_line_end() {
        let mut cursor = Cursor::new(vec![4]);
        cursor.right(false);
        cursor.end(false);
        assert_eq!(cursor.current.column, 4);
        cursor.end(false);
        assert_eq!(cursor.current.column, 1);
    }
    #[test]
    fn end_line_end_with_line_switch() {
        let mut cursor = Cursor::new(vec![2, 10]);
        cursor.down(false);
        cursor.right(false);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.current.column, 3);
        cursor.up(false);
        assert_eq!(cursor.current.column, 2);
        cursor.end(false);
        assert_eq!(cursor.current.column, 2);
    }
    #[test]
    fn end_select_empty() {
        let mut cursor = Cursor::new(vec![0]);
        cursor.end(true);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.extender.column, 0);
    }
    #[test]
    fn end_select_in_same_line() {
        let mut cursor = Cursor::new(vec![10]);
        cursor.right(false);
        cursor.end(true);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.extender.column, 10);
        cursor.end(true);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.extender.column, 1);
    }
    #[test]
    fn end_select_with_multi_line_selection() {
        let mut cursor = Cursor::new(vec![5, 5]);
        cursor.right(false);
        cursor.right(true);
        cursor.down(true);
        cursor.end(true);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.extender.column, 5);
        assert_eq!(cursor.extender.line, 1);
    }
}
