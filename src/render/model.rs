#[derive(Debug, Clone)]
pub enum Block {
    Heading { level: u8, text: String },
    Paragraph(String),
    Link { text: String, href: String },
    Image { key: String, alt: String },
    Pre(String),
}

#[derive(Debug, Clone, Default)]
pub struct RenderedPage {
    pub title: Option<String>,
    pub blocks: Vec<Block>,
}

pub fn render_page_ui(
    ui: &mut egui::Ui,
    page: &RenderedPage,
    textures: &std::collections::BTreeMap<String, egui::TextureHandle>,
) {
    if let Some(title) = &page.title {
        ui.label(egui::RichText::new(title).strong());
        ui.separator();
    }

    for b in &page.blocks {
        match b {
            Block::Heading { level, text } => {
                let size = match level {
                    1 => 26.0,
                    2 => 22.0,
                    3 => 18.0,
                    _ => 16.0,
                };
                ui.label(egui::RichText::new(text).size(size).strong());
            }
            Block::Paragraph(t) => {
                ui.label(t);
            }
            Block::Link { text, href } => {
                ui.horizontal(|ui| {
                    ui.label("链接:");
                    ui.hyperlink_to(text, href);
                });
            }
            Block::Image { key, alt } => {
                if let Some(tex) = textures.get(key) {
                    let mut size = tex.size_vec2();
                    let max_w = ui.available_width().max(100.0);
                    if size.x > max_w {
                        let scale = max_w / size.x;
                        size *= scale;
                    }
                    ui.add(egui::Image::new(tex).fit_to_exact_size(size));
                } else {
                    ui.label(format!("[图片加载中/失败] {alt}"));
                }
            }
            Block::Pre(t) => {
                ui.add(
                    egui::TextEdit::multiline(&mut t.clone())
                        .desired_rows(6)
                        .font(egui::TextStyle::Monospace)
                        .interactive(false),
                );
            }
        }
        ui.add_space(6.0);
    }
}
