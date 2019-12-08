#[path = "cursor.rs"]
mod cursor;

#[path = "text_rendering.rs"]
mod text_rendering;

pub mod editor {
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;
    use sdl2::rect::Point;
    use sdl2::render::WindowCanvas;
    use std::collections::HashSet;
    use std::time::Duration;

    use super::cursor::cursor::Cursor;
    use super::text_rendering::text_rendering::get_character_coords;

    static CHARACTER_WIDTH: i32 = 10;
    static CHARACTER_HEIGHT: i32 = 16;
    static LINE_GAP: i32 = CHARACTER_HEIGHT / 3;
    static CHARACTER_GAP: i32 = CHARACTER_WIDTH / 3;
    static CHARACTER_X_OFFSET: i32 = 10;
    static CHARACTER_Y_OFFSET: i32 = 5;

    static BACKGROUND_COLOR: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0xff,
    };
    static TEXT_COLOR: Color = Color {
        r: 80,
        g: 80,
        b: 80,
        a: 0xff,
    };
    static CURSOR_COLOR: Color = Color {
        r: 200,
        g: 200,
        b: 200,
        a: 0xff,
    };
    static CURSOR_EXTENDER_COLOR: Color = Color {
        r: 135,
        g: 200,
        b: 200,
        a: 0xff,
    };
    static CURSOR_SELECTION_COLOR: Color = Color {
        r: 250,
        g: 150,
        b: 100,
        a: 0xff,
    };

    fn get_character_x(column_index: i32) -> i32 {
        column_index * (CHARACTER_GAP + CHARACTER_WIDTH) + CHARACTER_X_OFFSET
    }

    fn get_character_y(line_index: i32) -> i32 {
        line_index * (LINE_GAP + CHARACTER_HEIGHT) + CHARACTER_Y_OFFSET
    }

    pub fn run(file_path: &str) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window_width = ((CHARACTER_WIDTH + CHARACTER_GAP) * 45) as u32;
        let window_height = ((CHARACTER_HEIGHT + LINE_GAP) * 20) as u32;

        let scroll_height_in_lines = (window_height as f32 - CHARACTER_Y_OFFSET as f32)
            / (LINE_GAP as f32 + CHARACTER_HEIGHT as f32);

        let window = video_subsystem
            .window("Editor", window_width, window_height)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas: WindowCanvas = window.into_canvas().build().unwrap();
        canvas.clear();
        canvas.present();

        let mut event_pump = sdl_context.event_pump().unwrap();

        let mut cursor = Cursor::from_file(file_path);

        let mut camera_line: i32 = 0;

        video_subsystem.text_input().start();

        'running: loop {
            canvas.set_draw_color(BACKGROUND_COLOR);
            canvas.clear();

            let mut pressed_keys = HashSet::new();
            pressed_keys = event_pump
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .collect();

            let is_selecting_text =
                pressed_keys.contains(&Keycode::LShift) || pressed_keys.contains(&Keycode::RShift);

            let is_holding_ctrl =
                pressed_keys.contains(&Keycode::LCtrl) || pressed_keys.contains(&Keycode::RCtrl);

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running;
                    }
                    Event::KeyDown {
                        keycode, repeat, ..
                    } => {
                        match keycode {
                            Some(Keycode::Right) => cursor.right(is_selecting_text),
                            Some(Keycode::Left) => cursor.left(is_selecting_text),
                            Some(Keycode::Down) => cursor.down(is_selecting_text),
                            Some(Keycode::Up) => cursor.up(is_selecting_text),
                            Some(Keycode::Home) => cursor.home(is_selecting_text),
                            Some(Keycode::End) => cursor.end(is_selecting_text),
                            Some(Keycode::Delete) => cursor.delete(),
                            Some(Keycode::Backspace) => cursor.backspace(),
                            Some(Keycode::Return) => cursor.new_line(),
                            Some(Keycode::S) => {
                                if !repeat && is_holding_ctrl {
                                    if let Ok(_result) = cursor.to_file(file_path) {
                                        println!(
                                            "Saving current content to \"{}\" succeeded",
                                            file_path
                                        );
                                    } else {
                                        println!(
                                            "Saving current content to \"{}\" failed",
                                            file_path
                                        );
                                    }
                                }
                            }
                            Some(Keycode::A) => {
                                if !repeat && is_holding_ctrl {
                                    cursor.current.line = 0;
                                    cursor.current.column = 0;
                                    cursor.extender.line = cursor.lines.len() - 1;
                                    cursor.extender.column =
                                        cursor.lines[cursor.lines.len() - 1].chars().count();
                                }
                            }
                            Some(_) | None => {}
                        }
                        if cursor.extender.line
                            > (camera_line as usize + scroll_height_in_lines as usize)
                        {
                            camera_line =
                                cursor.extender.line as i32 - scroll_height_in_lines as i32;
                        } else if cursor.extender.line < camera_line as usize {
                            camera_line = cursor.extender.line as i32;
                        }
                    }
                    Event::TextInput { text, .. } => {
                        let mut characters = text.chars();
                        if let Some(character) = characters.next() {
                            cursor.add(character);
                        }
                    }
                    Event::MouseWheel { y, .. } => {
                        let mouse_wheel_y_scalar = -y;
                        if (mouse_wheel_y_scalar < 0 && (camera_line + mouse_wheel_y_scalar) >= 0)
                            || (mouse_wheel_y_scalar > 0
                                && (camera_line + mouse_wheel_y_scalar)
                                    <= cursor.lines.len() as i32)
                        {
                            camera_line += mouse_wheel_y_scalar
                        }
                    }
                    _ => {}
                }
            }

            canvas.set_draw_color(TEXT_COLOR);

            for (line_index, line) in cursor.lines[(camera_line as usize)..].iter().enumerate() {
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

            if cursor.current.line >= camera_line as usize {
                canvas.set_draw_color(CURSOR_COLOR);
                canvas
                    .draw_line(
                        (
                            get_character_x(cursor.current.column as i32),
                            get_character_y(cursor.current.line as i32 - camera_line)
                                - CHARACTER_HEIGHT * 1 / 5,
                        ),
                        (
                            get_character_x(cursor.current.column as i32),
                            get_character_y(cursor.current.line as i32 - camera_line)
                                + CHARACTER_HEIGHT * 6 / 5,
                        ),
                    )
                    .unwrap();
            }
            canvas.set_draw_color(CURSOR_SELECTION_COLOR);
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
                                    get_character_y(current_line as i32 - camera_line)
                                        + CHARACTER_HEIGHT / 2,
                                ),
                                (
                                    get_character_x(
                                        cursor.lines[current_line].chars().count() as i32
                                    ),
                                    get_character_y(current_line as i32 - camera_line)
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
                                get_character_x(selection_to_end_of_line.column as i32),
                                get_character_y(selection_to_end_of_line.line as i32 - camera_line)
                                    + CHARACTER_HEIGHT / 2,
                            ),
                            (
                                get_character_x(
                                    cursor.lines[selection_to_end_of_line.line].chars().count()
                                        as i32,
                                ),
                                get_character_y(selection_to_end_of_line.line as i32 - camera_line)
                                    + CHARACTER_HEIGHT / 2,
                            ),
                        )
                        .unwrap();
                    canvas
                        .draw_line(
                            (
                                get_character_x(0),
                                get_character_y(
                                    selection_from_beginning_of_line.line as i32 - camera_line,
                                ) + CHARACTER_HEIGHT / 2,
                            ),
                            (
                                get_character_x(selection_from_beginning_of_line.column as i32),
                                get_character_y(
                                    selection_from_beginning_of_line.line as i32 - camera_line,
                                ) + CHARACTER_HEIGHT / 2,
                            ),
                        )
                        .unwrap();
                } else {
                    canvas
                        .draw_line(
                            (
                                get_character_x(cursor.current.column as i32),
                                get_character_y(cursor.current.line as i32 - camera_line)
                                    + CHARACTER_HEIGHT / 2,
                            ),
                            (
                                get_character_x(cursor.extender.column as i32),
                                get_character_y(cursor.current.line as i32 - camera_line)
                                    + CHARACTER_HEIGHT / 2,
                            ),
                        )
                        .unwrap();
                }

                if cursor.extender.line >= camera_line as usize {
                    canvas.set_draw_color(CURSOR_EXTENDER_COLOR);
                    canvas
                        .draw_line(
                            (
                                get_character_x(cursor.extender.column as i32),
                                get_character_y(cursor.extender.line as i32 - camera_line)
                                    - CHARACTER_HEIGHT * 1 / 5,
                            ),
                            (
                                get_character_x(cursor.extender.column as i32),
                                get_character_y(cursor.extender.line as i32 - camera_line)
                                    + CHARACTER_HEIGHT * 6 / 5,
                            ),
                        )
                        .unwrap();
                }
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
