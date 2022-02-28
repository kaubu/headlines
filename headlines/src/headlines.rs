use std::borrow::Cow;

use eframe::{egui::{CtxRef, FontDefinitions, FontFamily, CentralPanel, ScrollArea, Layout, Hyperlink, Separator, Label, Color32}, epi::App};

const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

struct NewsCardData {
    title: String,
    desc: String,
    url: String,
}

pub struct Headlines {
    articles: Vec<NewsCardData>,
}

impl Headlines {
    pub fn new() -> Headlines {
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("title{}", a),
            desc: format!("desc{}", a),
            url: format!("https://example.com/{}", a),
        });

        Headlines {    
            articles: Vec::from_iter(iter)
        }
    }

    fn configure_fonts(&self, ctx: &CtxRef) {
        // Load font using ctx object

        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "CustomFont".to_string(), 
            Cow::Borrowed(include_bytes!("../../DejaVuSans.ttf"))
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Heading, 
            (FontFamily::Proportional, 35.)
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Body, 
            (FontFamily::Proportional, 20.)
        );
        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "CustomFont".to_string()
        );
        ctx.set_fonts(font_def);
    }

    fn render_news_cards(&self, ui: &mut eframe::egui::Ui) {
        for a in &self.articles {
            ui.add_space(PADDING);

            // Render title
            let title = format!("➤ {}", a.title);
            ui.colored_label(WHITE, title);

            // Render description
            ui.add_space(PADDING);
            let desc = Label::new(&a.desc).text_style(eframe::egui::TextStyle::Button);
            ui.add(desc);

            // Render hyperlinks
            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.add(Hyperlink::new(&a.url).text("read more ⬏"));
            });
            ui.add(Separator::default());
        }
    }
}

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
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::auto_sized().show(ui, |ui| {
                self.render_news_cards(ui);
            });
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }
}
