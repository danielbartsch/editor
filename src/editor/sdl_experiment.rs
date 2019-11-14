#[path = "cursor.rs"]
mod cursor;

#[cfg(test)]
#[path = "cursor.test.rs"]
mod tests;

pub mod sdl2 {
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;
    use sdl2::rect::Point;
    use std::collections::HashSet;
    use std::time::Duration;

    use super::cursor::cursor::Cursor;

    static CHARACTER_WIDTH: i32 = 8;
    static CHARACTER_HEIGHT: i32 = 16;
    static LINE_GAP: i32 = 4;
    static CHARACTER_GAP: i32 = 2;

    pub fn run() {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window_width = 700;
        let window_height = 350;

        let window = video_subsystem
            .window("Editor", window_width, window_height)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();

        let mut event_pump = sdl_context.event_pump().unwrap();

        let mut cursor = Cursor::new(vec![
            String::from("0123456789abcdefghijklmnopqrstuvwxyz"),
            String::from("+-=*/\\'\"`#ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            String::from(""),
            String::from("Hi,"),
            String::from(""),
            String::from("This is an Editor"),
            String::from(""),
            String::from("It was made by \"Daniel Bartsch\" in `2019`."),
            String::from(""),
            String::from("It is written in \'Rust\',"),
            String::from("A new, pretty popular programming language."),
            String::from(""),
            String::from("# This is a shellscript-style comment!"),
            String::from("// (=C-Like-comment.)C + Java = This Syntax"),
            String::from(""),
            String::from("Some more characters: []{}^°$%<>?!&|_~€;:"),
        ]);

        video_subsystem.text_input().start();

        'running: loop {
            canvas.set_draw_color(Color::RGB(40, 40, 40));
            canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::TextInput { text, .. } => {
                        let mut characters = text.chars();
                        if let Some(character) = characters.next() {
                            cursor.add(character);
                        }
                    }
                    _ => {}
                }
            }

            canvas.set_draw_color(Color::RGB(80, 80, 80));

            let mut pressed_keys = HashSet::new();
            pressed_keys = event_pump
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .collect();

            let is_selecting_text =
                pressed_keys.contains(&Keycode::LShift) || pressed_keys.contains(&Keycode::RShift);

            if pressed_keys.contains(&Keycode::Right) {
                cursor.right(is_selecting_text);
            } else if pressed_keys.contains(&Keycode::Left) {
                cursor.left(is_selecting_text);
            } else if pressed_keys.contains(&Keycode::Down) {
                cursor.down(is_selecting_text);
            } else if pressed_keys.contains(&Keycode::Up) {
                cursor.up(is_selecting_text);
            } else if pressed_keys.contains(&Keycode::Delete) {
                cursor.delete();
            } else if pressed_keys.contains(&Keycode::Backspace) {
                cursor.backspace();
            } else if pressed_keys.contains(&Keycode::Return) {
                cursor.new_line();
            } else if pressed_keys.contains(&Keycode::Home) {
                cursor.home(is_selecting_text);
            } else if pressed_keys.contains(&Keycode::End) {
                cursor.end(is_selecting_text);
            }

            for (line_index, line) in cursor.lines.iter().enumerate() {
                let line_y_offset = line_index as i32 * (LINE_GAP + CHARACTER_HEIGHT);
                for (column_index, one_character_string) in
                    line.split("").filter(|string| string.len() > 0).enumerate()
                {
                    let mut characters = one_character_string.chars();
                    if let Some(character) = characters.next() {
                        let character_x_offset =
                            column_index as i32 * (CHARACTER_GAP + CHARACTER_WIDTH);
                        let coords = get_character_coords(&character);

                        for (index, (x1, y1)) in coords.iter().enumerate() {
                            if index < (coords.len() - 1) {
                                let (x2, y2) = coords[index + 1];

                                canvas
                                    .draw_line(
                                        Point::new(*x1 + character_x_offset, *y1 + line_y_offset),
                                        Point::new(x2 + character_x_offset, y2 + line_y_offset),
                                    )
                                    .unwrap();
                            }
                        }
                    }
                }
            }

