use crate::net::{http, url_input};
use crate::render::{fonts, html, model};
use std::time::{Duration, Instant};

type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub struct BrowserApp {
    url_input: String,
    request_preview: String,
    response_head_preview: String,
    status_preview: String,

    rendered: Option<model::RenderedPage>,
    images: std::collections::BTreeMap<String, egui::TextureHandle>,
    animations: std::collections::BTreeMap<String, GifAnimation>,

    last_error: Option<String>,
}

struct GifAnimation {
    frames: Vec<egui::ColorImage>,
    durations: Vec<Duration>,
    index: usize,
    next_at: Instant,
}

impl BrowserApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.0);
        let _font = fonts::install_cjk_fonts(&cc.egui_ctx);
        Self {
            url_input: "www.test.com.cn".to_string(),
            request_preview: String::new(),
            response_head_preview: String::new(),
            status_preview: String::new(),
            rendered: None,
            images: Default::default(),
            animations: Default::default(),
            last_error: None,
        }
    }

    fn go(&mut self, ctx: &egui::Context) {
        self.last_error = None;
        self.rendered = None;
        self.images.clear();
        self.animations.clear();
        self.status_preview.clear();
        self.response_head_preview.clear();

        let parsed = match url_input::parse_user_url(&self.url_input) {
            Ok(u) => u,
            Err(e) => {
                self.last_error = Some(e.to_string());
                return;
            }
        };

        let req = http::build_assignment_like_get_request(&parsed);
        self.request_preview = String::from_utf8_lossy(&req).to_string();

        let resp = match http::fetch(&parsed, &req) {
            Ok(r) => r,
            Err(e) => {
                self.last_error = Some(e.to_string());
                return;
            }
        };

        self.status_preview = format!("{} {}", resp.status_code, resp.reason_phrase);
        self.response_head_preview = resp
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\n");

        if !(200..300).contains(&resp.status_code) {
            let body_text = String::from_utf8_lossy(&resp.body).to_string();
            self.rendered = Some(model::RenderedPage {
                title: Some(format!("{} {}", resp.status_code, resp.reason_phrase)),
                blocks: vec![model::Block::Pre(body_text)],
            });
            return;
        }

        let (rendered, image_jobs) = match html::render_html(&parsed, &resp) {
            Ok(x) => x,
            Err(e) => {
                self.last_error = Some(e.to_string());
                return;
            }
        };
        self.rendered = Some(rendered);

        // Fetch images synchronously (simple, per assignment scope)
        for job in image_jobs {
            match http::fetch_url_bytes(&job.url) {
                Ok(bytes) => match crate::render::images::decode_image(&bytes) {
                    Ok(crate::render::images::DecodedImage::Static(ci)) => {
                        let tex = ctx.load_texture(job.key.clone(), ci, egui::TextureOptions::default());
                        self.images.insert(job.key, tex);
                    }
                    Ok(crate::render::images::DecodedImage::AnimatedGif { frames, durations }) => {
                        if frames.is_empty() {
                            continue;
                        }
                        let first = frames[0].clone();
                        let tex = ctx.load_texture(job.key.clone(), first, egui::TextureOptions::default());
                        self.images.insert(job.key.clone(), tex);
                        let now = Instant::now();
                        let first_delay = durations.get(0).copied().unwrap_or(Duration::from_millis(100));
                        self.animations.insert(
                            job.key,
                            GifAnimation {
                                frames,
                                durations,
                                index: 0,
                                next_at: now + first_delay,
                            },
                        );
                    }
                    Err(e) => {
                        self.last_error = Some(format!("Image decode failed: {}", e));
                    }
                },
                Err(e) => {
                    self.last_error = Some(format!("Image fetch failed: {}", e));
                }
            }
        }
    }

    fn tick_animations(&mut self, ctx: &egui::Context) {
        if self.animations.is_empty() {
            return;
        }

        let now = Instant::now();
        let mut next_wakeup: Option<Duration> = None;

        for (key, anim) in self.animations.iter_mut() {
            if now >= anim.next_at {
                if anim.frames.is_empty() {
                    continue;
                }
                anim.index = (anim.index + 1) % anim.frames.len();

                if let Some(tex) = self.images.get_mut(key) {
                    tex.set(anim.frames[anim.index].clone(), egui::TextureOptions::default());
                }

                let delay = anim
                    .durations
                    .get(anim.index)
                    .copied()
                    .unwrap_or(Duration::from_millis(100));
                anim.next_at = now + delay;
            }

            let remaining = anim
                .next_at
                .saturating_duration_since(now)
                .max(Duration::from_millis(5));
            next_wakeup = Some(match next_wakeup {
                Some(cur) => cur.min(remaining),
                None => remaining,
            });
        }

        if let Some(d) = next_wakeup {
            ctx.request_repaint_after(d);
        }
    }
}

impl eframe::App for BrowserApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.tick_animations(ctx);

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let edit = ui.add(
                    egui::TextEdit::singleline(&mut self.url_input)
                        .desired_width(f32::INFINITY)
                        .hint_text("输入 URL，例如：www.test.com.cn 或 http://www.test.com.cn/cgi-bin/adder.exe?1500&212"),
                );

                let go_clicked = ui.button("Go").clicked();
                let enter = edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                if go_clicked || enter {
                    self.go(ctx);
                }
            });

            if let Some(err) = &self.last_error {
                ui.colored_label(egui::Color32::RED, err);
            }
        });

        egui::SidePanel::left("left").resizable(true).default_width(420.0).show(ctx, |ui| {
            ui.heading("HTTP 请求");
            ui.add(
                egui::TextEdit::multiline(&mut self.request_preview)
                    .desired_rows(10)
                    .font(egui::TextStyle::Monospace),
            );
            ui.separator();
            ui.heading("HTTP 响应头");
            ui.label(format!("Status: {}", self.status_preview));
            ui.add(
                egui::TextEdit::multiline(&mut self.response_head_preview)
                    .desired_rows(12)
                    .font(egui::TextStyle::Monospace),
            );
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("页面显示");
            ui.separator();

            if let Some(page) = &self.rendered {
                model::render_page_ui(ui, page, &self.images);
            } else {
                ui.label("尚未加载。请输入 URL 并点击 Go。\n仅支持 HTTP（不支持 HTTPS）。");
            }
        });
    }
}

impl From<AnyError> for crate::net::Error {
    fn from(value: AnyError) -> Self {
        crate::net::Error::Other(value.to_string())
    }
}
