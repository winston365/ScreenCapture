use arboard::{Clipboard, ImageData};
use egui::{Color32, ColorImage};
use image::{ImageBuffer, Rgba};
use std::borrow::Cow;
use std::path::Path;

/// ColorImage를 클립보드에 복사
pub fn copy_image_to_clipboard(image: &ColorImage) -> Result<(), String> {
    let mut clipboard = Clipboard::new().map_err(|e| format!("Failed to access clipboard: {}", e))?;

    // ColorImage를 RGBA 버퍼로 변환
    let width = image.width();
    let height = image.height();
    let mut rgba_data = Vec::with_capacity(width * height * 4);

    for pixel in &image.pixels {
        rgba_data.push(pixel.r());
        rgba_data.push(pixel.g());
        rgba_data.push(pixel.b());
        rgba_data.push(pixel.a());
    }

    let img_data = ImageData {
        width,
        height,
        bytes: Cow::Owned(rgba_data),
    };

    clipboard
        .set_image(img_data)
        .map_err(|e| format!("Failed to copy image to clipboard: {}", e))
}

/// 텍스트를 클립보드에 복사
pub fn copy_text_to_clipboard(text: &str) -> Result<(), String> {
    let mut clipboard = Clipboard::new().map_err(|e| format!("Failed to access clipboard: {}", e))?;

    clipboard
        .set_text(text)
        .map_err(|e| format!("Failed to copy text to clipboard: {}", e))
}

/// 색상을 RGB 형식으로 복사
pub fn copy_color_rgb(color: Color32) -> Result<(), String> {
    let text = format!("rgb({}, {}, {})", color.r(), color.g(), color.b());
    copy_text_to_clipboard(&text)
}

/// 색상을 HEX 형식으로 복사
pub fn copy_color_hex(color: Color32) -> Result<(), String> {
    let text = format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b());
    copy_text_to_clipboard(&text)
}

/// 색상을 CMYK 형식으로 복사
pub fn copy_color_cmyk(color: Color32) -> Result<(), String> {
    let r = color.r() as f32 / 255.0;
    let g = color.g() as f32 / 255.0;
    let b = color.b() as f32 / 255.0;

    let k = 1.0 - r.max(g).max(b);
    let c = if k < 1.0 { (1.0 - r - k) / (1.0 - k) } else { 0.0 };
    let m = if k < 1.0 { (1.0 - g - k) / (1.0 - k) } else { 0.0 };
    let y = if k < 1.0 { (1.0 - b - k) / (1.0 - k) } else { 0.0 };

    let text = format!(
        "cmyk({:.0}%, {:.0}%, {:.0}%, {:.0}%)",
        c * 100.0,
        m * 100.0,
        y * 100.0,
        k * 100.0
    );
    copy_text_to_clipboard(&text)
}

/// ColorImage를 파일로 저장
pub fn save_image_to_file(image: &ColorImage, path: &Path) -> Result<(), String> {
    let width = image.width() as u32;
    let height = image.height() as u32;

    // ColorImage를 image crate의 ImageBuffer로 변환
    let mut img_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);

    for (i, pixel) in image.pixels.iter().enumerate() {
        let x = (i % image.width()) as u32;
        let y = (i / image.width()) as u32;
        img_buffer.put_pixel(x, y, Rgba([pixel.r(), pixel.g(), pixel.b(), pixel.a()]));
    }

    // 파일로 저장
    img_buffer
        .save(path)
        .map_err(|e| format!("Failed to save image: {}", e))
}

/// image crate의 ImageBuffer를 ColorImage로 변환
pub fn image_buffer_to_color_image(buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> ColorImage {
    let width = buffer.width() as usize;
    let height = buffer.height() as usize;

    let pixels: Vec<Color32> = buffer
        .pixels()
        .map(|p| Color32::from_rgba_premultiplied(p[0], p[1], p[2], p[3]))
        .collect();

    ColorImage {
        size: [width, height],
        pixels,
    }
}

/// 파일 대화상자를 통해 저장 경로 선택
pub fn show_save_dialog() -> Option<std::path::PathBuf> {
    use std::process::Command;

    // Windows 파일 대화상자는 실제 환경에서만 작동
    // 여기서는 기본 경로 반환
    let home = std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("screenshot_{}.png", timestamp);
    Some(std::path::PathBuf::from(home).join("Pictures").join(filename))
}
