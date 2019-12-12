#[path = "cursor.rs"]
mod cursor;

#[cfg(test)]
mod tests {
    use super::cursor::cursor::Cursor;
    #[test]
    fn right_empty() {
        let mut empty = Cursor::new(vec![String::from("")]);
        empty.right(false);
        assert_eq!(empty.current.column, 0);
    }
    #[test]
    fn right_in_line() {
        let mut cursor = Cursor::new(vec![String::from("abc")]);
        cursor.right(false);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.current.column, 3);
    }
    #[test]
    fn right_next_line() {
        let mut cursor = Cursor::new(vec![String::from("a"), String::from("")]);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.current.line, 1);
    }
    #[test]
    fn right_empty_line_with_next_line() {
        let mut cursor = Cursor::new(vec![String::from(""), String::from("ab")]);
        cursor.right(false);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.current.line, 1);
    }
    #[test]
    fn right_last_line() {
        let mut cursor = Cursor::new(vec![String::from("ab")]);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.current.column, 2);
        cursor.right(false);
        assert_eq!(cursor.current.column, 2);
    }
    #[test]
    fn right_select_empty() {
        let mut cursor = Cursor::new(vec![String::from("")]);
        cursor.right(true);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.extender.column, 0);
    }
    #[test]
    fn right_select_same_line() {
        let mut cursor = Cursor::new(vec![String::from("abc")]);
        cursor.right(true);
        cursor.right(true);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.extender.column, 2);
    }
    #[test]
    fn right_select_multiple_lines() {
        let mut cursor = Cursor::new(vec![String::from("a"), String::from("bc")]);
        cursor.right(true);
        cursor.right(true);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.extender.column, 0);
        assert_eq!(cursor.extender.line, 1);
    }
    #[test]
    fn right_from_selection_same_line() {
        let mut cursor = Cursor::new(vec![String::from("abcd")]);
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
        let mut cursor = Cursor::new(vec![String::from("abc"), String::from("abc")]);
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
        let mut cursor = Cursor::new(vec![String::from("")]);
        cursor.left(false);
        assert_eq!(cursor.current.column, 0);
    }
    #[test]
    fn left_same_line() {
        let mut cursor = Cursor::new(vec![String::from("abc")]);
        cursor.right(false);
        assert_eq!(cursor.current.column, 1);
        cursor.left(false);
        assert_eq!(cursor.current.column, 0);
        cursor.left(false);
        assert_eq!(cursor.current.column, 0);
    }
    #[test]
    fn left_start_of_line() {
        let mut cursor = Cursor::new(vec![String::from("abc"), String::from("a")]);
        cursor.down(false);
        cursor.left(false);
        assert_eq!(cursor.current.column, 3);
    }
    #[test]
    fn left_start_of_line_empty_line() {
        let mut cursor = Cursor::new(vec![String::from(""), String::from("abc")]);
        cursor.down(false);
        cursor.left(false);
        assert_eq!(cursor.current.column, 0);
    }
    #[test]
    fn left_select_empty() {
        let mut cursor = Cursor::new(vec![String::from("")]);
        cursor.left(true);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.extender.column, 0);
    }
    #[test]
    fn left_select_same_line() {
        let mut cursor = Cursor::new(vec![String::from("abc")]);
        cursor.right(false);
        cursor.right(false);
        cursor.left(true);
        assert_eq!(cursor.current.column, 2);
        assert_eq!(cursor.extender.column, 1);
    }
    #[test]
    fn left_select_multiple_lines() {
        let mut cursor = Cursor::new(vec![
            String::from("ab"),
            String::from(""),
            String::from("cd"),
        ]);
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
        let mut cursor = Cursor::new(vec![String::from("abcdef")]);
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
        let mut cursor = Cursor::new(vec![String::from("a"), String::from("bcd")]);
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
        let mut cursor = Cursor::new(vec![String::from("")]);
        cursor.up(true);
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.extender.line, 0);
    }
    #[test]
    fn up_select_same_line_lengths() {
        let mut cursor = Cursor::new(vec![String::from("ab"), String::from("cd")]);
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
        let mut cursor = Cursor::new(vec![String::from("a"), String::from(""), String::from("b")]);
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
        let mut cursor = Cursor::new(vec![String::from("a"), String::from("abc")]);
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
        let mut cursor = Cursor::new(vec![
            String::from("a"),
            String::from("bcd"),
            String::from("efg"),
        ]);
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
        let mut cursor = Cursor::new(vec![String::from("")]);
        cursor.down(true);
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.extender.line, 0);
    }
    #[test]
    fn down_select_same_line_lengths() {
        let mut cursor = Cursor::new(vec![String::from("a"), String::from("b")]);
        cursor.right(false);
        cursor.down(true);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.extender.column, 1);
        assert_eq!(cursor.extender.line, 1);
    }
    #[test]
    fn down_select_different_line_lengths() {
        let mut cursor = Cursor::new(vec![
            String::from("ab"),
            String::from(""),
            String::from("cd"),
        ]);
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
        let mut cursor = Cursor::new(vec![
            String::from("ab"),
            String::from(""),
            String::from("cd"),
        ]);
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
        let mut cursor = Cursor::new(vec![String::from("abcdef"), String::from("z")]);
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
        let mut cursor = Cursor::new(vec![
            String::from("abc"),
            String::from("d"),
            String::from("efg"),
        ]);
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
        let mut cursor = Cursor::new(vec![String::from("")]);
        cursor.up(false);
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.current.column, 0);
    }
    #[test]
    fn up_in_first_line() {
        let mut cursor = Cursor::new(vec![String::from("abc")]);
        cursor.right(false);
        assert_eq!(cursor.current.column, 1);
        cursor.up(false);
        assert_eq!(cursor.current.column, 1);
    }
    #[test]
    fn up_same_line_length() {
        let mut cursor = Cursor::new(vec![String::from("a"), String::from("b")]);
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
        let mut cursor = Cursor::new(vec![
            String::from("ab"),
            String::from("c"),
            String::from("de"),
        ]);
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
        let mut cursor = Cursor::new(vec![String::from("")]);
        cursor.down(false);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.current.line, 0);
    }
    #[test]
    fn down_in_last_line() {
        let mut cursor = Cursor::new(vec![String::from("abcd")]);
        cursor.right(false);
        cursor.right(false);
        assert_eq!(cursor.current.column, 2);
        cursor.down(false);
        assert_eq!(cursor.current.column, 2);
    }
    #[test]
    fn down_same_line_length() {
        let mut cursor = Cursor::new(vec![String::from("ab"), String::from("cd")]);
        cursor.right(false);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.current.line, 0);
        cursor.down(false);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.current.line, 1);
    }
    #[test]
    fn down_smaller_line_length() {
        let mut cursor = Cursor::new(vec![
            String::from("ab"),
            String::from("c"),
            String::from("de"),
        ]);
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
        let mut cursor = Cursor::new(vec![String::from("")]);
        cursor.delete();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.lines, vec![String::from("")]);
    }
    #[test]
    fn delete_in_line() {
        let mut cursor = Cursor::new(vec![String::from("abc")]);
        cursor.delete();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.lines, vec![String::from("bc")]);
    }
    #[test]
    fn delete_empty_line() {
        let mut cursor = Cursor::new(vec![String::from(""), String::from("abc")]);
        cursor.delete();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.lines, vec![String::from("abc")]);
    }
    #[test]
    fn delete_at_end_of_line() {
        let mut cursor = Cursor::new(vec![String::from("ab"), String::from("cdef")]);
        cursor.right(false);
        cursor.right(false);
        cursor.delete();
        assert_eq!(cursor.current.column, 2);
        assert_eq!(cursor.lines, vec![String::from("abcdef")]);
    }
    #[test]
    fn delete_at_end_of_last_line() {
        let mut cursor = Cursor::new(vec![String::from("ab")]);
        cursor.right(false);
        cursor.right(false);
        cursor.delete();
        assert_eq!(cursor.current.column, 2);
        assert_eq!(cursor.lines, vec![String::from("ab")]);
    }
    #[test]
    fn delete_multi_byte() {
        let mut cursor = Cursor::new(vec![String::from("°")]);
        cursor.delete();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.lines, vec![String::from("")]);
    }
    #[test]
    fn delete_with_selection() {
        let mut cursor = Cursor::new(vec![String::from("abcd")]);
        cursor.right(true);
        cursor.right(true);
        cursor.delete();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.lines, vec![String::from("cd")]);
    }
    #[test]
    fn delete_with_right_left_selection() {
        let mut cursor = Cursor::new(vec![String::from("abcd")]);
        cursor.right(false);
        cursor.right(false);
        cursor.left(true);
        cursor.left(true);
        cursor.delete();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.lines, vec![String::from("cd")]);
    }
    #[test]
    fn delete_with_multi_line_selection() {
        let mut cursor = Cursor::new(vec![
            String::from("abcd"),
            String::from("efgh"),
            String::from("ijkl"),
        ]);
        cursor.right(false);
        cursor.right(false);
        cursor.down(true);
        cursor.down(true);
        cursor.delete();
        assert_eq!(cursor.current.column, 2);
        assert_eq!(cursor.lines, vec![String::from("abkl")]);
    }
    #[test]
    fn delete_with_bottom_up_multi_line_selection() {
        let mut cursor = Cursor::new(vec![
            String::from("abcd"),
            String::from("efgh"),
            String::from("ijkl"),
        ]);
        cursor.down(false);
        cursor.down(false);
        cursor.right(false);
        cursor.right(false);
        cursor.up(true);
        cursor.up(true);
        cursor.delete();
        assert_eq!(cursor.current.column, 2);
        assert_eq!(cursor.lines, vec![String::from("abkl")]);
    }
    #[test]
    fn backspace_empty() {
        let mut cursor = Cursor::new(vec![String::from("")]);
        cursor.backspace();
        assert_eq!(cursor.lines, vec![String::from("")]);
    }
    #[test]
    fn backspace_in_line() {
        let mut cursor = Cursor::new(vec![String::from("ab")]);
        cursor.right(false);
        cursor.backspace();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.lines, vec![String::from("b")]);
    }
    #[test]
    fn backspace_delete_line() {
        let mut cursor = Cursor::new(vec![
            String::from("ab"),
            String::from(""),
            String::from("cd"),
        ]);
        cursor.down(false);
        cursor.backspace();
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.current.column, 2);
        assert_eq!(cursor.lines, vec![String::from("ab"), String::from("cd")]);
    }
    #[test]
    fn backspace_multi_byte() {
        let mut cursor = Cursor::new(vec![String::from("🌈°b")]);
        cursor.right(false);
        cursor.right(false);
        cursor.right(false);
        cursor.backspace();
        assert_eq!(cursor.current.column, 2);
        assert_eq!(cursor.lines, vec![String::from("🌈°")]);
    }
    #[test]
    fn backspace_with_multi_line_selection() {
        let mut cursor = Cursor::new(vec![
            String::from("abcd"),
            String::from("efgh"),
            String::from("ijkl"),
        ]);
        cursor.right(false);
        cursor.right(false);
        cursor.down(true);
        cursor.down(true);
        cursor.backspace();
        assert_eq!(cursor.current.column, 2);
        assert_eq!(cursor.lines, vec![String::from("abkl")]);
    }
    #[test]
    fn add_empty() {
        let mut cursor = Cursor::new(vec![String::from("")]);
        cursor.add('a');
        assert_eq!(cursor.lines, vec![String::from("a")]);
    }
    #[test]
    fn add() {
        let mut cursor = Cursor::new(vec![String::from("cde")]);
        cursor.add('b');
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.lines, vec![String::from("bcde")]);
    }
    #[test]
    fn add_after_multi_byte() {
        let mut cursor = Cursor::new(vec![String::from("🌈°")]);
        cursor.right(false);
        cursor.add('b');
        assert_eq!(cursor.current.column, 2);
        assert_eq!(cursor.lines, vec![String::from("🌈b°")]);
    }
    #[test]
    fn add_with_selection() {
        let mut cursor = Cursor::new(vec![String::from("cde")]);
        cursor.right(true);
        cursor.right(true);
        cursor.add('ß');
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.lines, vec![String::from("ße")]);
    }
    #[test]
    fn new_line_empty() {
        let mut cursor = Cursor::new(vec![String::from("")]);
        cursor.new_line();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.current.line, 1);
        assert_eq!(cursor.lines, vec![String::from(""), String::from("")]);
    }
    #[test]
    fn new_line_end_of_line_no_line_after() {
        let mut cursor = Cursor::new(vec![String::from("ab")]);
        cursor.right(false);
        cursor.right(false);
        cursor.new_line();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.current.line, 1);
        assert_eq!(cursor.lines, vec![String::from("ab"), String::from("")]);
    }
    #[test]
    fn new_line_middle_of_line() {
        let mut cursor = Cursor::new(vec![String::from("abcdef")]);
        cursor.right(false);
        cursor.right(false);
        cursor.new_line();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.current.line, 1);
        assert_eq!(cursor.lines, vec![String::from("ab"), String::from("cdef")]);
    }
    #[test]
    fn new_line_remember_column() {
        let mut cursor = Cursor::new(vec![String::from("abc"), String::from("def")]);
        cursor.right(false);
        cursor.right(false);
        cursor.new_line();
        assert_eq!(cursor.current.column, 0);
        cursor.down(false);
        assert_eq!(cursor.current.column, 2);
    }
    #[test]
    fn new_line_remember_column_line_smaller_than_remembered() {
        let mut cursor = Cursor::new(vec![String::from("abc"), String::from("d")]);
        cursor.right(false);
        cursor.right(false);
        cursor.new_line();
        assert_eq!(cursor.current.column, 0);
        cursor.down(false);
        assert_eq!(cursor.current.column, 1);
    }
    #[test]
    fn new_line_with_selection() {
        let mut cursor = Cursor::new(vec![
            String::from("abcd"),
            String::from("efgh"),
            String::from("ijkl"),
        ]);
        cursor.right(false);
        cursor.right(false);
        cursor.down(true);
        cursor.down(true);
        cursor.new_line();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.lines, vec![String::from("ab"), String::from("kl")]);
    }
    #[test]
    fn new_line_multi_byte() {
        let mut cursor = Cursor::new(vec![String::from("🌈°")]);
        cursor.right(false);
        cursor.right(false);
        cursor.new_line();
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.lines, vec![String::from("🌈°"), String::from("")]);
    }
    #[test]
    fn home_empty() {
        let mut cursor = Cursor::new(vec![String::from("")]);
        cursor.home(false);
        assert_eq!(cursor.current.column, 0);
    }
    #[test]
    fn home_line_start() {
        let mut cursor = Cursor::new(vec![String::from("abc")]);
        cursor.home(false);
        assert_eq!(cursor.current.column, 0);
    }
    #[test]
    fn home_middle_of_line() {
        let mut cursor = Cursor::new(vec![String::from("abc")]);
        cursor.right(false);
        cursor.right(false);
        cursor.home(false);
        assert_eq!(cursor.current.column, 0);
        cursor.home(false);
        assert_eq!(cursor.current.column, 2);
    }
    #[test]
    fn home_select_empty() {
        let mut cursor = Cursor::new(vec![String::from("")]);
        cursor.home(true);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.extender.column, 0);
    }
    #[test]
    fn home_select_in_same_line() {
        let mut cursor = Cursor::new(vec![String::from("abc")]);
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
        let mut cursor = Cursor::new(vec![String::from("abcd"), String::from("efgh")]);
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
        let mut cursor = Cursor::new(vec![String::from("")]);
        cursor.end(false);
        assert_eq!(cursor.current.column, 0);
    }
    #[test]
    fn end_line_start() {
        let mut cursor = Cursor::new(vec![String::from("abc")]);
        cursor.end(false);
        assert_eq!(cursor.current.column, 3);
    }
    #[test]
    fn end_line_end() {
        let mut cursor = Cursor::new(vec![String::from("abcd")]);
        cursor.right(false);
        cursor.end(false);
        assert_eq!(cursor.current.column, 4);
        cursor.end(false);
        assert_eq!(cursor.current.column, 1);
    }
    #[test]
    fn end_line_end_with_line_switch() {
        let mut cursor = Cursor::new(vec![String::from("ab"), String::from("cdefg")]);
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
        let mut cursor = Cursor::new(vec![String::from("")]);
        cursor.end(true);
        assert_eq!(cursor.current.column, 0);
        assert_eq!(cursor.extender.column, 0);
    }
    #[test]
    fn end_select_in_same_line() {
        let mut cursor = Cursor::new(vec![String::from("abcdef")]);
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
        let mut cursor = Cursor::new(vec![String::from("abcde"), String::from("fghij")]);
        cursor.right(false);
        cursor.right(true);
        cursor.down(true);
        cursor.end(true);
        assert_eq!(cursor.current.column, 1);
        assert_eq!(cursor.current.line, 0);
        assert_eq!(cursor.extender.column, 5);
        assert_eq!(cursor.extender.line, 1);
    }
    #[test]
    fn multi_line_string_to_cursor() {
        let cursor_linux =
            Cursor::multi_line_string_to_cursor(&"Hello\nWorld".to_string(), &"\n".to_string());
        let cursor_windows =
            Cursor::multi_line_string_to_cursor(&"Hello\n\rWorld".to_string(), &"\n\r".to_string());

        assert_eq!(cursor_linux.lines, cursor_windows.lines,);
        assert_eq!(
            cursor_linux.lines,
            Cursor::new(vec![String::from("Hello"), String::from("World"),]).lines
        );
    }
    #[test]
    fn to_multi_line_string() {
        let mut cursor = Cursor::new(vec![String::from("Hello"), String::from("World")]);

        assert_eq!(
            cursor.to_multi_line_string(&"\n".to_string()),
            "Hello\nWorld".to_string()
        );
        assert_eq!(
            cursor.to_multi_line_string(&"\n\r".to_string()),
            "Hello\n\rWorld".to_string()
        );
    }
}
