use eframe::{run_native, egui::{Vec2, CentralPanel, ScrollArea, Ui, Color32, Separator, TopBottomPanel, CtxRef, Label, Hyperlink, TextStyle}, NativeOptions, epi::App};

mod headlines;
use headlines::Headlines;

const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

impl App for Headlines {
    fn setup(
        &mut self, 
        ctx: &eframe::egui::CtxRef, 
        _frame: &mut eframe::epi::Frame<'_>, 
        _storage: Option<&dyn eframe::epi::Storage>
    ) {
        self.configure_fonts(ctx);
    }
    
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        self.render_top_panel(ctx);
        CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            ScrollArea::auto_sized().show(ui, |ui| {
                self.render_news_cards(ui);
            });
            render_footer(ctx);
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }
}

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("headlines");
    });
    ui.add_space(PADDING);
    let sep = Separator::default().spacing(20.);
    ui.add(sep);
}

fn render_footer(ctx: &CtxRef) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
            // Add api source website info
            ui.add(Label::new("API source: newsapi.org").monospace());
            // add link to egui framework
            ui.add(Hyperlink::new("https://github.com/emilk/egui").text("Made with egui").text_style(TextStyle::Monospace));
            // github link to headlines source
            ui.add(Hyperlink::new("https://github.com/kaubu/headlines")
                .text("kaubu/headlines")
                .text_style(TextStyle::Monospace));
            ui.add_space(10.);
        });
    });
}

fn main() {
    let app = Headlines::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540., 960.));
    run_native(Box::new(app), win_option);
}
