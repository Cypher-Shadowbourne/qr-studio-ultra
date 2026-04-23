use fast_qr::qr::QRBuilder;
use image::{Rgba, RgbaImage, ImageFormat};
use qr_code_styling::types::DotType;
use qr_code_styling::{
    Color, CornerDotType, CornerSquareType, CornersDotOptions, CornersSquareOptions, DotsOptions,
    ErrorCorrectionLevel, Gradient, ImageOptions, QRCodeStyling, QROptions, ShapeType,
};
use std::fmt::Write as _;
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

#[allow(dead_code)]
fn point_in_shape(shape: &str, x: f32, y: f32, size: f32) -> bool {
    point_in_shape_rect(shape, x, y, size, size)
}

fn point_in_shape_rect(shape: &str, x: f32, y: f32, width: f32, height: f32) -> bool {
    let half_w = width / 2.0;
    let half_h = height / 2.0;
    let dx = x - half_w;
    let dy = y - half_h;
    let ndx = if half_w > 0.0 { dx / half_w } else { 0.0 };
    let ndy = if half_h > 0.0 { dy / half_h } else { 0.0 };
    let adx = ndx.abs();
    let ady = ndy.abs();

    match shape {
        "circle" => ndx * ndx + ndy * ndy <= 1.0,
        "diamond" => adx + ady <= 1.0,
        "octagon" => {
            let side = 0.56;
            adx <= 1.0 && ady <= 1.0 && (adx + ady) <= (1.0 + side)
        }
        "rounded" => {
            let radius = 0.2;
            let inner_x = 1.0 - radius;
            let inner_y = 1.0 - radius;
            if adx <= inner_x || ady <= inner_y {
                true
            } else {
                let cx = adx - inner_x;
                let cy = ady - inner_y;
                cx * cx + cy * cy <= radius * radius
            }
        }
        _ => adx <= 1.0 && ady <= 1.0,
    }
}

fn trim_transparent_logo(logo: &RgbaImage) -> RgbaImage {
    let mut min_x = logo.width();
    let mut min_y = logo.height();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut found = false;

    for y in 0..logo.height() {
        for x in 0..logo.width() {
            if logo.get_pixel(x, y)[3] > 8 {
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
                found = true;
            }
        }
    }

    if !found {
        return logo.clone();
    }

    image::imageops::crop_imm(
        logo,
        min_x,
        min_y,
        max_x - min_x + 1,
        max_y - min_y + 1,
    )
    .to_image()
}

