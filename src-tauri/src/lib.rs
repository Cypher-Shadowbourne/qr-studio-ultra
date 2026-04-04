use fast_qr::convert::image::ImageBuilder;
use fast_qr::qr::QRBuilder;
use image::{Rgba, RgbaImage, ImageFormat};
use std::io::Cursor;
use base64::{engine::general_purpose, Engine as _};
use tauri_plugin_shell::ShellExt;

fn hex_to_rgba(hex: &str) -> Rgba<u8> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 { return Rgba([0, 0, 0, 255]); }
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    Rgba([r, g, b, 255])
}

// UPGRADED: Saves to Pictures folder and handles formats
#[tauri::command]
fn save_to_device(b64: String, format: String) -> Result<String, String> {
    let clean_b64 = if b64.contains(',') { b64.split(',').nth(1).unwrap_or(&b64) } else { &b64 };
    let decoded = general_purpose::STANDARD.decode(clean_b64).map_err(|e| e.to_string())?;

    // Target the Pictures folder so the Android Gallery sees it immediately
    let android_path = std::path::Path::new("/storage/emulated/0/Pictures");
    
    if android_path.exists() {
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let ext = if format.to_lowercase() == "jpg" { "jpg" } else { "png" };
        let file_path = android_path.join(format!("QR_Studio_{}.{}", timestamp, ext));
        
        std::fs::write(&file_path, decoded).map_err(|e| e.to_string())?;
        return Ok(format!("Saved to Gallery (Pictures/QR_Studio_{}.{})!", timestamp, ext));
    }

    Err("Pictures folder not found. Could not save.".to_string())
}

