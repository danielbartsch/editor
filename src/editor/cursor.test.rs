#[path = "cursor.rs"]
mod cursor;

#[cfg(test)]
mod tests {
    use super::cursor::cursor::Cursor;
    #[test]
    fn right_empty() {
        let mut empty = Cursor::new(vec![vec![]]);
        empty.right(false);
        assert_eq!(empty.current.column, 0);
    }
    #[test]
    fn right_in_line() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c']]);
        cursor.right(false);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.current.column, 3);
    }
    #[test]
    fn right_next_line() {
        let mut cursor = Cursor::new(vec![vec!['a'], vec![]]);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.current.line, 1);
    }
    #[test]
    fn right_empty_line_with_next_line() {
        let mut cursor = Cursor::new(vec![vec![], vec!['a', 'b']]);
        cursor.right(false);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.current.line, 1);
    }
    #[test]
    fn right_last_line() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b']]);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.current.column, 2);
        cursor.right(false);
        assert_eq!(cursor.current.column, 2);
    }
    #[test]
    fn right_select_empty() {
        let mut cursor = Cursor::new(vec![vec![]]);
        cursor.right(true);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.extender.column, 0);
    }
    #[test]
    fn right_select_same_line() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c']]);
        cursor.right(true);
        cursor.right(true);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.extender.column, 2);
    }
    #[test]
    fn right_select_multiple_lines() {
        let mut cursor = Cursor::new(vec![vec!['a'], vec!['b', 'c']]);
        cursor.right(true);
        cursor.right(true);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.extender.column, 0);
        assert_eq!(cursor.extender.line, 1);
    }
    #[test]
    fn right_from_selection_same_line() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c', 'd']]);
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
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c'], vec!['a', 'b', 'c']]);
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
        let mut cursor = Cursor::new(vec![vec![]]);
        cursor.left(false);
        assert_eq!(cursor.current.column, 0);
    }
    #[test]
    fn left_same_line() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c']]);
        cursor.right(false);
        assert_eq!(cursor.current.column, 1);
        cursor.left(false);
        assert_eq!(cursor.current.column, 0);
        cursor.left(false);
        assert_eq!(cursor.current.column, 0);
    }
    #[test]
    fn left_start_of_line() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c'], vec!['a']]);
        cursor.down(false);
        cursor.left(false);
        assert_eq!(cursor.current.column, 3);
    }
    #[test]
    fn left_start_of_line_empty_line() {
        let mut cursor = Cursor::new(vec![vec![], vec!['a', 'b', 'c']]);
        cursor.down(false);
        cursor.left(false);
        assert_eq!(cursor.current.column, 0);
    }
    #[test]
    fn left_select_empty() {
        let mut cursor = Cursor::new(vec![vec![]]);
        cursor.left(true);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.extender.column, 0);
    }
    #[test]
    fn left_select_same_line() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c']]);
        cursor.right(false);
        cursor.right(false);
        cursor.left(true);
        assert_eq!(cursor.current.column, 2);
        assert_eq!(cursor.extender.column, 1);
    }
    #[test]
    fn left_select_multiple_lines() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b'], vec![], vec!['c', 'd']]);
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
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c', 'd', 'e', 'f']]);
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
        let mut cursor = Cursor::new(vec![vec!['a'], vec!['b', 'c', 'd']]);
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
        let mut cursor = Cursor::new(vec![vec![]]);
        cursor.up(true);
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.extender.line, 0);
    }
    #[test]
    fn up_select_same_line_lengths() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b'], vec!['c', 'd']]);
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
        let mut cursor = Cursor::new(vec![vec!['a'], vec![], vec!['b']]);
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
        let mut cursor = Cursor::new(vec![vec!['a'], vec!['a', 'b', 'c']]);
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
        let mut cursor = Cursor::new(vec![vec!['a'], vec!['b', 'c', 'd'], vec!['e', 'f', 'g']]);
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
        let mut cursor = Cursor::new(vec![vec![]]);
        cursor.down(true);
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.extender.line, 0);
    }
    #[test]
    fn down_select_same_line_lengths() {
        let mut cursor = Cursor::new(vec![vec!['a'], vec!['b']]);
        cursor.right(false);
        cursor.down(true);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.extender.column, 1);
        assert_eq!(cursor.extender.line, 1);
    }
    #[test]
    fn down_select_different_line_lengths() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b'], vec![], vec!['c', 'd']]);
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
        let mut cursor = Cursor::new(vec![vec!['a', 'b'], vec![], vec!['c', 'd']]);
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
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c', 'd', 'e', 'f'], vec!['z']]);
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
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c'], vec!['d'], vec!['e', 'f', 'g']]);
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
        let mut cursor = Cursor::new(vec![vec![]]);
        cursor.up(false);
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.current.column, 0);
    }
    #[test]
    fn up_in_first_line() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c']]);
        cursor.right(false);
        assert_eq!(cursor.current.column, 1);
        cursor.up(false);
        assert_eq!(cursor.current.column, 1);
    }
    #[test]
    fn up_same_line_length() {
        let mut cursor = Cursor::new(vec![vec!['a'], vec!['b']]);
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
        let mut cursor = Cursor::new(vec![vec!['a', 'b'], vec!['c'], vec!['d', 'e']]);
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
        let mut cursor = Cursor::new(vec![vec![]]);
        cursor.down(false);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.current.line, 0);
    }
    #[test]
    fn down_in_last_line() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c', 'd']]);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.current.column, 2);
        cursor.down(false);
        assert_eq!(cursor.current.column, 2);
    }
    #[test]
    fn down_same_line_length() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b'], vec!['c', 'd']]);
        cursor.right(false);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.current.line, 0);
        cursor.down(false);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.current.line, 1);
    }
    #[test]
    fn down_smaller_line_length() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b'], vec!['c'], vec!['d', 'e']]);
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
        let mut cursor = Cursor::new(vec![vec![]]);
        cursor.delete();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.lines, vec![vec![]]);
    }
    #[test]
    fn delete_in_line() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c']]);
        cursor.delete();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.lines, vec![vec!['b', 'c']]);
    }
    #[test]
    fn delete_empty_line() {
        let mut cursor = Cursor::new(vec![vec![], vec!['a', 'b', 'c']]);
        cursor.delete();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.lines, vec![vec!['a', 'b', 'c']]);
    }
    #[test]
    fn delete_at_end_of_line() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b'], vec!['c', 'd', 'e', 'f']]);
        cursor.right(false);
        cursor.right(false);
        cursor.delete();
        assert_eq!(cursor.current.column, 2);
        assert_eq!(cursor.lines, vec![vec!['a', 'b', 'c', 'd', 'e', 'f']]);
    }
    #[test]
    fn delete_at_end_of_last_line() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b']]);
        cursor.right(false);
        cursor.right(false);
        cursor.delete();
        assert_eq!(cursor.current.column, 2);
        assert_eq!(cursor.lines, vec![vec!['a', 'b']]);
    }
    #[test]
    fn backspace_empty() {
        let mut cursor = Cursor::new(vec![vec![]]);
        cursor.backspace();
        assert_eq!(cursor.lines, vec![vec![]]);
    }
    #[test]
    fn backspace_in_line() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b']]);
        cursor.right(false);
        cursor.backspace();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.lines, vec![vec!['b']]);
    }
    #[test]
    fn backspace_delete_line() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b'], vec![], vec!['c', 'd']]);
        cursor.down(false);
        cursor.backspace();
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.current.column, 2);
        assert_eq!(cursor.lines, vec![vec!['a', 'b'], vec!['c', 'd']]);
    }
    #[test]
    fn add_empty() {
        let mut cursor = Cursor::new(vec![vec![]]);
        cursor.add('a');
        assert_eq!(cursor.lines, vec![vec!['a']]);
    }
    #[test]
    fn add() {
        let mut cursor = Cursor::new(vec![vec!['c', 'd', 'e']]);
        cursor.add('b');
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.lines, vec![vec!['b', 'c', 'd', 'e']]);
    }
    #[test]
    fn new_line_empty() {
        let mut cursor = Cursor::new(vec![vec![]]);
        cursor.new_line();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.current.line, 1);
        assert_eq!(cursor.lines, vec![vec![], vec![]]);
    }
    #[test]
    fn new_line_end_of_line_no_line_after() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b']]);
        cursor.right(false);
        cursor.right(false);
        cursor.new_line();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.current.line, 1);
        assert_eq!(cursor.lines, vec![vec!['a', 'b'], vec![]]);
    }
    #[test]
    fn new_line_middle_of_line() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c', 'd', 'e', 'f']]);
        cursor.right(false);
        cursor.right(false);
        cursor.new_line();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.current.line, 1);
        assert_eq!(cursor.lines, vec![vec!['a', 'b'], vec!['c', 'd', 'e', 'f']]);
    }
    #[test]
    fn new_line_remember_column() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]);
        cursor.right(false);
        cursor.right(false);
        cursor.new_line();
        assert_eq!(cursor.current.column, 0);
        cursor.down(false);
        assert_eq!(cursor.current.column, 2);
    }
    #[test]
    fn new_line_remember_column_line_smaller_than_remembered() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c'], vec!['d']]);
        cursor.right(false);
        cursor.right(false);
        cursor.new_line();
        assert_eq!(cursor.current.column, 0);
        cursor.down(false);
        assert_eq!(cursor.current.column, 1);
    }
    #[test]
    fn home_empty() {
        let mut cursor = Cursor::new(vec![vec![]]);
        cursor.home(false);
        assert_eq!(cursor.current.column, 0);
    }
    #[test]
    fn home_line_start() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c']]);
        cursor.home(false);
        assert_eq!(cursor.current.column, 0);
    }
    #[test]
    fn home_middle_of_line() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c']]);
        cursor.right(false);
        cursor.right(false);
        cursor.home(false);
        assert_eq!(cursor.current.column, 0);
        cursor.home(false);
        assert_eq!(cursor.current.column, 2);
    }
    #[test]
    fn home_select_empty() {
        let mut cursor = Cursor::new(vec![vec![]]);
        cursor.home(true);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.extender.column, 0);
    }
    #[test]
    fn home_select_in_same_line() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c']]);
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
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c', 'd'], vec!['e', 'f', 'g', 'h']]);
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
        let mut cursor = Cursor::new(vec![vec![]]);
        cursor.end(false);
        assert_eq!(cursor.current.column, 0);
    }
    #[test]
    fn end_line_start() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c']]);
        cursor.end(false);
        assert_eq!(cursor.current.column, 3);
    }
    #[test]
    fn end_line_end() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c', 'd']]);
        cursor.right(false);
        cursor.end(false);
        assert_eq!(cursor.current.column, 4);
        cursor.end(false);
        assert_eq!(cursor.current.column, 1);
    }
    #[test]
    fn end_line_end_with_line_switch() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b'], vec!['c', 'd', 'e', 'f', 'g']]);
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
        let mut cursor = Cursor::new(vec![vec![]]);
        cursor.end(true);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.extender.column, 0);
    }
    #[test]
    fn end_select_in_same_line() {
        let mut cursor = Cursor::new(vec![vec!['a', 'b', 'c', 'd', 'e', 'f']]);
        cursor.right(false);
        cursor.end(true);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.extender.column, 6);
        cursor.end(true);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.extender.column, 1);
    }
    #[test]
    fn end_select_with_multi_line_selection() {
        let mut cursor = Cursor::new(vec![
            vec!['a', 'b', 'c', 'd', 'e'],
            vec!['f', 'g', 'h', 'i', 'j'],
        ]);
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
