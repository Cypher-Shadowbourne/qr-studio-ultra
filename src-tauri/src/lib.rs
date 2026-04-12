use fast_qr::convert::image::ImageBuilder;
use fast_qr::qr::QRBuilder;
use image::{Rgba, RgbaImage, ImageFormat};
use std::io::Cursor;
use base64::{engine::general_purpose, Engine as _};
use tauri_plugin_shell::ShellExt;
use tauri::Manager;

#[cfg(target_os = "android")]
use jni::objects::{JClass, JObject, JString, JValue};
#[cfg(target_os = "android")]
use std::sync::mpsc;
#[cfg(target_os = "android")]
use std::time::Duration;

fn hex_to_rgba(hex: &str) -> Rgba<u8> {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 3 {
        let r = u8::from_str_radix(&hex[0..1], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[1..2], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[2..3], 16).unwrap_or(0);
        return Rgba([r * 17, g * 17, b * 17, 255]);
    }
    if hex.len() != 6 { return Rgba([0, 0, 0, 255]); }
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    Rgba([r, g, b, 255])
}

fn point_in_shape(shape: &str, x: f32, y: f32, size: f32) -> bool {
    let half = size / 2.0;
    let dx = x - half;
    let dy = y - half;
    let adx = dx.abs();
    let ady = dy.abs();

    match shape {
        "circle" => dx * dx + dy * dy <= half * half,
        "diamond" => adx + ady <= half,
        "octagon" => {
            let side = size * 0.28;
            adx <= half && ady <= half && (adx + ady) <= (half + side)
        }
        "rounded" => {
            let radius = size * 0.2;
            let inner = half - radius;
            if adx <= inner || ady <= inner {
                true
            } else {
                let cx = adx - inner;
                let cy = ady - inner;
                cx * cx + cy * cy <= radius * radius
            }
        }
        _ => adx <= half && ady <= half,
    }
}

fn blend_pixel(dst: &mut Rgba<u8>, src: Rgba<u8>) {
    let alpha = src[3] as f32 / 255.0;
    let inv_alpha = 1.0 - alpha;

    dst[0] = (src[0] as f32 * alpha + dst[0] as f32 * inv_alpha).round() as u8;
    dst[1] = (src[1] as f32 * alpha + dst[1] as f32 * inv_alpha).round() as u8;
    dst[2] = (src[2] as f32 * alpha + dst[2] as f32 * inv_alpha).round() as u8;
    dst[3] = ((src[3] as f32) + dst[3] as f32 * inv_alpha).round().clamp(0.0, 255.0) as u8;
}

#[tauri::command]
fn save_to_path(b64: String, path: String) -> Result<String, String> {
    let clean_b64 = if b64.contains(',') { b64.split(',').nth(1).unwrap_or(&b64) } else { &b64 };
    let decoded = general_purpose::STANDARD.decode(clean_b64).map_err(|e| e.to_string())?;

    std::fs::write(&path, decoded).map_err(|e| e.to_string())?;
    
    Ok(format!("Saved successfully to: {}", path))
}

#[derive(serde::Serialize)]
struct MobileSaveResult {
    message: String,
}