#[allow(deprecated)] 
#[tauri::command]
fn open_external_link(app: tauri::AppHandle, url: String) -> Result<String, String> {
    match app.shell().open(url, None) {
        Ok(_) => Ok("Opened successfully".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn generate_ultra_qr(
    data: String,
    color1: String,
    color2: String,
    bg_color: String,
    eye_out: String,
    eye_in: String,
    fill_type: String,
    main_shape: String, 
    eye_shape: String,
    logo_b64: Option<String>
) -> Result<String, String> {
    let qrcode = QRBuilder::new(data).ecl(fast_qr::ECL::H).build().map_err(|e| e.to_string())?;
    
    let pixmap = ImageBuilder::default().fit_width(600).to_pixmap(&qrcode);

    let width = pixmap.width();
    let height = pixmap.height();
    let mut img = RgbaImage::new(width, height);

    let c1 = hex_to_rgba(&color1);
    let c2 = hex_to_rgba(&color2);
    let bg = hex_to_rgba(&bg_color);
    let e_out = hex_to_rgba(&eye_out);
    let e_in = hex_to_rgba(&eye_in);

    let modules = qrcode.size; 
    let margin = 4.0;
    let total_modules = modules as f32 + (margin * 2.0);

    for x in 0..width {
        for y in 0..height {
            let mod_x_f32 = (x as f32 / width as f32) * total_modules;
            let mod_y_f32 = (y as f32 / height as f32) * total_modules;
            let mod_x = mod_x_f32 as u32;
            let mod_y = mod_y_f32 as u32;
            let m_size = total_modules as u32;

            let in_tl = mod_x >= 4 && mod_x < 11 && mod_y >= 4 && mod_y < 11;
            let in_tr = mod_x >= m_size - 11 && mod_x < m_size - 4 && mod_y >= 4 && mod_y < 11;
            let in_bl = mod_x >= 4 && mod_x < 11 && mod_y >= m_size - 11 && mod_y < m_size - 4;
            let is_eye = in_tl || in_tr || in_bl;

            let mut paint = false;
            let mut color_to_use = bg;

            if is_eye {
                let exact_rel_x = if in_tl || in_bl { mod_x_f32 - 4.0 } else { mod_x_f32 - (m_size as f32 - 11.0) };
                let exact_rel_y = if in_tl || in_tr { mod_y_f32 - 4.0 } else { mod_y_f32 - (m_size as f32 - 11.0) };
                
                let dx = exact_rel_x - 3.5;
                let dy = exact_rel_y - 3.5;

                if eye_shape == "circle" {
                    let dist = (dx*dx + dy*dy).sqrt();
                    if dist <= 1.5 { paint = true; color_to_use = e_in; }
                    else if dist >= 2.2 && dist <= 3.5 { paint = true; color_to_use = e_out; }
                } 
                else if eye_shape == "diamond" {
                    let dist = dx.abs() + dy.abs();
                    if dist <= 1.8 { paint = true; color_to_use = e_in; }
                    else if dist >= 2.5 && dist <= 4.0 { paint = true; color_to_use = e_out; }
                }
                else {
                    let pixel = pixmap.pixel(x, y).unwrap();
                    if pixel.red() < 128 {
                        paint = true;
                        if dx.abs() <= 1.5 && dy.abs() <= 1.5 { color_to_use = e_in; }
                        else { color_to_use = e_out; }
                    }
                }
            } else {
                let pixel = pixmap.pixel(x, y).unwrap();
                if pixel.red() < 128 {
                    if main_shape == "circle" {
                        let local_x = mod_x_f32 % 1.0;
                        let local_y = mod_y_f32 % 1.0;
                        let dx = local_x - 0.5;
                        let dy = local_y - 0.5;
                        if dx*dx + dy*dy <= 0.20 { paint = true; }
                    } else if main_shape == "rounded" {
                        let local_x = mod_x_f32 % 1.0;
                        let local_y = mod_y_f32 % 1.0;
                        let dx = (local_x - 0.5).abs();
                        let dy = (local_y - 0.5).abs();
                        if dx < 0.3 || dy < 0.3 {
                            paint = true;
                        } else {
                            let cx = dx - 0.3;
                            let cy = dy - 0.3;
                            if cx*cx + cy*cy <= 0.04 { paint = true; }
                        }
                    } else {
                        paint = true;
                    }

                    if paint {
                        if fill_type == "Linear" {
                            let factor = y as f32 / height as f32;
                            let r = ((1.0 - factor) * c1[0] as f32 + factor * c2[0] as f32) as u8;
                            let g = ((1.0 - factor) * c1[1] as f32 + factor * c2[1] as f32) as u8;
                            let b = ((1.0 - factor) * c1[2] as f32 + factor * c2[2] as f32) as u8;
                            color_to_use = Rgba([r, g, b, 255]);
                        } else {
                            color_to_use = c1;
                        }
                    }
                }
            }

            if paint { img.put_pixel(x, y, color_to_use); } 
            else { img.put_pixel(x, y, bg); }
        }
    }

    if let Some(logo_str) = logo_b64 {
        let clean_b64 = if logo_str.contains(',') { logo_str.split(',').nth(1).unwrap_or(&logo_str) } else { &logo_str };
        
        if let Ok(decoded) = general_purpose::STANDARD.decode(clean_b64) {
            if let Ok(logo_img) = image::load_from_memory(&decoded) {
                let logo_size = (width as f32 * 0.22) as u32; 
                let resized_logo = logo_img.resize(logo_size, logo_size, image::imageops::FilterType::Lanczos3);
                
                let x_offset = (width - resized_logo.width()) / 2;
                let y_offset = (height - resized_logo.height()) / 2;

                let padding = 8u32; 
                
                // CRITICAL FIX: Safe lower AND upper bounds using saturating_sub and min()
                let start_x = x_offset.saturating_sub(padding);
                let end_x = (x_offset + resized_logo.width() + padding).min(width);
                let start_y = y_offset.saturating_sub(padding);
                let end_y = (y_offset + resized_logo.height() + padding).min(height);

                // Because the bounds are strictly enforced, we can safely loop without the inner IF statement
                for lx in start_x..end_x {
                    for ly in start_y..end_y {
                        img.put_pixel(lx, ly, Rgba([255, 255, 255, 255]));
                    }
                }
                
                image::imageops::overlay(&mut img, &resized_logo, x_offset.into(), y_offset.into());
            }
        }
    }

    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, ImageFormat::Png).map_err(|e| e.to_string())?;
    Ok(format!("data:image/png;base64,{}", general_purpose::STANDARD.encode(buffer.into_inner())))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init());

    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        builder = builder.plugin(tauri_plugin_barcode_scanner::init());
    }

    builder
        .invoke_handler(tauri::generate_handler![
            generate_ultra_qr, 
            save_to_device, 
            open_external_link
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}