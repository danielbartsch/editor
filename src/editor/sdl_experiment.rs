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

    pub fn run() {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window_width = 700;
        let window_height = 350;

        let window = video_subsystem
            .window("My Rusty Experiment", window_width, window_height)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();

        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut x = 0;
        let mut y = 0;

        let mut cursor = Cursor::new(vec![1, 4, 27, 1, 35, 90, 70, 10, 43, 1, 1, 1, 10, 10, 10, 1]);

        'running: loop {
            canvas.set_draw_color(Color::RGB(30, 64, 255));
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

            canvas.set_draw_color(Color::RGB(64, 255, 30));

            let mut pressed_keys = HashSet::new();
            pressed_keys = event_pump
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .collect();

            x += if pressed_keys.contains(&Keycode::Right) {
                cursor.right();
                1
            } else if pressed_keys.contains(&Keycode::Left) {
                cursor.left();
                -1
            } else {
                0
            };

            y += if pressed_keys.contains(&Keycode::Down) {
                cursor.down();
                1
            } else if pressed_keys.contains(&Keycode::Up) {
                cursor.up();
                -1
            } else {
                0
            };

            println!("cursor: {:?}", cursor);

            for (lineIndex, length) in cursor.line_lengths.iter().enumerate() {
                let coords = get_line_coords_line_draw(length);

                for (index, (x1, y1)) in coords.iter().enumerate() {
                    if index < (coords.len() - 1) {
                        let (x2, y2) = coords[index + 1];

                        let line_y_offset = lineIndex as i32 * 20;

                        canvas.draw_line(
                            Point::new(*x1, *y1 + line_y_offset),
                            Point::new(x2, y2 + line_y_offset),
                        );
                    }
                }
            }

            let coords = get_unit_circle_coords_line_draw(42);
            for (index, (x1, y1)) in coords.iter().enumerate() {
                if index < (coords.len() - 1) {
                    let (x2, y2) = coords[index + 1];
                    canvas.draw_line(
                        (
                            ((x1 * 50.0) as i32 + x * 2) % window_width as i32,
                            ((y1 * 50.0) as i32 + y * 2) % window_height as i32,
                        ),
                        (
                            ((x2 * 50.0) as i32 + x * 2) % window_width as i32,
                            ((y2 * 50.0) as i32 + y * 2) % window_height as i32,
                        ),
                    );
                }
            }

            canvas.set_draw_color(Color::RGB(255, 64, 30));
            canvas.draw_line(
                (
                    cursor.start.column as i32 * 8,
                    cursor.start.line as i32 * 20,
                ),
                (
                    cursor.start.column as i32 * 8,
                    cursor.start.line as i32 * 20 + 16,
                ),
            );

            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 16));
        }
    }

    fn get_unit_circle_coords(coord_count: u16) -> Vec<(f64, f64)> {
        (0..coord_count)
            .map(|degrees| {
                (((degrees as f64) / (coord_count as f64)) * 2.0 * std::f64::consts::PI).sin_cos()
            })
            .collect::<Vec<(f64, f64)>>()
    }

    fn get_unit_circle_coords_line_draw(coord_count: u16) -> Vec<(f64, f64)> {
        let mut coords = get_unit_circle_coords(coord_count);
        if coords.len() > 0 {
            coords.push(coords[0]);
            coords
        } else {
            coords
        }
    }

    fn get_line_coords(length: &usize) -> Vec<(i32, i32)> {
        let character_width = 8;
        let character_height = 16;

        let line_length = length * character_width;
        vec![
            (0, 0),
            (line_length as i32, 0),
            (line_length as i32, character_height),
            (0, character_height),
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
