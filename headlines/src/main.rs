mod headlines;
use headlines::Headlines;

use eframe::{run_native, egui::{Vec2}, NativeOptions};

fn main() {
    let app = Headlines::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540., 960.));
    run_native(Box::new(app), win_option);
}
