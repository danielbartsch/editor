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
    use std::error::Error;
    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    use std::time::Duration;

    use super::cursor::cursor::Cursor;
    use super::text_rendering::text_rendering::get_character_coords;

    static CHARACTER_WIDTH: i32 = 10;
    static CHARACTER_HEIGHT: i32 = 16;
    static LINE_GAP: i32 = CHARACTER_HEIGHT / 3;
    static CHARACTER_GAP: i32 = CHARACTER_WIDTH / 3;
    static CHARACTER_X_OFFSET: i32 = 10;
    static CHARACTER_Y_OFFSET: i32 = 5;

    fn get_character_x(column_index: i32) -> i32 {
        column_index * (CHARACTER_GAP + CHARACTER_WIDTH) + CHARACTER_X_OFFSET
    }

    fn get_character_y(line_index: i32) -> i32 {
        line_index * (LINE_GAP + CHARACTER_HEIGHT) + CHARACTER_Y_OFFSET
    }

    pub fn run() {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window_width = ((CHARACTER_WIDTH + CHARACTER_GAP) * 45) as u32;
        let window_height = ((CHARACTER_HEIGHT + LINE_GAP) * 20) as u32;

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

        let file_path = "./editorTestFile";

        let string_file = fs::read_to_string(file_path).expect("Unable to read file");
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
                    } => {
                        let path = Path::new(file_path);
                        let display = path.display();

                        let mut file = match File::create(&path) {
                            Err(why) => {
                                panic!("couldn't create {}: {}", display, why.description())
                            }
                            Ok(file) => file,
                        };

                        match file.write_all(
                            cursor
                                .lines
                                .into_iter()
                                .collect::<Vec<String>>()
                                .join("\n")
                                .as_bytes(),
                        ) {
                            Err(why) => {
                                panic!("couldn't write to {}: {}", display, why.description())
                            }
                            Ok(_) => {
                                println!("successfully wrote to {}", display);
                                break 'running;
                            }
                        }
                    }
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
                let line_y_offset = get_character_y(line_index as i32);
                for (column_index, one_character_string) in line
                    .split("")
                    .filter(|string| string.chars().count() > 0)
                    .enumerate()
                {
                    let mut characters = one_character_string.chars();
                    if let Some(character) = characters.next() {
                        let character_x_offset = get_character_x(column_index as i32);
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
                        get_character_x(cursor.current.column as i32),
                        get_character_y(cursor.current.line as i32),
                    ),
                    (
                        get_character_x(cursor.current.column as i32),
                        get_character_y(cursor.current.line as i32) + CHARACTER_HEIGHT,
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
                                    get_character_x(0),
                                    get_character_y(current_line as i32) + CHARACTER_HEIGHT / 2,
                                ),
                                (
                                    get_character_x(
                                        cursor.lines[current_line].chars().count() as i32
                                    ),
                                    get_character_y(current_line as i32) + CHARACTER_HEIGHT / 2,
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
                                get_character_x(selection_to_end_of_line.column as i32),
                                get_character_y(selection_to_end_of_line.line as i32)
                                    + CHARACTER_HEIGHT / 2,
                            ),
                            (
                                get_character_x(
                                    cursor.lines[selection_to_end_of_line.line].chars().count()
                                        as i32,
                                ),
                                get_character_y(selection_to_end_of_line.line as i32)
                                    + CHARACTER_HEIGHT / 2,
                            ),
                        )
                        .unwrap();
                    canvas
                        .draw_line(
                            (
                                get_character_x(0),
                                get_character_y(selection_from_beginning_of_line.line as i32)
                                    + CHARACTER_HEIGHT / 2,
                            ),
                            (
                                get_character_x(selection_from_beginning_of_line.column as i32),
                                get_character_y(selection_from_beginning_of_line.line as i32)
                                    + CHARACTER_HEIGHT / 2,
                            ),
                        )
                        .unwrap();
                } else {
                    canvas
                        .draw_line(
                            (
                                get_character_x(cursor.current.column as i32),
                                get_character_y(cursor.current.line as i32) + CHARACTER_HEIGHT / 2,
                            ),
                            (
                                get_character_x(cursor.extender.column as i32),
                                get_character_y(cursor.current.line as i32) + CHARACTER_HEIGHT / 2,
                            ),
                        )
                        .unwrap();
                }

                canvas.set_draw_color(Color::RGB(135, 200, 200));
                canvas
                    .draw_line(
                        (
                            get_character_x(cursor.extender.column as i32),
                            get_character_y(cursor.extender.line as i32),
                        ),
                        (
                            get_character_x(cursor.extender.column as i32),
                            get_character_y(cursor.extender.line as i32) + CHARACTER_HEIGHT,
                        ),
                    )
                    .unwrap();
            }

            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 16));
        }
    }

    fn get_line_coords(column_from: usize, column_to: usize) -> Vec<(i32, i32)> {
        let from = column_from as i32 * (CHARACTER_GAP + CHARACTER_WIDTH);
        let to = column_to as i32 * (CHARACTER_GAP + CHARACTER_WIDTH);
        vec![
            (from, 0),
            (to, 0),
            (to, CHARACTER_HEIGHT),
            (from, CHARACTER_HEIGHT),
        ]
    }

    fn get_line_coords_line_draw(column_from: usize, column_to: usize) -> Vec<(i32, i32)> {
        let mut coords = get_line_coords(column_from, column_to);
        if coords.len() > 0 {
            coords.push(coords[0]);
            coords
        } else {
            coords
        }
    }
}