fn fit_contain_dimensions(source_width: u32, source_height: u32, max_size: u32) -> (u32, u32) {
    if source_width == 0 || source_height == 0 || max_size == 0 {
        return (max_size.max(1), max_size.max(1));
    }

    if source_width >= source_height {
        let width = max_size;
        let height = ((max_size as f32) * (source_height as f32 / source_width as f32))
            .round()
            .max(1.0) as u32;
        (width, height)
    } else {
        let height = max_size;
        let width = ((max_size as f32) * (source_width as f32 / source_height as f32))
            .round()
            .max(1.0) as u32;
        (width, height)
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

fn gradient_color_at(
    c1: Rgba<u8>,
    c2: Rgba<u8>,
    c3: Rgba<u8>,
    c4: Option<Rgba<u8>>,
    factor: f32,
) -> Rgba<u8> {
    let factor = factor.clamp(0.0, 1.0);
    let (start, end, local_factor) = if let Some(c4) = c4 {
        if factor <= 1.0 / 3.0 {
            (c1, c2, factor * 3.0)
        } else if factor <= 2.0 / 3.0 {
            (c2, c3, (factor - 1.0 / 3.0) * 3.0)
        } else {
            (c3, c4, (factor - 2.0 / 3.0) * 3.0)
        }
    } else if factor <= 0.5 {
        (c1, c2, factor * 2.0)
    } else {
        (c2, c3, (factor - 0.5) * 2.0)
    };

    let r = ((1.0 - local_factor) * start[0] as f32 + local_factor * end[0] as f32) as u8;
    let g = ((1.0 - local_factor) * start[1] as f32 + local_factor * end[1] as f32) as u8;
    let b = ((1.0 - local_factor) * start[2] as f32 + local_factor * end[2] as f32) as u8;
    Rgba([r, g, b, 255])
}

#[tauri::command]
fn save_to_path(b64: String, path: String) -> Result<String, String> {
    let clean_b64 = strip_data_url_prefix(&b64);
    let decoded = general_purpose::STANDARD.decode(clean_b64).map_err(|e| e.to_string())?;

    std::fs::write(&path, decoded).map_err(|e| e.to_string())?;
    
    Ok(format!("Saved successfully to: {}", path))
}

#[tauri::command]
fn save_svg_to_path(options: QrOptions, path: String) -> Result<String, String> {
    let svg = build_svg_qr(&options)?;
    std::fs::write(&path, svg).map_err(|e| e.to_string())?;

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

#[cfg(target_os = "android")]
fn print_android_image(
    app: &tauri::AppHandle,
    decoded: &[u8],
    title: &str,
    mime_type: &str,
) -> Result<String, String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Could not access the main app window for Android printing.".to_string())?;

    let (tx, rx) = mpsc::channel();
    let bytes = decoded.to_vec();
    let title = title.to_string();
    let mime_type = mime_type.to_string();

    window
        .with_webview(move |webview| {
            webview.jni_handle().exec(move |env, activity, _| {
                let result = (|| -> Result<String, String> {
                    let class_name = env
                        .new_string("com.cypher.qrstudioultra.MediaStoreSaver")
                        .map_err(|e| format!("Could not prepare Android printer class name: {e}"))?;
                    let saver_class = env
                        .call_method(
                            activity,
                            "getAppClass",
                            "(Ljava/lang/String;)Ljava/lang/Class;",
                            &[JValue::Object(&JObject::from(class_name))],
                        )
                        .map_err(|e| format!("Could not resolve Android printer class: {e}"))?
                        .l()
                        .map_err(|e| format!("Could not find Android printer class: {e}"))?;
                    let saver_class = JClass::from(saver_class);
                    let saver_instance = env
                        .get_static_field(&saver_class, "INSTANCE", "Lcom/cypher/qrstudioultra/MediaStoreSaver;")
                        .map_err(|e| format!("Could not access Android printer singleton: {e}"))?
                        .l()
                        .map_err(|e| format!("Android printer singleton was invalid: {e}"))?;

                    let bytes_array = env
                        .byte_array_from_slice(&bytes)
                        .map_err(|e| format!("Could not prepare image bytes for Android printing: {e}"))?;
                    let title_java = env
                        .new_string(&title)
                        .map_err(|e| format!("Could not prepare Android print title: {e}"))?;
                    let mime_java = env
                        .new_string(&mime_type)
                        .map_err(|e| format!("Could not prepare Android print MIME type: {e}"))?;

                    let bytes_obj = JObject::from(bytes_array);
                    let title_obj = JObject::from(title_java);
                    let mime_obj = JObject::from(mime_java);

                    let result = env
                        .call_method(
                            &saver_instance,
                            "printQrImage",
                            "(Landroid/app/Activity;[BLjava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                            &[
                                JValue::Object(activity),
                                JValue::Object(&bytes_obj),
                                JValue::Object(&title_obj),
                                JValue::Object(&mime_obj),
                            ],
                        )
                        .map_err(|e| format!("Android print failed: {e}"))?
                        .l()
                        .map_err(|e| format!("Android print returned an invalid result: {e}"))?;

                    let result = env
                        .get_string(&JString::from(result))
                        .map_err(|e| format!("Could not read Android print result: {e}"))?
                        .to_string_lossy()
                        .into_owned();

                    Ok(result)
                })();

                let _ = tx.send(result);
            });
        })
        .map_err(|e| format!("Could not schedule Android print: {e}"))?;

    rx.recv_timeout(Duration::from_secs(15))
        .map_err(|_| "Android print timed out before returning a result.".to_string())?
}

// Save into a user-visible folder on mobile whenever possible.
#[tauri::command]
async fn save_to_device(app: tauri::AppHandle, b64: String, format: String) -> Result<MobileSaveResult, String> {
    let clean_b64 = strip_data_url_prefix(&b64);
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
        let _ = mime_type;
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
async fn print_current_image(app: tauri::AppHandle, b64: String, title: String) -> Result<String, String> {
    let clean_b64 = strip_data_url_prefix(&b64);
    let decoded = general_purpose::STANDARD.decode(clean_b64).map_err(|e| e.to_string())?;

    #[cfg(target_os = "android")]
    {
        return tauri::async_runtime::spawn_blocking(move || {
            print_android_image(&app, &decoded, &title, "image/png")
        })
        .await
        .map_err(|e| format!("Android print task failed: {e}"))?;
    }

    #[cfg(not(target_os = "android"))]
    {
        let _ = app;
        let _ = decoded;
        let _ = title;
        Err("Native printing is only implemented on Android in this command.".to_string())
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
        let _ = app;
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
        let _ = app;
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

fn get_eye_coords(modules: i32) -> Vec<(i32, i32, i32, i32)> {
    vec![
        (0, 7, 0, 7),           // Top-left
        (modules - 7, modules, 0, 7), // Top-right
        (0, 7, modules - 7, modules), // Bottom-left
    ]
}

fn strip_data_url_prefix(value: &str) -> &str {
    if value.contains(',') {
        value.split(',').nth(1).unwrap_or(value)
    } else {
        value
    }
}

fn parse_svg_color(value: &str) -> Result<Color, String> {
    Color::from_hex(value).map_err(|e| e.to_string())
}

fn map_dot_type(shape: &str) -> DotType {
    match shape {
        "circle" => DotType::Dots,
        "rounded" => DotType::Rounded,
        _ => DotType::Square,
    }
}

fn map_corner_square_type(shape: &str) -> CornerSquareType {
    match shape {
        "circle" => CornerSquareType::Dot,
        "rounded" => CornerSquareType::ExtraRounded,
        _ => CornerSquareType::Square,
    }
}

fn map_corner_dot_type(shape: &str) -> CornerDotType {
    match shape {
        "square" => CornerDotType::Square,
        _ => CornerDotType::Dot,
    }
}

fn inject_svg_background(svg: String, bg_color: &str) -> String {
    if let Some(svg_start) = svg.find("<svg") {
        if let Some(relative_end) = svg[svg_start..].find('>') {
            let insert_at = svg_start + relative_end;
            let background = format!(r#"<rect width="100%" height="100%" fill="{}"/>"#, bg_color);
            let mut result = String::with_capacity(svg.len() + background.len());
            result.push_str(&svg[..insert_at + 1]);
            result.push_str(&background);
            result.push_str(&svg[insert_at + 1..]);
            return result;
        }
    }

    svg
}

#[derive(serde::Deserialize)]
struct QrOptions {
    data: String,
    color1: String,
    color2: String,
    color3: Option<String>,
    color4: Option<String>,
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
    logo_b64: Option<String>,
    #[serde(rename = "logoSize")]
    logo_size: Option<u32>,
    #[serde(rename = "logoOpacity")]
    logo_opacity: Option<u32>,
    #[serde(rename = "enableFrame")]
    enable_frame: Option<bool>,
    #[serde(rename = "frameText")]
    frame_text: Option<String>,
    #[serde(rename = "frameTextTop")]
    frame_text_top: Option<String>,
    #[serde(rename = "frameTextMode")]
    frame_text_mode: Option<String>,
    #[serde(rename = "frameTextTopMode")]
    frame_text_top_mode: Option<String>,
    #[serde(rename = "frameTextRadius")]
    frame_text_radius: Option<f32>,
    #[serde(rename = "frameTextTopRadius")]
    frame_text_top_radius: Option<f32>,
    #[serde(rename = "frameTextSpacing")]
    frame_text_spacing: Option<f32>,
    #[serde(rename = "frameTextTopSpacing")]
    frame_text_top_spacing: Option<f32>,
    #[serde(rename = "frameTextSize")]
    frame_text_size: Option<f32>,
    #[serde(rename = "frameTextTopSize")]
    frame_text_top_size: Option<f32>,
    #[serde(rename = "frameTextColor")]
    frame_text_color: Option<String>,
    #[serde(rename = "frameTextTopColor")]
    frame_text_top_color: Option<String>,
    #[serde(rename = "matchTextStyle")]
    match_text_style: Option<bool>,
    #[serde(rename = "matchTextTopStyle")]
    match_text_top_style: Option<bool>,
    #[serde(rename = "transparentTextBg")]
    transparent_text_bg: Option<bool>,
    #[serde(rename = "transparentTextTopBg")]
    transparent_text_top_bg: Option<bool>,
    #[serde(rename = "transparentFrameBg")]
    transparent_frame_bg: Option<bool>,
    #[serde(rename = "ringStyle")]
    ring_style: Option<String>,
    #[serde(rename = "ringColor")]
    ring_color: Option<String>,
    #[serde(rename = "ringColor2")]
    ring_color2: Option<String>,
    #[serde(rename = "ringColor3")]
    ring_color3: Option<String>,
    #[serde(rename = "ringColor4")]
    ring_color4: Option<String>,
    #[serde(rename = "ringUseFourthStop")]
    ring_use_fourth_stop: Option<bool>,
    #[serde(rename = "ringGradientMode")]
    ring_gradient_mode: Option<String>,
    #[serde(rename = "ringColorMode")]
    ring_color_mode: Option<String>,
    #[serde(rename = "centerOverlayMode")]
    center_overlay_mode: Option<String>,
    #[serde(rename = "centerOverlayStyle")]
    center_overlay_style: Option<String>,
    #[serde(rename = "centerOverlayColor")]
    center_overlay_color: Option<String>,
    #[serde(rename = "centerOverlayColor2")]
    center_overlay_color2: Option<String>,
    #[serde(rename = "centerOverlayColor3")]
    center_overlay_color3: Option<String>,
    #[serde(rename = "centerOverlayColor4")]
    center_overlay_color4: Option<String>,
    #[serde(rename = "centerOverlayUseFourthStop")]
    center_overlay_use_fourth_stop: Option<bool>,
    #[serde(rename = "centerOverlayGradientMode")]
    center_overlay_gradient_mode: Option<String>,
    #[serde(rename = "centerOverlayColorMode")]
    center_overlay_color_mode: Option<String>,
}

fn svg_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn strip_xml_declaration(svg: &str) -> &str {
    let trimmed = svg.trim_start();
    if trimmed.starts_with("<?xml") {
        if let Some(end) = trimmed.find("?>") {
            return trimmed[end + 2..].trim_start();
        }
    }
    trimmed
}

fn shape_path_d_rect(shape: &str, cx: f32, cy: f32, width: f32, height: f32) -> String {
    let half_w = width / 2.0;
    let half_h = height / 2.0;
    match shape {
        "circle" => {
            format!(
                "M {:.2} {:.2} A {:.2} {:.2} 0 1 0 {:.2} {:.2} A {:.2} {:.2} 0 1 0 {:.2} {:.2} Z",
                cx,
                cy - half_h,
                half_w,
                half_h,
                cx,
                cy + half_h,
                half_w,
                half_h,
                cx,
                cy - half_h
            )
        }
        "rounded" => {
            let r = width.min(height) * 0.2;
            let left = cx - half_w;
            let top = cy - half_h;
            let right = cx + half_w;
            let bottom = cy + half_h;
            format!(
                "M {:.2} {:.2} L {:.2} {:.2} Q {:.2} {:.2} {:.2} {:.2} L {:.2} {:.2} Q {:.2} {:.2} {:.2} {:.2} L {:.2} {:.2} Q {:.2} {:.2} {:.2} {:.2} L {:.2} {:.2} Q {:.2} {:.2} {:.2} {:.2} Z",
                left + r, top,
                right - r, top,
                right, top, right, top + r,
                right, bottom - r,
                right, bottom, right - r, bottom,
                left + r, bottom,
                left, bottom, left, bottom - r,
                left, top + r,
                left, top, left + r, top
            )
        }
        "diamond" => format!(
            "M {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} Z",
            cx,
            cy - half_h,
            cx + half_w,
            cy,
            cx,
            cy + half_h,
            cx - half_w,
            cy
        ),
        "octagon" => {
            let side_x = width * 0.28;
            let side_y = height * 0.28;
            let left = cx - half_w;
            let top = cy - half_h;
            let right = cx + half_w;
            let bottom = cy + half_h;
            format!(
                "M {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} Z",
                cx - side_x, top,
                cx + side_x, top,
                right, cy - side_y,
                right, cy + side_y,
                cx + side_x, bottom,
                cx - side_x, bottom,
                left, cy + side_y,
                left, cy - side_y
            )
        }
        _ => format!(
            "M {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} Z",
            cx - half_w,
            cy - half_h,
            cx + half_w,
            cy - half_h,
            cx + half_w,
            cy + half_h,
            cx - half_w,
            cy + half_h
        ),
    }
}

fn shape_path_d(shape: &str, cx: f32, cy: f32, size: f32) -> String {
    shape_path_d_rect(shape, cx, cy, size, size)
}

fn get_qr_svg_size(shape: &str) -> f32 {
    match shape {
        "diamond" => 340.0,
        "octagon" => 400.0,
        "rounded" => 460.0,
        "square" => 480.0,
        _ => 440.0,
    }
}

fn get_flat_badge_y(shape: &str, is_top: bool) -> f32 {
    match shape {
        "diamond" => if is_top { 148.0 } else { 640.0 },
        "octagon" => if is_top { 118.0 } else { 670.0 },
        "rounded" => if is_top { 98.0 } else { 690.0 },
        "circle" => if is_top { 88.0 } else { 700.0 },
        _ => if is_top { 88.0 } else { 700.0 },
    }
}

fn get_shape_perimeter(shape: &str, radius: f32) -> f32 {
    match shape {
        "square" => 8.0 * radius,
        "diamond" => 4.0 * radius * 2.0_f32.sqrt(),
        "octagon" => 8.0 * radius * 0.828,
        "rounded" => {
            let side = (radius - radius * 0.2) * 2.0;
            let arc = std::f32::consts::PI * (radius * 0.2) / 2.0;
            4.0 * side + 4.0 * arc
        }
        _ => 2.0 * std::f32::consts::PI * radius,
    }
}

fn get_frame_ring_clearance(ring_style: &str, text_size: f32) -> f32 {
    let frame_ring_size = 700.0_f32;
    let base_line = 4.0_f32.max(frame_ring_size * 0.034);
    let mut ring_inset = base_line / 2.0;

    if ring_style == "double" {
        ring_inset = frame_ring_size * 0.034 + (2.0_f32.max(frame_ring_size * 0.012) / 2.0);
    } else if ring_style == "diamond" {
        ring_inset = 6.0_f32.max(frame_ring_size * 0.034) / 2.0 + 8.0;
    } else if ring_style == "neon" {
        ring_inset = 4.0_f32.max(frame_ring_size * 0.028) / 2.0 + 10.0;
    }

    ring_inset + text_size * 0.65 + 10.0
}

fn get_curved_text_radius(requested_radius: f32, text_size: f32, ring_style: &str) -> f32 {
    let safe_radius = 700.0 / 2.0 - get_frame_ring_clearance(ring_style, text_size);
    requested_radius.clamp(200.0, safe_radius)
}

fn get_center_overlay_dimensions(logo_render_width: f32, logo_render_height: f32) -> (f32, f32) {
    let min_side = logo_render_width.min(logo_render_height);
    let line_width = 4.0_f32.max(min_side * 0.04);
    let gap = 2.0_f32.max(min_side * 0.03);
    (
        logo_render_width + line_width + gap * 2.0,
        logo_render_height + line_width + gap * 2.0,
    )
}

fn get_logo_render_dimensions_from_b64(logo_b64: &str, rendered_qr_size: f32, logo_size_percent: u32) -> Option<(f32, f32)> {
    let decoded = general_purpose::STANDARD.decode(strip_data_url_prefix(logo_b64)).ok()?;
    let logo = image::load_from_memory(&decoded).ok()?.to_rgba8();
    let trimmed = trim_transparent_logo(&logo);
    let max_size = ((rendered_qr_size) * (logo_size_percent as f32 / 100.0)).round().max(1.0) as u32;
    let (width, height) = fit_contain_dimensions(trimmed.width(), trimmed.height(), max_size);
    Some((width as f32, height as f32))
}

fn approximate_char_width(ch: char, font_size: f32, spacing: f32) -> f32 {
    let width_factor = match ch {
        'I' | 'J' | 'L' | '1' => 0.36,
        'M' | 'W' => 0.9,
        ' ' => 0.35,
        _ => 0.62,
    };
    font_size * width_factor + spacing
}

fn get_path_point(shape: &str, progress: f32, radius: f32, cx: f32, cy: f32) -> (f32, f32, f32) {
    if shape == "circle" {
        let a = progress * std::f32::consts::PI * 2.0 - std::f32::consts::FRAC_PI_2;
        let x = cx + a.cos() * radius;
        let y = cy + a.sin() * radius;
        return (x, y, a + std::f32::consts::FRAC_PI_2);
    }

    if shape == "square" {
        let side = radius * 2.0;
        let p = (progress + 0.125).rem_euclid(1.0);
        if p < 0.25 {
            let f = p / 0.25;
            return (cx - radius + f * side, cy - radius, 0.0);
        } else if p < 0.5 {
            let f = (p - 0.25) / 0.25;
            return (cx + radius, cy - radius + f * side, std::f32::consts::FRAC_PI_2);
        } else if p < 0.75 {
            let f = (p - 0.5) / 0.25;
            return (cx + radius - f * side, cy + radius, std::f32::consts::PI);
        } else {
            let f = (p - 0.75) / 0.25;
            return (cx - radius, cy + radius - f * side, -std::f32::consts::FRAC_PI_2);
        }
    }

    if shape == "diamond" {
        let p = (progress + 0.125).rem_euclid(1.0);
        if p < 0.25 {
            let f = p / 0.25;
            return (cx + f * radius, cy - radius + f * radius, std::f32::consts::FRAC_PI_4);
        } else if p < 0.5 {
            let f = (p - 0.25) / 0.25;
            return (
                cx + radius - f * radius,
                cy + f * radius,
                3.0 * std::f32::consts::FRAC_PI_4,
            );
        } else if p < 0.75 {
            let f = (p - 0.5) / 0.25;
            return (
                cx - f * radius,
                cy + radius - f * radius,
                -3.0 * std::f32::consts::FRAC_PI_4,
            );
        } else {
            let f = (p - 0.75) / 0.25;
            return (cx - radius + f * radius, cy - f * radius, -std::f32::consts::FRAC_PI_4);
        }
    }

    if shape == "octagon" {
        let p = progress.rem_euclid(1.0);
        let seg = (p * 8.0).floor() as usize;
        let f = (p * 8.0).fract();
        let pts = [
            (cx - radius * 0.28, cy - radius, 0.0),
            (cx + radius * 0.28, cy - radius, std::f32::consts::FRAC_PI_4),
            (cx + radius, cy - radius * 0.28, std::f32::consts::FRAC_PI_2),
            (cx + radius, cy + radius * 0.28, 3.0 * std::f32::consts::FRAC_PI_4),
            (cx + radius * 0.28, cy + radius, std::f32::consts::PI),
            (cx - radius * 0.28, cy + radius, -3.0 * std::f32::consts::FRAC_PI_4),
            (cx - radius, cy + radius * 0.28, -std::f32::consts::FRAC_PI_2),
            (cx - radius, cy - radius * 0.28, -std::f32::consts::FRAC_PI_4),
        ];
        let p1 = pts[seg];
        let p2 = pts[(seg + 1) % 8];
        return (p1.0 + (p2.0 - p1.0) * f, p1.1 + (p2.1 - p1.1) * f, p1.2);
    }

    if shape == "rounded" {
        let r = radius * 0.2;
        let side = (radius - r) * 2.0;
        let arc_len = std::f32::consts::PI * r / 2.0;
        let total_len = 4.0 * side + 4.0 * arc_len;
        let mut d = progress * total_len;
        d = (d + side / 2.0 + total_len).rem_euclid(total_len);

        if d < side {
            return (cx - (radius - r) + d, cy - radius, 0.0);
        } else if d < side + arc_len {
            let a = (d - side) / arc_len * std::f32::consts::FRAC_PI_2 - std::f32::consts::FRAC_PI_2;
            return (
                cx + (radius - r) + a.cos() * r,
                cy - (radius - r) + a.sin() * r,
                a + std::f32::consts::FRAC_PI_2,
            );
        } else if d < 2.0 * side + arc_len {
            return (cx + radius, cy - (radius - r) + (d - side - arc_len), std::f32::consts::FRAC_PI_2);
        } else if d < 2.0 * side + 2.0 * arc_len {
            let a = (d - 2.0 * side - arc_len) / arc_len * std::f32::consts::FRAC_PI_2;
            return (
                cx + (radius - r) + a.cos() * r,
                cy + (radius - r) + a.sin() * r,
                a + std::f32::consts::FRAC_PI_2,
            );
        } else if d < 3.0 * side + 2.0 * arc_len {
            return (cx + (radius - r) - (d - 2.0 * side - 2.0 * arc_len), cy + radius, std::f32::consts::PI);
        } else if d < 3.0 * side + 3.0 * arc_len {
            let a = (d - 3.0 * side - 2.0 * arc_len) / arc_len * std::f32::consts::FRAC_PI_2 + std::f32::consts::FRAC_PI_2;
            return (
                cx - (radius - r) + a.cos() * r,
                cy + (radius - r) + a.sin() * r,
                a + std::f32::consts::FRAC_PI_2,
            );
        } else if d < 4.0 * side + 3.0 * arc_len {
            return (
                cx - radius,
                cy + (radius - r) - (d - 3.0 * side - 3.0 * arc_len),
                -std::f32::consts::FRAC_PI_2,
            );
        } else {
            let a = (d - 4.0 * side - 3.0 * arc_len) / arc_len * std::f32::consts::FRAC_PI_2 + std::f32::consts::PI;
            return (
                cx - (radius - r) + a.cos() * r,
                cy - (radius - r) + a.sin() * r,
                a + std::f32::consts::FRAC_PI_2,
            );
        }
    }

    let a = progress * std::f32::consts::PI * 2.0 - std::f32::consts::FRAC_PI_2;
    (cx + a.cos() * radius, cy + a.sin() * radius, a + std::f32::consts::FRAC_PI_2)
}

fn build_base_qr_svg(options: &QrOptions, size: u32) -> Result<String, String> {
    let dot_color_1 = parse_svg_color(&options.color1)?;
    let dot_color_2 = parse_svg_color(&options.color2)?;
    let eye_out = parse_svg_color(&options.eye_out)?;
    let eye_in = parse_svg_color(&options.eye_in)?;

    let dots = if options.fill_type.eq_ignore_ascii_case("solid") {
        DotsOptions::new(map_dot_type(&options.main_shape)).with_color(dot_color_1)
    } else {
        DotsOptions::new(map_dot_type(&options.main_shape))
            .with_gradient(Gradient::simple_linear(dot_color_1, dot_color_2))
    };

    let mut builder = QRCodeStyling::builder()
        .data(&options.data)
        .size(size)
        .margin(12)
        .qr_options(QROptions::new().with_error_correction_level(ErrorCorrectionLevel::H))
        .dots_options(dots)
        .corners_square_options(
            CornersSquareOptions::new(map_corner_square_type(&options.eye_shape)).with_color(eye_out),
        )
        .corners_dot_options(
            CornersDotOptions::new(map_corner_dot_type(&options.eye_shape)).with_color(eye_in),
        );

    if options.bg_shape == "circle" {
        builder = builder.shape(ShapeType::Circle);
    }

    if let Some(logo_b64) = options.logo_b64.as_ref().filter(|value| !value.is_empty()) {
        let logo_bytes = general_purpose::STANDARD
            .decode(strip_data_url_prefix(logo_b64))
            .map_err(|e| e.to_string())?;
        let trimmed_logo = trim_transparent_logo(&image::load_from_memory(&logo_bytes).map_err(|e| e.to_string())?.to_rgba8());
        let mut trimmed_logo_buffer = Cursor::new(Vec::new());
        image::DynamicImage::ImageRgba8(trimmed_logo)
            .write_to(&mut trimmed_logo_buffer, ImageFormat::Png)
            .map_err(|e| e.to_string())?;
        let image_size = (options.logo_size.unwrap_or(22).clamp(10, 36) as f64) / 100.0;
        builder = builder
            .image(trimmed_logo_buffer.into_inner())
            .image_options(
                ImageOptions::default()
                    .with_image_size(image_size)
                    .with_margin(5)
                    .with_hide_background_dots(true),
            );
    }

    let qr = builder.build().map_err(|e| e.to_string())?;
    let svg = qr.render_svg().map_err(|e| e.to_string())?;
    Ok(inject_svg_background(svg, &options.bg_color))
}

fn build_linear_gradient_def(id: &str, c1: &str, c2: &str, c3: &str, c4: &str, use_fourth: bool) -> String {
    let mut defs = format!(r#"<linearGradient id="{}" x1="0%" y1="0%" x2="100%" y2="100%">"#, id);
    let _ = write!(defs, r#"<stop offset="0%" stop-color="{}"/>"#, c1);
    let _ = write!(defs, r#"<stop offset="50%" stop-color="{}"/>"#, c2);
    let _ = write!(defs, r#"<stop offset="80%" stop-color="{}"/>"#, c3);
    if use_fourth {
        let _ = write!(defs, r#"<stop offset="100%" stop-color="{}"/>"#, c4);
    }
    defs.push_str("</linearGradient>");
    defs
}

fn build_frame_gradient_def(options: &QrOptions) -> Option<String> {
    let ring_style = options.ring_style.as_deref().unwrap_or("solid");
    let ring_color_mode = options.ring_color_mode.as_deref().unwrap_or("solid");
    let use_gradient = ring_style != "solid" && ring_style != "none" && ring_color_mode == "gradient";
    if !use_gradient {
        return None;
    }

    let use_match_main = options.ring_gradient_mode.as_deref().unwrap_or("match-main") == "match-main";
    let c1 = if use_match_main { &options.color1 } else { options.ring_color.as_deref().unwrap_or(&options.color1) };
    let c2 = if use_match_main { &options.color2 } else { options.ring_color2.as_deref().unwrap_or(&options.color2) };
    let c3 = if use_match_main {
        options.color3.as_deref().unwrap_or(&options.color2)
    } else {
        options.ring_color3.as_deref().unwrap_or(c2)
    };
    let use_fourth = if use_match_main {
        options.color4.is_some()
    } else {
        options.ring_use_fourth_stop.unwrap_or(false)
    };
    let c4 = if use_match_main {
        options.color4.as_deref().unwrap_or(c3)
    } else {
        options.ring_color4.as_deref().unwrap_or(c3)
    };

    Some(build_linear_gradient_def("frameRingGradient", c1, c2, c3, c4, use_fourth))
}

fn build_center_overlay_gradient_def(options: &QrOptions) -> Option<String> {
    let mode = options.center_overlay_mode.as_deref().unwrap_or("none");
    if mode == "none" || options.logo_b64.as_ref().filter(|v| !v.is_empty()).is_none() {
        return None;
    }
    if mode == "match" {
        return build_frame_gradient_def(options)
            .map(|_| String::new());
    }

    let style = options.center_overlay_style.as_deref().unwrap_or("solid");
    let color_mode = options.center_overlay_color_mode.as_deref().unwrap_or("solid");
    let use_gradient = style != "solid" && style != "none" && color_mode == "gradient";
    if !use_gradient {
        return None;
    }

    let gradient_mode = options.center_overlay_gradient_mode.as_deref().unwrap_or("match-outer");
    let (c1, c2, c3, c4, use_fourth) = if gradient_mode == "match-main" {
        (
            options.color1.as_str(),
            options.color2.as_str(),
            options.color3.as_deref().unwrap_or(&options.color2),
            options.color4.as_deref().unwrap_or(options.color3.as_deref().unwrap_or(&options.color2)),
            options.color4.is_some(),
        )
    } else if gradient_mode == "match-outer" {
        let use_match_main = options.ring_gradient_mode.as_deref().unwrap_or("match-main") == "match-main";
        let c1 = if use_match_main { &options.color1 } else { options.ring_color.as_deref().unwrap_or(&options.color1) };
        let c2 = if use_match_main { &options.color2 } else { options.ring_color2.as_deref().unwrap_or(&options.color2) };
        let c3 = if use_match_main {
            options.color3.as_deref().unwrap_or(&options.color2)
        } else {
            options.ring_color3.as_deref().unwrap_or(c2)
        };
        let use_fourth = if use_match_main { options.color4.is_some() } else { options.ring_use_fourth_stop.unwrap_or(false) };
        let c4 = if use_match_main { options.color4.as_deref().unwrap_or(c3) } else { options.ring_color4.as_deref().unwrap_or(c3) };
        (c1, c2, c3, c4, use_fourth)
    } else {
        let c1 = options.center_overlay_color.as_deref().unwrap_or("#000000");
        let c2 = options.center_overlay_color2.as_deref().unwrap_or(c1);
        let c3 = options.center_overlay_color3.as_deref().unwrap_or(c2);
        let c4 = options.center_overlay_color4.as_deref().unwrap_or(c3);
        let use_fourth = options.center_overlay_use_fourth_stop.unwrap_or(false);
        (c1, c2, c3, c4, use_fourth)
    };

    Some(build_linear_gradient_def("centerOverlayGradient", c1, c2, c3, c4, use_fourth))
}

fn build_ring_paths_rect(
    shape: &str,
    cx: f32,
    cy: f32,
    width: f32,
    height: f32,
    style: &str,
    stroke: &str,
    glow_id: &str,
) -> String {
    if style == "none" {
        return String::new();
    }
    let base_line = 4.0_f32.max(width.min(height) * 0.04);
    let mut attrs = String::new();
    let _ = write!(
        attrs,
        r#"fill="none" stroke="{}" stroke-linejoin="round" stroke-linecap="round" "#,
        stroke
    );

    if style == "double" {
        let outer = shape_path_d_rect(shape, cx, cy, width, height);
        let inner = shape_path_d_rect(shape, cx, cy, width - base_line * 1.5, height - base_line * 1.5);
        return format!(
            r#"<path d="{}" {} stroke-width="{:.2}"/><path d="{}" {} stroke-width="{:.2}"/>"#,
            outer,
            attrs,
            base_line * 0.4,
            inner,
            attrs,
            base_line * 0.4
        );
    }

    if style == "dashed" {
        let _ = write!(attrs, r#"stroke-dasharray="{:.2} {:.2}" "#, base_line * 2.0, base_line * 2.0);
    } else if style == "dotted" {
        let _ = write!(attrs, r#"stroke-dasharray="{:.2} {:.2}" "#, 2.0, 6.0);
    }

    let path = shape_path_d_rect(shape, cx, cy, width, height);
    if style == "neon" {
        format!(
            r##"<path d="{}" {} stroke-width="{:.2}" opacity="0.88" filter="url(#{})"/><path d="{}" fill="none" stroke="#FFFFFF" stroke-width="{:.2}" stroke-linejoin="round" stroke-linecap="round"/>"##,
            path,
            attrs,
            base_line,
            glow_id,
            path,
            2.0_f32.max(width.min(height) * 0.006)
        )
    } else {
        format!(r#"<path d="{}" {} stroke-width="{:.2}"/>"#, path, attrs, base_line)
    }
}

fn build_ring_paths(shape: &str, cx: f32, cy: f32, size: f32, style: &str, stroke: &str, glow_id: &str) -> String {
    build_ring_paths_rect(shape, cx, cy, size, size, style, stroke, glow_id)
}

fn build_ring_svg(options: &QrOptions) -> String {
    let ring_style = options.ring_style.as_deref().unwrap_or("solid");
    let ring_color = options.ring_color.as_deref().unwrap_or("#000000");
    let use_gradient = options.ring_color_mode.as_deref().unwrap_or("solid") == "gradient"
        && ring_style != "solid"
        && ring_style != "none";
    let stroke = if use_gradient { "url(#frameRingGradient)" } else { ring_color };
    build_ring_paths(&options.bg_shape, 400.0, 400.0, 700.0, ring_style, stroke, "frameNeonGlow")
}

fn build_center_overlay_svg(options: &QrOptions, canvas_size: f32, rendered_qr_size: f32) -> String {
    if options.logo_b64.as_ref().filter(|v| !v.is_empty()).is_none() {
        return String::new();
    }
    let mode = options.center_overlay_mode.as_deref().unwrap_or("none");
    if mode == "none" || (mode == "match" && !options.enable_frame.unwrap_or(false)) {
        return String::new();
    }

    let (style, stroke) = if mode == "match" {
        let ring_style = options.ring_style.as_deref().unwrap_or("solid");
        let use_gradient = options.ring_color_mode.as_deref().unwrap_or("solid") == "gradient"
            && ring_style != "solid"
            && ring_style != "none";
        let stroke = if use_gradient { "url(#frameRingGradient)" } else { options.ring_color.as_deref().unwrap_or("#000000") };
        (ring_style.to_string(), stroke.to_string())
    } else {
        let style = options.center_overlay_style.as_deref().unwrap_or("solid").to_string();
        let use_gradient = options.center_overlay_color_mode.as_deref().unwrap_or("solid") == "gradient"
            && style != "solid"
            && style != "none";
        let stroke = if use_gradient { "url(#centerOverlayGradient)" } else { options.center_overlay_color.as_deref().unwrap_or("#000000") };
        (style, stroke.to_string())
    };

    if style == "none" {
        return String::new();
    }

    let Some(logo_b64) = options.logo_b64.as_deref() else {
        return String::new();
    };
    let Some((logo_width, logo_height)) = get_logo_render_dimensions_from_b64(
        logo_b64,
        rendered_qr_size,
        options.logo_size.unwrap_or(22).clamp(10, 36),
    ) else {
        return String::new();
    };
    let (overlay_width, overlay_height) = get_center_overlay_dimensions(logo_width, logo_height);
    let center = canvas_size / 2.0;
    build_ring_paths_rect(&options.bg_shape, center, center, overlay_width, overlay_height, &style, &stroke, "centerOverlayGlow")
}

fn build_text_color(current_color: &str, match_style: bool, ring_style: &str, use_gradient: bool, ring_color: &str) -> String {
    if match_style {
        if use_gradient {
            "url(#frameRingGradient)".to_string()
        } else if ring_style == "neon" {
            "#FFFFFF".to_string()
        } else if ring_style == "diamond" {
            ring_color.to_string()
        } else {
            current_color.to_string()
        }
    } else {
        current_color.to_string()
    }
}

fn build_flat_text_svg(
    text: &str,
    is_top: bool,
    options: &QrOptions,
    ring_style: &str,
    use_gradient: bool,
) -> String {
    if text.is_empty() {
        return String::new();
    }

    let text_upper = svg_escape(&text.to_uppercase());
    let current_color = if is_top {
        options.frame_text_top_color.as_deref().unwrap_or("#000000")
    } else {
        options.frame_text_color.as_deref().unwrap_or("#000000")
    };
    let match_style = if is_top {
        options.match_text_top_style.unwrap_or(false)
    } else {
        options.match_text_style.unwrap_or(false)
    };
    let transparent_bg = if is_top {
        options.transparent_text_top_bg.unwrap_or(false)
    } else {
        options.transparent_text_bg.unwrap_or(false)
    };
    let font_size = if is_top {
        options.frame_text_top_size.unwrap_or(44.0)
    } else {
        options.frame_text_size.unwrap_or(44.0)
    };
    let badge_y = get_flat_badge_y(&options.bg_shape, is_top);
    let approx_width = text.chars().map(|ch| approximate_char_width(ch, font_size, 0.0)).sum::<f32>();
    let badge_width = approx_width + 80.0;
    let badge_height = 70.0;
    let bx = 400.0 - badge_width / 2.0;
    let by = badge_y - badge_height / 2.0;
    let ring_color = options.ring_color.as_deref().unwrap_or("#000000");
    let fill = build_text_color(current_color, match_style, ring_style, use_gradient, ring_color);
    let mut svg = String::new();

    if !transparent_bg {
        if match_style && (ring_style == "rounded" || options.bg_shape == "rounded") {
            let _ = write!(
                svg,
                r#"<rect x="{:.2}" y="{:.2}" width="{:.2}" height="{:.2}" rx="15" ry="15" fill="{}"/>"#,
                bx,
                by,
                badge_width,
                badge_height,
                options.bg_color
            );
        } else {
            let _ = write!(
                svg,
                r#"<rect x="{:.2}" y="{:.2}" width="{:.2}" height="{:.2}" fill="{}"/>"#,
                bx,
                by,
                badge_width,
                badge_height,
                options.bg_color
            );
        }

        if match_style && !matches!(ring_style, "none" | "neon" | "gradient") {
            let stroke_attr = if use_gradient { "url(#frameRingGradient)" } else { ring_color };
            let dash = if ring_style == "dotted" {
                r#" stroke-dasharray="2 6""#
            } else if ring_style == "dashed" {
                r#" stroke-dasharray="12 6""#
            } else {
                ""
            };
            let _ = write!(
                svg,
                r#"<rect x="{:.2}" y="{:.2}" width="{:.2}" height="{:.2}" {} fill="none" stroke="{}" stroke-width="3"{}{} />"#,
                bx,
                by,
                badge_width,
                badge_height,
                if ring_style == "rounded" || options.bg_shape == "rounded" { r#"rx="15" ry="15""# } else { "" },
                stroke_attr,
                dash,
                if ring_style == "double" { r#""# } else { "" }
            );
            if ring_style == "double" {
                let _ = write!(
                    svg,
                    r#"<rect x="{:.2}" y="{:.2}" width="{:.2}" height="{:.2}" {} fill="none" stroke="{}" stroke-width="1"/>"#,
                    bx + 5.0,
                    by + 5.0,
                    badge_width - 10.0,
                    badge_height - 10.0,
                    if ring_style == "rounded" || options.bg_shape == "rounded" { r#"rx="12" ry="12""# } else { "" },
                    stroke_attr,
                );
            }
        }
    }

    let _ = write!(
        svg,
        r#"<text x="400" y="{:.2}" text-anchor="middle" dominant-baseline="middle" font-family="'Segoe UI', Arial, sans-serif" font-size="{:.2}" font-weight="700" fill="{}">{}</text>"#,
        badge_y,
        font_size,
        fill,
        text_upper
    );
    svg
}

fn build_curved_text_svg(
    text: &str,
    is_top: bool,
    options: &QrOptions,
    ring_style: &str,
    use_gradient: bool,
) -> String {
    if text.is_empty() {
        return String::new();
    }

    let font_size = if is_top {
        options.frame_text_top_size.unwrap_or(44.0)
    } else {
        options.frame_text_size.unwrap_or(44.0)
    };
    let requested_radius = if is_top {
        options.frame_text_top_radius.unwrap_or(350.0)
    } else {
        options.frame_text_radius.unwrap_or(350.0)
    };
    let spacing = if is_top {
        options.frame_text_top_spacing.unwrap_or(1.0)
    } else {
        options.frame_text_spacing.unwrap_or(1.0)
    };
    let current_color = if is_top {
        options.frame_text_top_color.as_deref().unwrap_or("#000000")
    } else {
        options.frame_text_color.as_deref().unwrap_or("#000000")
    };
    let match_style = if is_top {
        options.match_text_top_style.unwrap_or(false)
    } else {
        options.match_text_style.unwrap_or(false)
    };
    let ring_color = options.ring_color.as_deref().unwrap_or("#000000");
    let fill = build_text_color(current_color, match_style, ring_style, use_gradient, ring_color);

    let curved_radius = get_curved_text_radius(requested_radius, font_size, ring_style);
    let chars: Vec<char> = text.to_uppercase().chars().collect();
    let char_widths: Vec<f32> = chars
        .iter()
        .map(|ch| approximate_char_width(*ch, font_size, spacing))
        .collect();
    let total_text_width: f32 = char_widths.iter().sum();
    let perimeter = get_shape_perimeter(&options.bg_shape, curved_radius);
    let width_to_progress = 1.0 / perimeter;
    let direction = if is_top { 1.0 } else { -1.0 };
    let mut current_progress = if is_top {
        -(total_text_width / 2.0) * width_to_progress
    } else {
        0.5 + (total_text_width / 2.0) * width_to_progress
    };

    let mut svg = String::new();
    for (index, ch) in chars.iter().enumerate() {
        let char_progress = current_progress + direction * (char_widths[index] / 2.0) * width_to_progress;
        let (x, y, angle) = get_path_point(&options.bg_shape, char_progress, curved_radius, 400.0, 400.0);
        let rotation = if is_top { angle } else { angle + std::f32::consts::PI };
        let _ = write!(
            svg,
            r#"<text x="0" y="0" text-anchor="middle" dominant-baseline="middle" font-family="'Segoe UI', Arial, sans-serif" font-size="{:.2}" font-weight="700" fill="{}" transform="translate({:.2} {:.2}) rotate({:.2})">{}</text>"#,
            font_size,
            fill,
            x,
            y,
            rotation.to_degrees(),
            svg_escape(&ch.to_string())
        );
        current_progress += direction * char_widths[index] * width_to_progress;
    }
    svg
}

fn build_frame_text_svg(options: &QrOptions) -> String {
    let top_text = options.frame_text_top.as_deref().unwrap_or("");
    let bottom_text = options.frame_text.as_deref().unwrap_or("");
    let top_mode = options.frame_text_top_mode.as_deref().unwrap_or("flat");
    let bottom_mode = options.frame_text_mode.as_deref().unwrap_or("flat");
    let ring_style = options.ring_style.as_deref().unwrap_or("solid");
    let use_gradient = options.ring_color_mode.as_deref().unwrap_or("solid") == "gradient"
        && ring_style != "solid"
        && ring_style != "none";

    let mut svg = String::new();
    if top_mode == "curved" {
        svg.push_str(&build_curved_text_svg(top_text, true, options, ring_style, use_gradient));
    } else {
        svg.push_str(&build_flat_text_svg(top_text, true, options, ring_style, use_gradient));
    }

    if bottom_mode == "curved" {
        svg.push_str(&build_curved_text_svg(bottom_text, false, options, ring_style, use_gradient));
    } else {
        svg.push_str(&build_flat_text_svg(bottom_text, false, options, ring_style, use_gradient));
    }
    svg
}

fn build_framed_svg(options: &QrOptions) -> Result<String, String> {
    let qr_size = get_qr_svg_size(&options.bg_shape) as u32;
    let qr_offset = (800.0 - qr_size as f32) / 2.0;
    let inner_qr_svg = build_base_qr_svg(options, qr_size)?;
    let nested_qr = strip_xml_declaration(&inner_qr_svg);
    let bg_fill = if options.transparent_frame_bg.unwrap_or(false) {
        String::new()
    } else {
        format!(
            r#"<path d="{}" fill="{}"/>"#,
            shape_path_d(&options.bg_shape, 400.0, 400.0, 800.0),
            options.bg_color
        )
    };
    let gradient_def = build_frame_gradient_def(options).unwrap_or_default();
    let center_gradient_def = build_center_overlay_gradient_def(options).unwrap_or_default();
    let neon_def = if options.ring_style.as_deref().unwrap_or("solid") == "neon" {
        r#"<filter id="frameNeonGlow" x="-40%" y="-40%" width="180%" height="180%"><feGaussianBlur stdDeviation="14" result="blur"/><feMerge><feMergeNode in="blur"/><feMergeNode in="SourceGraphic"/></feMerge></filter>"#.to_string()
    } else {
        String::new()
    };
    let center_neon_def = if (options.center_overlay_mode.as_deref().unwrap_or("none") == "custom"
        && options.center_overlay_style.as_deref().unwrap_or("solid") == "neon")
        || (options.center_overlay_mode.as_deref().unwrap_or("none") == "match"
            && options.enable_frame.unwrap_or(false)
            && options.ring_style.as_deref().unwrap_or("solid") == "neon")
    {
        r#"<filter id="centerOverlayGlow" x="-40%" y="-40%" width="180%" height="180%"><feGaussianBlur stdDeviation="8" result="blur"/><feMerge><feMergeNode in="blur"/><feMergeNode in="SourceGraphic"/></feMerge></filter>"#.to_string()
    } else {
        String::new()
    };

    Ok(format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 800 800" width="800" height="800"><defs>{}{}{}{}<clipPath id="frameInnerClip"><path d="{}"/></clipPath></defs>{}{}<g clip-path="url(#frameInnerClip)"><g transform="translate({:.2} {:.2})">{}</g></g>{}{}</svg>"#,
        gradient_def,
        center_gradient_def,
        neon_def,
        center_neon_def,
        shape_path_d(&options.bg_shape, 400.0, 400.0, 660.0),
        bg_fill,
        build_ring_svg(options),
        qr_offset,
        qr_offset,
        nested_qr,
        build_center_overlay_svg(options, 800.0, qr_size as f32),
        build_frame_text_svg(options)
    ))
}

fn build_svg_qr(options: &QrOptions) -> Result<String, String> {
    if options.enable_frame.unwrap_or(false) {
        build_framed_svg(options)
    } else {
        let base = build_base_qr_svg(options, 600)?;
        let nested_qr = strip_xml_declaration(&base);
        let center_gradient_def = build_center_overlay_gradient_def(options).unwrap_or_default();
        let center_neon_def = if options.center_overlay_style.as_deref().unwrap_or("solid") == "neon" {
            r#"<filter id="centerOverlayGlow" x="-40%" y="-40%" width="180%" height="180%"><feGaussianBlur stdDeviation="8" result="blur"/><feMerge><feMergeNode in="blur"/><feMergeNode in="SourceGraphic"/></feMerge></filter>"#.to_string()
        } else {
            String::new()
        };
        Ok(format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 600 600" width="600" height="600"><defs>{}{}</defs>{}{}</svg>"#,
            center_gradient_def,
            center_neon_def,
            nested_qr,
            build_center_overlay_svg(options, 600.0, 600.0)
        ))
    }
}

#[tauri::command]
fn generate_ultra_qr(options: QrOptions) -> Result<String, String> {
    let data = options.data;
    let color1 = options.color1;
    let color2 = options.color2;
    let color3 = options.color3.unwrap_or_else(|| color2.clone());
    let color4 = options.color4;
    let bg_color = options.bg_color;
    let eye_out = options.eye_out;
    let eye_in = options.eye_in;
    let fill_type = options.fill_type;
    let main_shape = options.main_shape;
    let bg_shape = options.bg_shape;
    let eye_shape = options.eye_shape;
    let logo_b64 = options.logo_b64;
    let logo_size_percent = options.logo_size.unwrap_or(22).clamp(10, 36);
    let logo_opacity_percent = options.logo_opacity.unwrap_or(100).clamp(15, 100);

    let qrcode = QRBuilder::new(data).ecl(fast_qr::ECL::H).build().map_err(|e| e.to_string())?;

    let width = 600u32;
    let height = 600u32;
    let mut img = RgbaImage::new(width, height);

    let c1 = hex_to_rgba(&color1);
    let c2 = hex_to_rgba(&color2);
    let c3 = hex_to_rgba(&color3);
    let c4 = color4.as_ref().map(|value| hex_to_rgba(value));
    let bg = hex_to_rgba(&bg_color);
    let e_out = hex_to_rgba(&eye_out);
    let e_in = hex_to_rgba(&eye_in);

    let modules = qrcode.size as i32;
    let margin = 4.0;
    let total_modules = modules as f32 + (margin * 2.0);
    let sample_offsets = [(0.25f32, 0.25f32), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)];
    let eye_coords = get_eye_coords(modules);

    for x in 0..width {
        for y in 0..height {
            let mut sum_r = 0.0f32;
            let mut sum_g = 0.0f32;
            let mut sum_b = 0.0f32;

            for (sample_dx, sample_dy) in sample_offsets {
                let mod_x_f32 = ((x as f32 + sample_dx) / width as f32) * total_modules;
                let mod_y_f32 = ((y as f32 + sample_dy) / height as f32) * total_modules;
                
                let qr_x_f32 = mod_x_f32 - margin;
                let qr_y_f32 = mod_y_f32 - margin;
                
                let qr_x = qr_x_f32.floor() as i32;
                let qr_y = qr_y_f32.floor() as i32;

                let mut is_eye = false;
                let mut is_eye_reserved = false;
                let mut eye_rel_x = 0.0f32;
                let mut eye_rel_y = 0.0f32;

                for (x1, x2, y1, y2) in &eye_coords {
                    if qr_x >= *x1 && qr_x < *x2 && qr_y >= *y1 && qr_y < *y2 {
                        is_eye = true;
                        eye_rel_x = qr_x_f32 - (*x1 as f32);
                        eye_rel_y = qr_y_f32 - (*y1 as f32);
                        break;
                    }
                    // Reserved area for eyes (usually 1 module around the eye)
                    if qr_x >= *x1 - 1 && qr_x < *x2 + 1 && qr_y >= *y1 - 1 && qr_y < *y2 + 1 {
                        is_eye_reserved = true;
                    }
                }

                let in_qr_bounds = qr_x >= 0 && qr_y >= 0 && qr_x < modules && qr_y < modules;
                let module_is_dark = in_qr_bounds && qrcode[qr_y as usize][qr_x as usize].value();

                let local_x = (qr_x_f32 - qr_x_f32.floor()).clamp(0.0, 0.9999);
                let local_y = (qr_y_f32 - qr_y_f32.floor()).clamp(0.0, 0.9999);

                let mut paint = false;
                let mut sample_color = bg;

                if is_eye {
                    let dx = eye_rel_x - 3.5;
                    let dy = eye_rel_y - 3.5;

                    if eye_shape == "circle" {
                        let dist = (dx * dx + dy * dy).sqrt();
                        if dist <= 1.5 { paint = true; sample_color = e_in; }
                        else if (2.5..=3.5).contains(&dist) { paint = true; sample_color = e_out; }
                    } else if eye_shape == "diamond" {
                        let dist = dx.abs() + dy.abs();
                        if dist <= 2.0 { paint = true; sample_color = e_in; }
                        else if (3.0..=5.0).contains(&dist) { paint = true; sample_color = e_out; }
                    } else if eye_shape == "rounded" {
                        let adx = dx.abs();
                        let ady = dy.abs();
                        let mut in_inner = false;
                        if adx <= 1.5 && ady <= 1.5 {
                            if adx < 1.0 || ady < 1.0 { in_inner = true; }
                            else {
                                let cx = adx - 1.0;
                                let cy = ady - 1.0;
                                if cx * cx + cy * cy <= 0.25 { in_inner = true; }
                            }
                        }

                        let mut in_outer = false;
                        if adx <= 3.5 && ady <= 3.5 {
                            if adx < 2.5 || ady < 2.5 { in_outer = true; }
                            else {
                                let cx = adx - 2.5;
                                let cy = ady - 2.5;
                                if cx * cx + cy * cy <= 1.0 { in_outer = true; }
                            }
                        }

                        if in_inner { paint = true; sample_color = e_in; }
                        else if in_outer && (adx >= 2.5 || ady >= 2.5) { paint = true; sample_color = e_out; }
                    } else if module_is_dark {
                        paint = true;
                        if dx.abs() <= 1.5 && dy.abs() <= 1.5 { sample_color = e_in; }
                        else { sample_color = e_out; }
                    }
                } else if !is_eye_reserved && module_is_dark {
                    if main_shape == "circle" {
                        let dx = local_x - 0.5;
                        let dy = local_y - 0.5;
                        if dx * dx + dy * dy <= 0.21 { paint = true; }
                    } else if main_shape == "rounded" {
                        let dx = (local_x - 0.5).abs();
                        let dy = (local_y - 0.5).abs();
                        if dx < 0.34 || dy < 0.34 {
                            paint = true;
                        } else {
                            let cx = dx - 0.34;
                            let cy = dy - 0.34;
                            if cx * cx + cy * cy <= 0.028 { paint = true; }
                        }
                    } else if main_shape == "diamond" {
                        let dx = (local_x - 0.5).abs();
                        let dy = (local_y - 0.5).abs();
                        if dx + dy <= 0.52 { paint = true; }
                    } else {
                        paint = true;
                    }

                    if paint {
                        sample_color = if fill_type == "Linear" {
                            gradient_color_at(c1, c2, c3, c4, (y as f32 + sample_dy) / height as f32)
                        } else {
                            c1
                        };
                    }
                }

                if paint {
                    sum_r += sample_color[0] as f32;
                    sum_g += sample_color[1] as f32;
                    sum_b += sample_color[2] as f32;
                } else {
                    sum_r += bg[0] as f32;
                    sum_g += bg[1] as f32;
                    sum_b += bg[2] as f32;
                }
            }

            let samples = sample_offsets.len() as f32;
            img.put_pixel(
                x,
                y,
                Rgba([
                    (sum_r / samples).round() as u8,
                    (sum_g / samples).round() as u8,
                    (sum_b / samples).round() as u8,
                    255,
                ]),
            );
        }
    }

    if let Some(logo_str) = logo_b64 {
        let clean_b64 = strip_data_url_prefix(&logo_str);
        
        if let Ok(decoded) = general_purpose::STANDARD.decode(clean_b64) {
            if let Ok(logo_img) = image::load_from_memory(&decoded) {
                let trimmed_logo = trim_transparent_logo(&logo_img.to_rgba8());
                let logo_size = ((width as f32) * (logo_size_percent as f32 / 100.0)).round() as u32;
                let (target_width, target_height) = fit_contain_dimensions(
                    trimmed_logo.width(),
                    trimmed_logo.height(),
                    logo_size.max(1),
                );
                let resized_logo = image::DynamicImage::ImageRgba8(trimmed_logo)
                    .resize(target_width, target_height, image::imageops::FilterType::Lanczos3)
                    .to_rgba8();

                let x_offset = (width - resized_logo.width()) / 2;
                let y_offset = (height - resized_logo.height()) / 2;

                let padding = 12u32;

                let padded_width = resized_logo.width() + padding * 2;
                let padded_height = resized_logo.height() + padding * 2;
                let padded_x = x_offset.saturating_sub(padding);
                let padded_y = y_offset.saturating_sub(padding);

                for local_x in 0..padded_width {
                    for local_y in 0..padded_height {
                        let target_x = padded_x + local_x;
                        let target_y = padded_y + local_y;
                        if target_x >= width || target_y >= height {
                            continue;
                        }
                        if point_in_shape_rect(&bg_shape, local_x as f32, local_y as f32, padded_width as f32, padded_height as f32) {
                            img.put_pixel(target_x, target_y, Rgba([255, 255, 255, 255]));
                        }
                    }
                }

                for local_x in 0..resized_logo.width() {
                    for local_y in 0..resized_logo.height() {
                        if !point_in_shape_rect(&bg_shape, local_x as f32, local_y as f32, resized_logo.width() as f32, resized_logo.height() as f32) {
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

                        let mut adjusted_src = src;
                        adjusted_src[3] = ((adjusted_src[3] as f32) * (logo_opacity_percent as f32 / 100.0))
                            .round()
                            .clamp(0.0, 255.0) as u8;
                        if adjusted_src[3] == 0 {
                            continue;
                        }

                        let dst = img.get_pixel_mut(target_x, target_y);
                        blend_pixel(dst, adjusted_src);
                    }
                }
            }
        }
    }

    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, ImageFormat::Png).map_err(|e| e.to_string())?;
    Ok(format!("data:image/png;base64,{}", general_purpose::STANDARD.encode(buffer.into_inner())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eye_coords() {
        let coords = get_eye_coords(21);
        assert_eq!(coords.len(), 3);
        assert_eq!(coords[0], (0, 7, 0, 7));
        assert_eq!(coords[1], (14, 21, 0, 7));
        assert_eq!(coords[2], (0, 7, 14, 21));
    }

    #[test]
    fn test_hex_to_rgba() {
        let black = hex_to_rgba("#000000");
        assert_eq!(black, Rgba([0, 0, 0, 255]));

        let white = hex_to_rgba("#FFFFFF");
        assert_eq!(white, Rgba([255, 255, 255, 255]));

        let short_red = hex_to_rgba("#F00");
        assert_eq!(short_red, Rgba([255, 0, 0, 255]));

        let invalid = hex_to_rgba("invalid");
        assert_eq!(invalid, Rgba([0, 0, 0, 255]));
    }

    #[test]
    fn test_point_in_shape_square() {
        assert!(point_in_shape("square", 5.0, 5.0, 10.0));
        assert!(point_in_shape("square", 0.0, 0.0, 10.0));
        assert!(point_in_shape("square", 10.0, 10.0, 10.0));
    }

    #[test]
    fn test_point_in_shape_circle() {
        assert!(point_in_shape("circle", 5.0, 5.0, 10.0));
        assert!(!point_in_shape("circle", 0.0, 0.0, 10.0));
    }

    #[test]
    fn test_styled_qr_uses_high_error_correction() {
        let qr = QRCodeStyling::builder()
            .data("https://example.com")
            .size(300)
            .qr_options(QROptions::new().with_error_correction_level(ErrorCorrectionLevel::H))
            .build()
            .expect("styled qr should build");

        assert_eq!(
            qr.options().qr_options.error_correction_level,
            ErrorCorrectionLevel::H
        );
    }

    #[test]
    fn test_build_svg_qr_with_frame_contains_overlay_markup() {
        let mut logo_buffer = Cursor::new(Vec::new());
        image::DynamicImage::ImageRgba8(RgbaImage::from_pixel(2, 2, Rgba([0, 170, 136, 255])))
            .write_to(&mut logo_buffer, ImageFormat::Png)
            .expect("test png should encode");
        let logo_b64 = format!(
            "data:image/png;base64,{}",
            general_purpose::STANDARD.encode(logo_buffer.into_inner())
        );

        let options = QrOptions {
            data: "https://example.com".to_string(),
            color1: "#000000".to_string(),
            color2: "#222222".to_string(),
            color3: Some("#444444".to_string()),
            color4: None,
            bg_color: "#FFFFFF".to_string(),
            eye_out: "#111111".to_string(),
            eye_in: "#333333".to_string(),
            fill_type: "Solid".to_string(),
            main_shape: "square".to_string(),
            bg_shape: "square".to_string(),
            eye_shape: "square".to_string(),
            logo_b64: Some(logo_b64),
            logo_size: Some(22),
            logo_opacity: Some(100),
            enable_frame: Some(true),
            frame_text: Some("Scan Me".to_string()),
            frame_text_top: Some("Top".to_string()),
            frame_text_mode: Some("flat".to_string()),
            frame_text_top_mode: Some("curved".to_string()),
            frame_text_radius: Some(350.0),
            frame_text_top_radius: Some(350.0),
            frame_text_spacing: Some(1.0),
            frame_text_top_spacing: Some(1.0),
            frame_text_size: Some(44.0),
            frame_text_top_size: Some(44.0),
            frame_text_color: Some("#A946E0".to_string()),
            frame_text_top_color: Some("#A946E0".to_string()),
            match_text_style: Some(false),
            match_text_top_style: Some(false),
            transparent_text_bg: Some(false),
            transparent_text_top_bg: Some(true),
            transparent_frame_bg: Some(false),
            ring_style: Some("solid".to_string()),
            ring_color: Some("#A946E0".to_string()),
            ring_color2: Some("#8b5e3c".to_string()),
            ring_color3: Some("#e6a756".to_string()),
            ring_color4: Some("#ffd166".to_string()),
            ring_use_fourth_stop: Some(true),
            ring_gradient_mode: Some("custom".to_string()),
            ring_color_mode: Some("solid".to_string()),
            center_overlay_mode: Some("custom".to_string()),
            center_overlay_style: Some("solid".to_string()),
            center_overlay_color: Some("#00AA88".to_string()),
            center_overlay_color2: Some("#22BB99".to_string()),
            center_overlay_color3: Some("#44CCAA".to_string()),
            center_overlay_color4: Some("#66DDBB".to_string()),
            center_overlay_use_fourth_stop: Some(true),
            center_overlay_gradient_mode: Some("custom".to_string()),
            center_overlay_color_mode: Some("solid".to_string()),
        };

        let svg = build_svg_qr(&options).expect("svg should generate");
        assert!(svg.contains("<svg"));
        assert!(svg.contains("frameInnerClip"));
        assert!(svg.contains("SCAN ME"));
        assert!(svg.contains(">T<") || svg.contains(">O<") || svg.contains(">P<"));
        assert!(svg.contains("#00AA88"));
    }
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
            save_svg_to_path,
            open_external_link,
            open_last_saved_image,
            share_last_saved_image,
            print_current_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
