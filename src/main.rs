#[cfg(test)]
#[path = "editor/cursor.test.rs"]
mod tests;

#[path = "editor/ui.rs"]
mod ui;
use crate::ui::editor;

fn main() {
    editor::run("./editorTestFile");
}
