use std::path::{Path, PathBuf};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FontInstallResult {
    pub installed: bool,
    pub source: Option<String>,
}

pub fn install_cjk_fonts(ctx: &egui::Context) -> FontInstallResult {
    let mut fonts = egui::FontDefinitions::default();

    // Prefer user-provided font files in repo.
    let candidates = candidate_font_paths();

    for path in candidates {
        if let Some(bytes) = try_read_font_file(&path) {
            let name = "cjk".to_string();
            fonts
                .font_data
                .insert(name.clone(), egui::FontData::from_owned(bytes));

            if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
                family.insert(0, name.clone());
            }
            if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
                family.insert(0, name);
            }

            ctx.set_fonts(fonts);
            return FontInstallResult {
                installed: true,
                source: Some(path.display().to_string()),
            };
        }
    }

    FontInstallResult {
        installed: false,
        source: None,
    }
}

fn try_read_font_file(path: &Path) -> Option<Vec<u8>> {
    if !path.exists() {
        return None;
    }
    // egui currently expects TTF/OTF bytes; TTC collections are typically not supported.
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    let ext = ext.to_ascii_lowercase();
    if ext != "ttf" && ext != "otf" {
        return None;
    }
    std::fs::read(path).ok()
}

fn candidate_font_paths() -> Vec<PathBuf> {
    let mut v = Vec::new();

    // Repo-relative (most reliable)
    v.push(PathBuf::from("assets/fonts/NotoSansSC-Regular.otf"));
    v.push(PathBuf::from("assets/fonts/NotoSansSC-Regular.ttf"));
    v.push(PathBuf::from("assets/fonts/NotoSansCJKsc-Regular.otf"));
    v.push(PathBuf::from("assets/fonts/NotoSansCJKsc-Regular.ttf"));

    // Common Linux locations (may or may not exist)
    #[cfg(target_os = "linux")]
    {
        v.push(PathBuf::from("/usr/share/fonts/opentype/noto/NotoSansSC-Regular.otf"));
        v.push(PathBuf::from("/usr/share/fonts/truetype/noto/NotoSansSC-Regular.ttf"));
        v.push(PathBuf::from("/usr/share/fonts/opentype/noto/NotoSansCJKsc-Regular.otf"));
        v.push(PathBuf::from("/usr/share/fonts/truetype/noto/NotoSansCJKsc-Regular.ttf"));
        v.push(PathBuf::from("/usr/share/fonts/truetype/wqy/wqy-microhei.ttf"));
        v.push(PathBuf::from("/usr/share/fonts/truetype/wqy/wqy-zenhei.ttf"));
    }

    // Common macOS locations (ttf/otf only; most system CJK fonts are .ttc)
    #[cfg(target_os = "macos")]
    {
        v.push(PathBuf::from("/Library/Fonts/NotoSansSC-Regular.otf"));
        v.push(PathBuf::from("/Library/Fonts/NotoSansSC-Regular.ttf"));
        v.push(PathBuf::from("/System/Library/Fonts/Supplemental/Arial Unicode.ttf"));
    }

    // Common Windows locations
    #[cfg(target_os = "windows")]
    {
        v.push(PathBuf::from(r"C:\Windows\Fonts\msyh.ttf"));
        v.push(PathBuf::from(r"C:\Windows\Fonts\simhei.ttf"));
    }

    v
}
