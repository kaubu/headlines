use eframe::{run_native, epi::App, egui::CentralPanel, NativeOptions};

struct Headlines;

impl App for Headlines {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("article text");
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }
}

fn main() {
    let app = Headlines;
    let win_option = NativeOptions::default();
    run_native(Box::new(app), win_option);
}
