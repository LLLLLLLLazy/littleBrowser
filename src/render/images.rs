use crate::net::{Error, Result};
use std::io::Cursor;
use std::time::Duration;

pub enum DecodedImage {
    Static(egui::ColorImage),
    AnimatedGif { frames: Vec<egui::ColorImage>, durations: Vec<Duration> },
}

pub fn decode_image(bytes: &[u8]) -> Result<DecodedImage> {
    if looks_like_gif(bytes) {
        if let Ok(anim) = decode_gif_animation(bytes) {
            if anim.frames.len() >= 2 {
                return Ok(DecodedImage::AnimatedGif {
                    frames: anim.frames,
                    durations: anim.durations,
                });
            }
            if let Some(first) = anim.frames.into_iter().next() {
                return Ok(DecodedImage::Static(first));
            }
        }
        // Fall through to static decode if gif animation decode fails.
    }

    Ok(DecodedImage::Static(decode_first_frame(bytes)?))
}

fn decode_first_frame(bytes: &[u8]) -> Result<egui::ColorImage> {
    let img = image::load_from_memory(bytes).map_err(|e| Error::Other(e.to_string()))?;
    let rgba = img.to_rgba8();
    Ok(rgba_to_color_image(rgba.width(), rgba.height(), &rgba.into_raw()))
}

struct GifAnim {
    frames: Vec<egui::ColorImage>,
    durations: Vec<Duration>,
}

fn decode_gif_animation(bytes: &[u8]) -> Result<GifAnim> {
    use image::AnimationDecoder;

    let decoder = image::codecs::gif::GifDecoder::new(Cursor::new(bytes))
        .map_err(|e| Error::Other(e.to_string()))?;

    let frames = decoder
        .into_frames()
        .collect_frames()
        .map_err(|e| Error::Other(e.to_string()))?;

    let mut out_frames = Vec::with_capacity(frames.len());
    let mut out_durations = Vec::with_capacity(frames.len());

    for f in frames {
        let delay = f.delay();
        let (numer_ms, denom) = delay.numer_denom_ms();
        let ms = if denom == 0 {
            100
        } else {
            // duration in ms = numer/denom
            let v = (numer_ms as u64 + denom as u64 - 1) / denom as u64;
            v.max(10)
        };
        out_durations.push(Duration::from_millis(ms));

        let buf = f.into_buffer();
        out_frames.push(rgba_to_color_image(buf.width(), buf.height(), &buf.into_raw()));
    }

    Ok(GifAnim {
        frames: out_frames,
        durations: out_durations,
    })
}

fn rgba_to_color_image(width: u32, height: u32, raw: &[u8]) -> egui::ColorImage {
    let size = [width as usize, height as usize];
    let pixels = raw
        .chunks_exact(4)
        .map(|px| egui::Color32::from_rgba_unmultiplied(px[0], px[1], px[2], px[3]))
        .collect::<Vec<_>>();
    egui::ColorImage { size, pixels }
}

fn looks_like_gif(bytes: &[u8]) -> bool {
    bytes.len() >= 6 && (&bytes[..6] == b"GIF87a" || &bytes[..6] == b"GIF89a")
}
