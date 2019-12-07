#[path = "cursor.rs"]
mod cursor;

#[cfg(test)]
#[path = "cursor.test.rs"]
mod tests;

#[path = "text_rendering.rs"]
mod text_rendering;

pub mod sdl2 {
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;
    use sdl2::rect::Point;
    use std::collections::HashSet;
    use std::time::Duration;

    use super::cursor::cursor::Cursor;
    use super::text_rendering::text_rendering::get_character_coords;

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

        let string_file = fs::read_to_string("./editorTestFile").expect("Unable to read file");
        let mut cursor = Cursor::new(
            string_file
                .split('\n')
                .map(|string| String::from(string))
                .collect::<Vec<String>>(),
        );

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
                for (column_index, one_character_string) in line
                    .split("")
                    .filter(|string| string.chars().count() > 0)
                    .enumerate()
                {
                    let mut characters = one_character_string.chars();
                    if let Some(character) = characters.next() {
                        let character_x_offset =
                            column_index as i32 * (CHARACTER_GAP + CHARACTER_WIDTH);
                        let coords =
                            get_character_coords(&character, CHARACTER_WIDTH, CHARACTER_HEIGHT);

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
                if cursor.current.line != cursor.extender.line {
                    for current_line in if cursor.current.line > cursor.extender.line {
                        (cursor.extender.line + 1)..cursor.current.line
                    } else {
                        (cursor.current.line + 1)..cursor.extender.line
                    } {
                        canvas
                            .draw_line(
                                (
                                    0,
                                    current_line as i32 * (LINE_GAP + CHARACTER_HEIGHT)
                                        + CHARACTER_HEIGHT / 2,
                                ),
                                (
                                    cursor.lines[current_line].chars().count() as i32
                                        * (CHARACTER_GAP + CHARACTER_WIDTH),
                                    current_line as i32 * (LINE_GAP + CHARACTER_HEIGHT)
                                        + CHARACTER_HEIGHT / 2,
                                ),
                            )
                            .unwrap();
                    }

                    let (selection_to_end_of_line, selection_from_beginning_of_line) =
                        if cursor.current > cursor.extender {
                            (&cursor.extender, &cursor.current)
                        } else {
                            (&cursor.current, &cursor.extender)
                        };
                    canvas
                        .draw_line(
                            (
                                selection_to_end_of_line.column as i32
                                    * (CHARACTER_GAP + CHARACTER_WIDTH),
                                selection_to_end_of_line.line as i32
                                    * (LINE_GAP + CHARACTER_HEIGHT)
                                    + CHARACTER_HEIGHT / 2,
                            ),
                            (
                                cursor.lines[selection_to_end_of_line.line].chars().count() as i32
                                    * (CHARACTER_GAP + CHARACTER_WIDTH),
                                selection_to_end_of_line.line as i32
                                    * (LINE_GAP + CHARACTER_HEIGHT)
                                    + CHARACTER_HEIGHT / 2,
                            ),
                        )
                        .unwrap();
                    canvas
                        .draw_line(
                            (
                                0,
                                selection_from_beginning_of_line.line as i32
                                    * (LINE_GAP + CHARACTER_HEIGHT)
                                    + CHARACTER_HEIGHT / 2,
                            ),
                            (
                                selection_from_beginning_of_line.column as i32
                                    * (CHARACTER_GAP + CHARACTER_WIDTH),
                                selection_from_beginning_of_line.line as i32
                                    * (LINE_GAP + CHARACTER_HEIGHT)
                                    + CHARACTER_HEIGHT / 2,
                            ),
                        )
                        .unwrap();
                } else {
                    canvas
                        .draw_line(
                            (
                                cursor.current.column as i32 * (CHARACTER_GAP + CHARACTER_WIDTH),
                                cursor.current.line as i32 * (LINE_GAP + CHARACTER_HEIGHT)
                                    + CHARACTER_HEIGHT / 2,
                            ),
                            (
                                cursor.extender.column as i32 * (CHARACTER_GAP + CHARACTER_WIDTH),
                                cursor.current.line as i32 * (LINE_GAP + CHARACTER_HEIGHT)
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
}
