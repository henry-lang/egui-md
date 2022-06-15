use eframe::egui::*;

struct EguiMd {
    code: String,
}

impl Default for EguiMd {
    fn default() -> Self {
        Self {
            code: String::new(),
        }
    }
}

impl eframe::App for EguiMd {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {
                ScrollArea::vertical()
                    .id_source("editor")
                    .show(&mut columns[0], |ui| self.editor_ui(ui));
                ScrollArea::vertical()
                    .id_source("viewer")
                    .show(&mut columns[1], |ui| self.viewer_ui(ui))
            });
        });
    }
}

impl EguiMd {
    fn editor_ui(&mut self, ui: &mut Ui) {
        ui.add(
            TextEdit::multiline(&mut self.code)
                .desired_width(f32::INFINITY)
                .desired_rows(
                    (ui.available_height() / ui.text_style_height(&TextStyle::Monospace)) as usize,
                )
                .font(TextStyle::Monospace),
        );
    }

    fn viewer_ui(&mut self, ui: &mut Ui) {
        ui.add(Label::new("Hello, world!"));
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui-md",
        options,
        Box::new(|_| Box::new(EguiMd::default())),
    )
}
