#[cfg(test)]
#[path = "editor/cursor.test.rs"]
mod tests;

#[path = "editor/ui.rs"]
mod ui;
use crate::ui::sdl2::*;

fn main() {
    run();
}