#[cfg(target_os = "android")]
fn save_to_android_gallery(
    app: &tauri::AppHandle,
    decoded: &[u8],
    filename: &str,
    mime_type: &str,
) -> Result<String, String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Could not access the main app window for Android gallery save.".to_string())?;

    let (tx, rx) = mpsc::channel();
    let bytes = decoded.to_vec();
    let filename = filename.to_string();
    let mime_type = mime_type.to_string();

    window
        .with_webview(move |webview| {
            webview.jni_handle().exec(move |env, activity, _| {
                let result = (|| -> Result<String, String> {
                    let class_name = env
                        .new_string("com.cypher.qrstudioultra.MediaStoreSaver")
                        .map_err(|e| format!("Could not prepare Android saver class name: {e}"))?;
                    let saver_class = env
                        .call_method(
                            activity,
                            "getAppClass",
                            "(Ljava/lang/String;)Ljava/lang/Class;",
                            &[JValue::Object(&JObject::from(class_name))],
                        )
                        .map_err(|e| format!("Could not resolve Android media saver class: {e}"))?
                        .l()
                        .map_err(|e| format!("Could not find Android media saver: {e}"))?;
                    let saver_class = JClass::from(saver_class);
                    let saver_instance = env
                        .get_static_field(&saver_class, "INSTANCE", "Lcom/cypher/qrstudioultra/MediaStoreSaver;")
                        .map_err(|e| format!("Could not access Android media saver singleton: {e}"))?
                        .l()
                        .map_err(|e| format!("Android media saver singleton was invalid: {e}"))?;

                    let bytes_array = env
                        .byte_array_from_slice(&bytes)
                        .map_err(|e| format!("Could not prepare image bytes for Android save: {e}"))?;
                    let filename_java = env
                        .new_string(&filename)
                        .map_err(|e| format!("Could not prepare Android filename: {e}"))?;
                    let mime_java = env
                        .new_string(&mime_type)
                        .map_err(|e| format!("Could not prepare Android MIME type: {e}"))?;

                    let bytes_obj = JObject::from(bytes_array);
                    let filename_obj = JObject::from(filename_java);
                    let mime_obj = JObject::from(mime_java);

                    let saved_msg = env
                        .call_method(
                            &saver_instance,
                            "saveQrImage",
                            "(Landroid/app/Activity;[BLjava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                            &[
                                JValue::Object(activity),
                                JValue::Object(&bytes_obj),
                                JValue::Object(&filename_obj),
                                JValue::Object(&mime_obj),
                            ],
                        )
                        .map_err(|e| format!("Android gallery save failed: {e}"))?
                        .l()
                        .map_err(|e| format!("Android gallery save returned an invalid result: {e}"))?;

                    let saved_msg = env
                        .get_string(&JString::from(saved_msg))
                        .map_err(|e| format!("Could not read Android gallery save message: {e}"))?
                        .to_string_lossy()
                        .into_owned();

                    Ok(saved_msg)
                })();

                let _ = tx.send(result);
            });
        })
        .map_err(|e| format!("Could not schedule Android gallery save: {e}"))?;

    rx.recv_timeout(Duration::from_secs(15))
        .map_err(|_| "Android gallery save timed out before returning a result.".to_string())?
}

#[cfg(target_os = "android")]
fn run_android_media_action(app: &tauri::AppHandle, method_name: &'static str) -> Result<String, String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Could not access the main app window for Android media action.".to_string())?;

    let (tx, rx) = mpsc::channel();

    window
        .with_webview(move |webview| {
            webview.jni_handle().exec(move |env, activity, _| {
                let result = (|| -> Result<String, String> {
                    let class_name = env
                        .new_string("com.cypher.qrstudioultra.MediaStoreSaver")
                        .map_err(|e| format!("Could not prepare Android saver class name: {e}"))?;
                    let saver_class = env
                        .call_method(
                            activity,
                            "getAppClass",
                            "(Ljava/lang/String;)Ljava/lang/Class;",
                            &[JValue::Object(&JObject::from(class_name))],
                        )
                        .map_err(|e| format!("Could not resolve Android media saver class: {e}"))?
                        .l()
                        .map_err(|e| format!("Could not find Android media saver: {e}"))?;
                    let saver_class = JClass::from(saver_class);
                    let saver_instance = env
                        .get_static_field(&saver_class, "INSTANCE", "Lcom/cypher/qrstudioultra/MediaStoreSaver;")
                        .map_err(|e| format!("Could not access Android media saver singleton: {e}"))?
                        .l()
                        .map_err(|e| format!("Android media saver singleton was invalid: {e}"))?;

                    let result = env
                        .call_method(
                            &saver_instance,
                            method_name,
                            "(Landroid/app/Activity;)Ljava/lang/String;",
                            &[JValue::Object(activity)],
                        )
                        .map_err(|e| format!("Android media action failed: {e}"))?
                        .l()
                        .map_err(|e| format!("Android media action returned an invalid result: {e}"))?;

                    let result = env
                        .get_string(&JString::from(result))
                        .map_err(|e| format!("Could not read Android media action result: {e}"))?
                        .to_string_lossy()
                        .into_owned();

                    Ok(result)
                })();

                let _ = tx.send(result);
            });
        })
        .map_err(|e| format!("Could not schedule Android media action: {e}"))?;

    rx.recv_timeout(Duration::from_secs(15))
        .map_err(|_| "Android media action timed out before returning a result.".to_string())?
}

