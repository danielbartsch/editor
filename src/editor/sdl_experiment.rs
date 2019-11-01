#[path = "cursor.rs"]
mod cursor;
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
            1, 4, 27, 1, 35, 90, 70, 10, 43, 1, 1, 1, 10, 10, 10, 1,
        ]);

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

            let do_select_text =
                pressed_keys.contains(&Keycode::LShift) || pressed_keys.contains(&Keycode::RShift);

            if pressed_keys.contains(&Keycode::Right) {
                cursor.right();
            } else if pressed_keys.contains(&Keycode::Left) {
                cursor.left();
            } else if pressed_keys.contains(&Keycode::Down) {
                cursor.down();
            } else if pressed_keys.contains(&Keycode::Up) {
                cursor.up();
            } else if pressed_keys.contains(&Keycode::Delete) {
                cursor.delete();
            } else if pressed_keys.contains(&Keycode::Backspace) {
                cursor.backspace();
            } else if pressed_keys.contains(&Keycode::Return) {
                cursor.new_line();
            } else if pressed_keys.contains(&Keycode::Home) {
                cursor.home(do_select_text);
            } else if pressed_keys.contains(&Keycode::End) {
                cursor.end(do_select_text);
            } else if pressed_keys.len() > 0 {
                cursor.add();
            }

            for (line_index, length) in cursor.line_lengths.iter().enumerate() {
                let coords = get_line_coords_line_draw(length);

                for (index, (x1, y1)) in coords.iter().enumerate() {
                    if index < (coords.len() - 1) {
                        let (x2, y2) = coords[index + 1];

                        let line_y_offset = line_index as i32 * (LINE_GAP + CHARACTER_HEIGHT);

                        canvas
                            .draw_line(
                                Point::new(*x1, *y1 + line_y_offset),
                                Point::new(x2, y2 + line_y_offset),
                            )
                            .unwrap();
                    }
                }
            }

            canvas.set_draw_color(Color::RGB(200, 200, 200));
            canvas
                .draw_line(
                    (
                        cursor.start.column as i32 * CHARACTER_WIDTH,
                        cursor.start.line as i32 * (LINE_GAP + CHARACTER_HEIGHT),
                    ),
                    (
                        cursor.start.column as i32 * CHARACTER_WIDTH,
                        cursor.start.line as i32 * (LINE_GAP + CHARACTER_HEIGHT) + CHARACTER_HEIGHT,
                    ),
                )
                .unwrap();
            if cursor.start != cursor.end {
                canvas
                    .draw_line(
                        (
                            cursor.end.column as i32 * CHARACTER_WIDTH,
                            cursor.end.line as i32 * (LINE_GAP + CHARACTER_HEIGHT),
                        ),
                        (
                            cursor.end.column as i32 * CHARACTER_WIDTH,
                            cursor.end.line as i32 * (LINE_GAP + CHARACTER_HEIGHT)
                                + CHARACTER_HEIGHT,
                        ),
                    )
                    .unwrap();
                if cursor.start.line == cursor.end.line {
                    canvas
                        .draw_line(
                            (
                                cursor.start.column as i32 * CHARACTER_WIDTH,
                                cursor.start.line as i32 * (LINE_GAP + CHARACTER_HEIGHT)
                                    + CHARACTER_HEIGHT / 2,
                            ),
                            (
                                cursor.end.column as i32 * CHARACTER_WIDTH,
                                cursor.end.line as i32 * (LINE_GAP + CHARACTER_HEIGHT)
                                    + CHARACTER_HEIGHT / 2,
                            ),
                        )
                        .unwrap();
                }
            }

            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 16));
        }
    }

    fn get_line_coords(length: &usize) -> Vec<(i32, i32)> {
        let line_length = *length as i32 * CHARACTER_WIDTH;
        vec![
            (0, 0),
            (line_length as i32, 0),
            (line_length as i32, CHARACTER_HEIGHT),
            (0, CHARACTER_HEIGHT),
        ]
    }

    fn get_line_coords_line_draw(length: &usize) -> Vec<(i32, i32)> {
        let mut coords = get_line_coords(length);
        if coords.len() > 0 {
            coords.push(coords[0]);
            coords
        } else {
            coords
        }
    }
}