            canvas.set_draw_color(Color::RGB(200, 200, 200));
            canvas
                .draw_line(
                    (
                        cursor.current.column as i32 * (CHARACTER_GAP + CHARACTER_WIDTH),
                        cursor.current.line as i32 * (LINE_GAP + CHARACTER_HEIGHT),
                    ),
                    (
                        cursor.current.column as i32 * (CHARACTER_GAP + CHARACTER_WIDTH),
                        cursor.current.line as i32 * (LINE_GAP + CHARACTER_HEIGHT)
                            + CHARACTER_HEIGHT,
                    ),
                )
                .unwrap();
            if cursor.current != cursor.extender {
                for current_line in if cursor.current.line > cursor.extender.line {
                    cursor.extender.line..=cursor.current.line
                } else {
                    cursor.current.line..=cursor.extender.line
                } {
                    canvas
                        .draw_line(
                            (
                                if cursor.current.line == current_line {
                                    cursor.current.column as i32 * (CHARACTER_GAP + CHARACTER_WIDTH)
                                } else {
                                    0
                                },
                                current_line as i32 * (LINE_GAP + CHARACTER_HEIGHT)
                                    + CHARACTER_HEIGHT / 2,
                            ),
                            (
                                if cursor.extender.line == current_line {
                                    cursor.extender.column as i32
                                        * (CHARACTER_GAP + CHARACTER_WIDTH)
                                } else {
                                    cursor.lines[current_line].len() as i32
                                        * (CHARACTER_GAP + CHARACTER_WIDTH)
                                },
                                current_line as i32 * (LINE_GAP + CHARACTER_HEIGHT)
                                    + CHARACTER_HEIGHT / 2,
                            ),
                        )
                        .unwrap();
                }
                canvas.set_draw_color(Color::RGB(135, 200, 200));
                canvas
                    .draw_line(
                        (
                            cursor.extender.column as i32 * (CHARACTER_GAP + CHARACTER_WIDTH),
                            cursor.extender.line as i32 * (LINE_GAP + CHARACTER_HEIGHT),
                        ),
                        (
                            cursor.extender.column as i32 * (CHARACTER_GAP + CHARACTER_WIDTH),
                            cursor.extender.line as i32 * (LINE_GAP + CHARACTER_HEIGHT)
                                + CHARACTER_HEIGHT,
                        ),
                    )
                    .unwrap();
            }

            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 16));
        }
    }

    fn get_line_coords(length: usize) -> Vec<(i32, i32)> {
        let line_length = length as i32 * (CHARACTER_GAP + CHARACTER_WIDTH);
        vec![
            (0, 0),
            (line_length as i32, 0),
            (line_length as i32, CHARACTER_HEIGHT),
            (0, CHARACTER_HEIGHT),
        ]
    }

    fn get_line_coords_line_draw(length: usize) -> Vec<(i32, i32)> {
        let mut coords = get_line_coords(length);
        if coords.len() > 0 {
            coords.push(coords[0]);
            coords
        } else {
            coords
        }
    }

    fn get_character_coords(character: &char) -> Vec<(i32, i32)> {
        match character {
            'a' => vec![
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT - CHARACTER_HEIGHT / 4),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT - CHARACTER_HEIGHT / 4),
            ],
            'b' => vec![
                (0, 0),
                (0, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT / 2),
            ],
            'c' => vec![
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
            ],
            'd' => vec![
                (CHARACTER_WIDTH, 0),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
            ],
            'e' => vec![
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT - CHARACTER_HEIGHT / 4),
                (0, CHARACTER_HEIGHT - CHARACTER_HEIGHT / 4),
            ],
            'f' => vec![
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT * 5 / 4),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 4),
                (0, CHARACTER_HEIGHT / 4),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 4),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 4),
                (CHARACTER_WIDTH / 2, 0),
                (CHARACTER_WIDTH, 0),
            ],
            'g' => vec![
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT * 5 / 4),
                (0, CHARACTER_HEIGHT * 5 / 4),
            ],
            'h' => vec![
                (0, 0),
                (0, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
            ],
            'i' => vec![
                (CHARACTER_WIDTH / 4, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH * 3 / 4, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH / 4, CHARACTER_HEIGHT / 2),
            ],
            'j' => vec![
                (CHARACTER_WIDTH / 4, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT * 5 / 4),
                (0, CHARACTER_HEIGHT * 5 / 4),
            ],
            'k' => vec![
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT / 3),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 3),
                (0, CHARACTER_HEIGHT / 3),
                (0, 0),
            ],
            'l' => vec![
                (CHARACTER_WIDTH / 4, 0),
                (CHARACTER_WIDTH / 4, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH * 3 / 4, CHARACTER_HEIGHT),
            ],
            'm' => vec![
                (0, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
            ],
            'n' => vec![
                (0, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
            ],
            'o' => vec![
                (0, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT / 2),
            ],
            'p' => vec![
                (0, CHARACTER_HEIGHT * 5 / 4),
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT),
            ],
            'q' => vec![
                (CHARACTER_WIDTH, CHARACTER_HEIGHT * 5 / 4),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
            ],
            'r' => vec![
                (0, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
            ],
            's' => vec![
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT - CHARACTER_HEIGHT / 4),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT - CHARACTER_HEIGHT / 4),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT),
            ],
            't' => vec![
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 4),
                (0, CHARACTER_HEIGHT / 4),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 4),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 4),
                (CHARACTER_WIDTH / 2, 0),
            ],
            'u' => vec![
                (0, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
            ],
            'v' => vec![
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
            ],
            'w' => vec![
                (0, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
            ],
            'x' => vec![
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT - CHARACTER_HEIGHT / 4),
                (0, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
            ],
            'y' => vec![
                (0, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT * 5 / 4),
                (0, CHARACTER_HEIGHT * 5 / 4),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT * 5 / 4),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
            ],
            'z' => vec![
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT - CHARACTER_HEIGHT / 4),
                (0, CHARACTER_HEIGHT - CHARACTER_HEIGHT / 4),
                (0, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
            ],
            '0' => vec![
                (0, 0),
                (CHARACTER_WIDTH / 2, 0),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH / 2, 0),
                (CHARACTER_WIDTH, 0),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT),
                (0, 0),
            ],
            '1' => vec![
                (CHARACTER_WIDTH / 4, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH * 3 / 4, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH / 2, 0),
                (CHARACTER_WIDTH / 4, 0),
            ],
            '2' => vec![
                (0, 0),
                (CHARACTER_WIDTH, 0),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
            ],
            '3' => vec![
                (0, 0),
                (CHARACTER_WIDTH, 0),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT),
            ],
            '4' => vec![
                (0, 0),
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, 0),
            ],
            '5' => vec![
                (CHARACTER_WIDTH, 0),
                (0, 0),
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT),
            ],
            '6' => vec![
                (CHARACTER_WIDTH, 0),
                (0, 0),
                (0, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT / 2),
            ],
            '7' => vec![
                (0, 0),
                (CHARACTER_WIDTH, 0),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
            ],
            '8' => vec![
                (0, 0),
                (CHARACTER_WIDTH, 0),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT / 2),
                (0, 0),
            ],
            '9' => vec![
                (0, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, 0),
                (0, 0),
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
            ],
            ',' => vec![
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT * 5 / 4),
            ],
            '.' => vec![
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT * 11 / 12),
            ],
            '\'' => vec![
                (CHARACTER_WIDTH / 2, 0),
                (CHARACTER_WIDTH / 2, -CHARACTER_HEIGHT / 3),
            ],
            '"' => vec![
                (CHARACTER_WIDTH * 1 / 3, 0),
                (CHARACTER_WIDTH * 1 / 3, -CHARACTER_HEIGHT / 3),
                (CHARACTER_WIDTH * 2 / 3, 0),
                (CHARACTER_WIDTH * 2 / 3, -CHARACTER_HEIGHT / 3),
            ],
            '`' => vec![
                (CHARACTER_WIDTH * 3 / 5, 0),
                (CHARACTER_WIDTH * 2 / 5, -CHARACTER_HEIGHT / 3),
            ],
            '+' => vec![
                (CHARACTER_WIDTH / 4, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH * 3 / 4, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT * 3 / 4),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 4),
            ],
            '-' => vec![
                (CHARACTER_WIDTH / 4, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH * 3 / 4, CHARACTER_HEIGHT / 2),
            ],
            '*' => vec![
                (CHARACTER_WIDTH / 6, CHARACTER_HEIGHT * 3 / 7),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH * 5 / 6, CHARACTER_HEIGHT * 3 / 7),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 4),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH * 5 / 6, CHARACTER_HEIGHT * 5 / 7),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH / 6, CHARACTER_HEIGHT * 5 / 7),
            ],
            '/' => vec![(0, CHARACTER_HEIGHT), (CHARACTER_WIDTH, 0)],
            '=' => vec![
                (CHARACTER_WIDTH / 4, CHARACTER_HEIGHT * 2 / 5),
                (CHARACTER_WIDTH * 3 / 4, CHARACTER_HEIGHT * 2 / 5),
                (CHARACTER_WIDTH / 4, CHARACTER_HEIGHT * 3 / 5),
                (CHARACTER_WIDTH * 3 / 4, CHARACTER_HEIGHT * 3 / 5),
            ],
            '\\' => vec![(0, 0), (CHARACTER_WIDTH, CHARACTER_HEIGHT)],
            '#' => vec![
                (CHARACTER_WIDTH / 4, 0),
                (CHARACTER_WIDTH / 4, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH / 4, CHARACTER_HEIGHT / 4),
                (0, CHARACTER_HEIGHT / 4),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 4),
                (CHARACTER_WIDTH * 3 / 4, CHARACTER_HEIGHT / 4),
                (CHARACTER_WIDTH * 3 / 4, 0),
                (CHARACTER_WIDTH * 3 / 4, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH * 3 / 4, CHARACTER_HEIGHT / 4),
                (CHARACTER_WIDTH * 3 / 4, CHARACTER_HEIGHT * 3 / 4),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT * 3 / 4),
                (0, CHARACTER_HEIGHT * 3 / 4),
            ],
            'A' => vec![
                (0, CHARACTER_HEIGHT),
                (0, 0),
                (CHARACTER_WIDTH, 0),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
            ],
            'B' => vec![
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT),
                (0, 0),
                (CHARACTER_WIDTH, 0),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH / 3, CHARACTER_HEIGHT / 2),
            ],
            'C' => vec![
                (CHARACTER_WIDTH, 0),
                (0, 0),
                (0, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
            ],
            'D' => vec![
                (0, 0),
                (CHARACTER_WIDTH * 4 / 5, 0),
                (CHARACTER_WIDTH * 4 / 5, CHARACTER_HEIGHT / 6),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 6),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT * 5 / 6),
                (CHARACTER_WIDTH * 4 / 5, CHARACTER_HEIGHT * 5 / 6),
                (CHARACTER_WIDTH * 4 / 5, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT),
                (0, 0),
            ],
            'E' => vec![
                (CHARACTER_WIDTH, 0),
                (0, 0),
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
            ],
            'F' => vec![
                (CHARACTER_WIDTH, 0),
                (0, 0),
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT),
            ],
            'G' => vec![
                (CHARACTER_WIDTH, 0),
                (0, 0),
                (0, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 2),
            ],
            'H' => vec![
                (0, 0),
                (0, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, 0),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
            ],
            'I' => vec![
                (0, 0),
                (CHARACTER_WIDTH, 0),
                (CHARACTER_WIDTH / 2, 0),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
            ],
            'J' => vec![
                (0, 0),
                (CHARACTER_WIDTH, 0),
                (CHARACTER_WIDTH / 2, 0),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT),
            ],
            'K' => vec![
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT * 5 / 12),
                (0, CHARACTER_HEIGHT * 5 / 12),
                (0, CHARACTER_HEIGHT),
                (0, 0),
                (0, CHARACTER_HEIGHT * 7 / 12),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT * 7 / 12),
                (CHARACTER_WIDTH, 0),
            ],
            'L' => vec![
                (0, 0),
                (0, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
            ],
            'M' => vec![
                (0, CHARACTER_HEIGHT),
                (0, 0),
                (CHARACTER_WIDTH / 2, 0),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH / 2, 0),
                (CHARACTER_WIDTH, 0),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
            ],
            'N' => vec![
                (0, CHARACTER_HEIGHT),
                (0, 0),
                (CHARACTER_WIDTH / 2, 0),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, 0),
            ],
            'O' => vec![
                (0, 0),
                (CHARACTER_WIDTH, 0),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT),
                (0, 0),
            ],
            'P' => vec![
                (0, CHARACTER_HEIGHT),
                (0, 0),
                (CHARACTER_WIDTH, 0),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT / 2),
            ],
            'Q' => vec![
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT),
                (0, 0),
                (CHARACTER_WIDTH, 0),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 2),
            ],
            'R' => vec![
                (0, CHARACTER_HEIGHT),
                (0, 0),
                (CHARACTER_WIDTH, 0),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH * 3 / 4, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH * 3 / 4, CHARACTER_HEIGHT),
            ],
            'S' => vec![
                (CHARACTER_WIDTH, 0),
                (0, 0),
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT * 3 / 4),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 4),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (0, CHARACTER_HEIGHT),
            ],
            'T' => vec![
                (0, 0),
                (CHARACTER_WIDTH, 0),
                (CHARACTER_WIDTH / 2, 0),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT),
            ],
            'U' => vec![
                (0, 0),
                (0, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, 0),
            ],
            'V' => vec![
                (0, 0),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, 0),
            ],
            'W' => vec![
                (0, 0),
                (0, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH / 2, 0),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, 0),
            ],
            'X' => vec![
                (0, 0),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT - CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, 0),
            ],
            'Y' => vec![
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT / 2),
                (0, 0),
                (0, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, 0),
            ],
            'Z' => vec![
                (0, 0),
                (CHARACTER_WIDTH, 0),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT * 3 / 4),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 4),
                (CHARACTER_WIDTH / 2, CHARACTER_HEIGHT / 2),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT / 2),
                (0, CHARACTER_HEIGHT),
                (CHARACTER_WIDTH, CHARACTER_HEIGHT),
            ],
            _ => vec![],
        }
    }
}
