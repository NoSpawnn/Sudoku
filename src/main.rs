use std::env;

use gui::gui::Program;

mod gui;
mod sudoku;

fn main() -> iced::Result {
    unsafe {
        env::set_var("WGPU_POWER_PREF", "high");
    }

    Program::run()
}