// Save into a user-visible folder on mobile whenever possible.
#[tauri::command]
async fn save_to_device(app: tauri::AppHandle, b64: String, format: String) -> Result<MobileSaveResult, String> {
    let clean_b64 = if b64.contains(',') { b64.split(',').nth(1).unwrap_or(&b64) } else { &b64 };
    let decoded = general_purpose::STANDARD.decode(clean_b64).map_err(|e| e.to_string())?;

    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let ext = if format.to_lowercase() == "jpg" { "jpg" } else { "png" };
    let filename = format!("QR_Studio_{}.{}", timestamp, ext);
    let mime_type = if ext == "jpg" { "image/jpeg" } else { "image/png" };

    #[cfg(target_os = "android")]
    {
        let message = tauri::async_runtime::spawn_blocking(move || {
            save_to_android_gallery(&app, &decoded, &filename, mime_type)
        })
        .await
        .map_err(|e| format!("Android save task failed: {e}"))??;

        return Ok(MobileSaveResult { message });
    }

    #[cfg(not(target_os = "android"))]
    {
        let target_dir = app.path().picture_dir()
            .or_else(|_| app.path().download_dir())
            .map_err(|e| format!("Could not locate a save folder: {}", e))?
            .join("QR Studio Ultra");

        println!("Final target directory: {}", target_dir.display());

        if !target_dir.exists() {
            std::fs::create_dir_all(&target_dir).map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        let file_path = target_dir.join(&filename);
        std::fs::write(&file_path, decoded).map_err(|e| format!("Failed to write file {}: {}", file_path.display(), e))?;
        
        let msg = format!("Saved to: {}", file_path.display());

        Ok(MobileSaveResult { message: msg })
    }
}

#[tauri::command]
async fn open_last_saved_image(app: tauri::AppHandle) -> Result<String, String> {
    #[cfg(target_os = "android")]
    {
        return tauri::async_runtime::spawn_blocking(move || {
            run_android_media_action(&app, "openLastSavedImage")
        })
        .await
        .map_err(|e| format!("Android open action failed: {e}"))?;
    }

    #[cfg(not(target_os = "android"))]
    {
        Err("Open last saved image is only implemented on Android.".to_string())
    }
}

#[tauri::command]
async fn share_last_saved_image(app: tauri::AppHandle) -> Result<String, String> {
    #[cfg(target_os = "android")]
    {
        return tauri::async_runtime::spawn_blocking(move || {
            run_android_media_action(&app, "shareLastSavedImage")
        })
        .await
        .map_err(|e| format!("Android share action failed: {e}"))?;
    }

    #[cfg(not(target_os = "android"))]
    {
        Err("Share last saved image is only implemented on Android.".to_string())
    }
}

#[allow(deprecated)] 
#[tauri::command]
fn open_external_link(app: tauri::AppHandle, url: String) -> Result<String, String> {
    match app.shell().open(url, None) {
        Ok(_) => Ok("Opened successfully".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[derive(serde::Deserialize)]
struct QrOptions {
    data: String,
    color1: String,
    color2: String,
    #[serde(rename = "bgColor")]
    bg_color: String,
    #[serde(rename = "eyeOut")]
    eye_out: String,
    #[serde(rename = "eyeIn")]
    eye_in: String,
    #[serde(rename = "fillType")]
    fill_type: String,
    #[serde(rename = "mainShape")]
    main_shape: String,
    #[serde(rename = "bgShape")]
    bg_shape: String,
    #[serde(rename = "eyeShape")]
    eye_shape: String,
    #[serde(rename = "logoB64")]
    logo_b64: Option<String>
}

#[tauri::command]
fn generate_ultra_qr(options: QrOptions) -> Result<String, String> {
    let data = options.data;
    let color1 = options.color1;
    let color2 = options.color2;
    let bg_color = options.bg_color;
    let eye_out = options.eye_out;
    let eye_in = options.eye_in;
    let fill_type = options.fill_type;
    let main_shape = options.main_shape;
    let bg_shape = options.bg_shape;
    let eye_shape = options.eye_shape;
    let logo_b64 = options.logo_b64;

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

            let in_tl = (4..11).contains(&mod_x) && (4..11).contains(&mod_y);
            let in_tr = (m_size - 11..m_size - 4).contains(&mod_x) && (4..11).contains(&mod_y);
            let in_bl = (4..11).contains(&mod_x) && (m_size - 11..m_size - 4).contains(&mod_y);
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
                    else if (2.5..=3.5).contains(&dist) { paint = true; color_to_use = e_out; }
                } 
                else if eye_shape == "diamond" {
                    let dist = dx.abs() + dy.abs();
                    if dist <= 2.0 { paint = true; color_to_use = e_in; }
                    else if (3.0..=5.0).contains(&dist) { paint = true; color_to_use = e_out; }
                }
                else if eye_shape == "rounded" {
                    let adx = dx.abs();
                    let ady = dy.abs();
                    
                    let mut in_inner = false;
                    if adx <= 1.5 && ady <= 1.5 {
                        if adx < 1.0 || ady < 1.0 { in_inner = true; }
                        else {
                            let cx = adx - 1.0; let cy = ady - 1.0;
                            if cx*cx + cy*cy <= 0.25 { in_inner = true; }
                        }
                    }

                    let mut in_outer = false;
                    if adx <= 3.5 && ady <= 3.5 {
                        if adx < 2.5 || ady < 2.5 { in_outer = true; }
                        else {
                            let cx = adx - 2.5; let cy = ady - 2.5;
                            if cx*cx + cy*cy <= 1.0 { in_outer = true; }
                        }
                    }

                    if in_inner { paint = true; color_to_use = e_in; }
                    else if in_outer && (adx >= 2.5 || ady >= 2.5) {
                        paint = true; color_to_use = e_out;
                    }
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
                    } else if main_shape == "diamond" {
                        let local_x = mod_x_f32 % 1.0;
                        let local_y = mod_y_f32 % 1.0;
                        let dx = (local_x - 0.5).abs();
                        let dy = (local_y - 0.5).abs();
                        if dx + dy <= 0.5 { paint = true; }
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
                let resized_logo = logo_img
                    .resize(logo_size, logo_size, image::imageops::FilterType::Lanczos3)
                    .to_rgba8();
                
                let x_offset = (width - resized_logo.width()) / 2;
                let y_offset = (height - resized_logo.height()) / 2;

                let padding = 12u32;

                let padded_size = resized_logo.width() + padding * 2;
                let padded_x = x_offset.saturating_sub(padding);
                let padded_y = y_offset.saturating_sub(padding);

                for local_x in 0..padded_size {
                    for local_y in 0..padded_size {
                        let target_x = padded_x + local_x;
                        let target_y = padded_y + local_y;
                        if target_x >= width || target_y >= height {
                            continue;
                        }
                        if point_in_shape(&bg_shape, local_x as f32, local_y as f32, padded_size as f32) {
                            img.put_pixel(target_x, target_y, Rgba([255, 255, 255, 255]));
                        }
                    }
                }

                for local_x in 0..resized_logo.width() {
                    for local_y in 0..resized_logo.height() {
                        if !point_in_shape(&bg_shape, local_x as f32, local_y as f32, resized_logo.width() as f32) {
                            continue;
                        }

                        let target_x = x_offset + local_x;
                        let target_y = y_offset + local_y;
                        if target_x >= width || target_y >= height {
                            continue;
                        }

                        let src = *resized_logo.get_pixel(local_x, local_y);
                        if src[3] == 0 {
                            continue;
                        }

                        let dst = img.get_pixel_mut(target_x, target_y);
                        blend_pixel(dst, src);
                    }
                }
            }
        }
    }

    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, ImageFormat::Png).map_err(|e| e.to_string())?;
    Ok(format!("data:image/png;base64,{}", general_purpose::STANDARD.encode(buffer.into_inner())))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init());

    #[cfg(any(target_os = "android", target_os = "ios"))]
    let builder = builder.plugin(tauri_plugin_barcode_scanner::init());

    builder
        .invoke_handler(tauri::generate_handler![
            generate_ultra_qr, 
            save_to_device, 
            save_to_path,
            open_external_link,
            open_last_saved_image,
            share_last_saved_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
