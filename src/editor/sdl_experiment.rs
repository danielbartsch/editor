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
            vec!['H', 'i', ','],
            vec![],
            vec![
                'T', 'h', 'i', 's', ' ', 'i', 's', ' ', 'a', 'n', ' ', 'E', 'd', 'i', 't', 'o', 'r',
            ],
            vec![],
            vec![
                'I', 't', ' ', 'w', 'a', 's', ' ', 'm', 'a', 'd', 'e', 'b', 'y', ' ', 'D', 'a',
                'n', 'i', 'e', 'l', ' ', 'B', 'a', 'r', 't', 's', 'c', 'h', ' ', 'i', 'n', ' ',
                '2', '0', '1', '9',
            ],
            vec![],
            vec![
                'I', 't', ' ', 'i', 's', ' ', 'w', 'r', 'i', 't', 't', 'e', 'n', ' ', 'i', 'n',
                ' ', 'R', 'u', 's', 't', ',',
            ],
            vec![
                'A', ' ', 'n', 'e', 'w', ',', ' ', 'p', 'r', 'e', 't', 't', 'y', ' ', 'p', 'o',
                'p', 'u', 'l', 'a', 'r', ' ', 'p', 'r', 'o', 'g', 'r', 'a', 'm', 'm', 'i', 'n',
                'g', ' ', 'l', 'a', 'n', 'g', 'u', 'a', 'g', 'e',
            ],
            vec![],
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
            } else if pressed_keys.len() > 0 {
                for key_code in pressed_keys.drain() {
                    let maybe_character = match key_code {
                        Keycode::A => Some('a'),
                        Keycode::B => Some('b'),
                        Keycode::C => Some('c'),
                        Keycode::D => Some('d'),
                        Keycode::E => Some('e'),
                        Keycode::F => Some('f'),
                        Keycode::G => Some('g'),
                        Keycode::H => Some('h'),
                        Keycode::I => Some('i'),
                        Keycode::J => Some('j'),
                        Keycode::K => Some('k'),
                        Keycode::L => Some('l'),
                        Keycode::M => Some('m'),
                        Keycode::N => Some('n'),
                        Keycode::O => Some('o'),
                        Keycode::P => Some('p'),
                        Keycode::Q => Some('q'),
                        Keycode::R => Some('r'),
                        Keycode::S => Some('s'),
                        Keycode::T => Some('t'),
                        Keycode::U => Some('u'),
                        Keycode::V => Some('v'),
                        Keycode::W => Some('w'),
                        Keycode::X => Some('x'),
                        Keycode::Y => Some('y'),
                        Keycode::Z => Some('z'),
                        _ => None,
                    };
                    if let Some(character) = maybe_character {
                        cursor.add(character);
                    }
                }
            }

            for (line_index, line) in cursor.lines.iter().enumerate() {
                let coords = get_line_coords_line_draw(&line.len());

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
                        cursor.current.column as i32 * CHARACTER_WIDTH,
                        cursor.current.line as i32 * (LINE_GAP + CHARACTER_HEIGHT),
                    ),
                    (
                        cursor.current.column as i32 * CHARACTER_WIDTH,
                        cursor.current.line as i32 * (LINE_GAP + CHARACTER_HEIGHT)
                            + CHARACTER_HEIGHT,
                    ),
                )
                .unwrap();
            if cursor.current != cursor.extender {
                for current_line in if cursor.current.line > cursor.extender.line {
                    cursor.extender.line..(cursor.current.line + 1)
                } else {
                    cursor.current.line..(cursor.extender.line + 1)
                } {
                    canvas
                        .draw_line(
                            (
                                if cursor.current.line == current_line {
                                    cursor.current.column as i32 * CHARACTER_WIDTH
                                } else {
                                    0
                                },
                                current_line as i32 * (LINE_GAP + CHARACTER_HEIGHT)
                                    + CHARACTER_HEIGHT / 2,
                            ),
                            (
                                if cursor.extender.line == current_line {
                                    cursor.extender.column as i32 * CHARACTER_WIDTH
                                } else {
                                    cursor.lines[current_line].len() as i32 * CHARACTER_WIDTH
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
                            cursor.extender.column as i32 * CHARACTER_WIDTH,
                            cursor.extender.line as i32 * (LINE_GAP + CHARACTER_HEIGHT),
                        ),
                        (
                            cursor.extender.column as i32 * CHARACTER_WIDTH,
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
