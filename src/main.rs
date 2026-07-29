use std::path::Path;
use std::process::exit;

const BASE: &str = "https://micompress.com";
const UA: &str = "micompress-rust/0.1.0 (+https://micompress.com)";

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct Meta {
    success: bool,
    error: Option<String>,
    download_url: Option<String>,
    input_size: u64,
    output_size: u64,
    saved_percent: i64,
}

/// Map a filename to its MiCompress category endpoint segment.
fn endpoint_for(filename: &str) -> Option<&'static str> {
    let ext = Path::new(filename).extension()?.to_str()?.to_lowercase();
    let category = match ext.as_str() {
        "jpg" | "jpeg" | "png" | "webp" | "gif" | "bmp" | "tiff" | "tif" | "avif" | "svg"
        | "heic" | "heif" | "ico" | "psd" | "dng" | "raw" | "cr2" | "arw" | "nef" => "image",
        "mp4" | "mov" | "avi" | "mkv" | "webm" | "mpeg" | "mpg" | "m4v" | "wmv" | "flv" | "3gp" => "video",
        "mp3" | "wav" | "flac" | "ogg" | "m4a" | "aac" | "wma" | "opus" => "audio",
        "pdf" => "pdf",
        "csv" | "json" | "xml" | "md" | "odt" | "ods" | "docx" | "doc" | "ppt" | "pptx"
        | "xls" | "xlsx" | "epub" | "html" | "htm" | "hwp" | "txt" | "rtf" => "document",
        "zip" | "7z" | "tar" | "gz" | "tgz" | "rar" => "archive",
        _ => return None,
    };
    Some(match category {
        "image" => "image-compressors",
        "video" => "video-compressors",
        "audio" => "audio-compressors",
        "document" => "document-compressors",
        "pdf" => "pdf-compressors",
        _ => "archive-compressors",
    })
}

fn human(n: u64) -> String {
    let units = ["B", "KB", "MB", "GB"];
    let mut v = n as f64;
    let mut i = 0;
    while v >= 1024.0 && i < units.len() - 1 {
        v /= 1024.0;
        i += 1;
    }
    if i == 0 {
        format!("{}{}", v.round() as u64, units[i])
    } else {
        format!("{:.1}{}", v, units[i])
    }
}

fn out_path(input: &str, output: &Option<String>, suffix: &str) -> String {
    let p = Path::new(input);
    let name = p.file_name().and_then(|s| s.to_str()).unwrap_or(input);
    if let Some(dir) = output {
        let _ = std::fs::create_dir_all(dir);
        return Path::new(dir).join(name).to_string_lossy().into_owned();
    }
    let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or(name);
    let ext = p.extension().and_then(|s| s.to_str());
    let parent = p.parent().map(|x| x.to_string_lossy().into_owned()).unwrap_or_default();
    let fname = match ext {
        Some(e) => format!("{stem}{suffix}.{e}"),
        None => format!("{stem}{suffix}"),
    };
    if parent.is_empty() { fname } else { format!("{parent}/{fname}") }
}

fn compress(
    client: &reqwest::blocking::Client,
    path: &str,
    preset: &str,
    api_base: &str,
    token: &Option<String>,
) -> Result<(Vec<u8>, Meta), Box<dyn std::error::Error>> {
    let filename = Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("invalid filename")?;
    let endpoint = endpoint_for(filename).ok_or("unsupported file type")?;

    let form = reqwest::blocking::multipart::Form::new()
        .text("preset", preset.to_string())
        .file("file", path)?;

    let url = format!("{}/{}/compress", api_base.trim_end_matches('/'), endpoint);
    let mut req = client.post(&url).header("Accept", "application/json").multipart(form);
    if let Some(t) = token {
        req = req.header("Authorization", format!("Bearer {t}"));
    }
    let meta: Meta = req.send()?.json()?;
    if !meta.success {
        return Err(meta.error.unwrap_or_else(|| "compression failed".into()).into());
    }
    let dl = meta.download_url.clone().ok_or("no download url")?;

    let mut dreq = client.get(format!("{}{}", api_base.trim_end_matches('/'), dl));
    if let Some(t) = token {
        dreq = dreq.header("Authorization", format!("Bearer {t}"));
    }
    let bytes = dreq.send()?.error_for_status()?.bytes()?.to_vec();
    Ok((bytes, meta))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut files: Vec<String> = Vec::new();
    let mut preset = "recommended".to_string();
    let mut api_base = BASE.to_string();
    let mut suffix = "-min".to_string();
    let mut output: Option<String> = None;
    let mut token: Option<String> = None;
    let mut quiet = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-v" | "--version" => { println!("0.1.0"); return; }
            "-h" | "--help" => {
                println!("micompress <file...> [-o dir] [-p lossless|recommended|extreme] [--token jwt]");
                println!("Powered by micompress.com");
                return;
            }
            "-q" | "--quiet" => quiet = true,
            "-o" | "--output" => { i += 1; output = args.get(i).cloned(); }
            "-p" | "--preset" => { i += 1; if let Some(v) = args.get(i) { preset = v.clone(); } }
            "--api" => { i += 1; if let Some(v) = args.get(i) { api_base = v.clone(); } }
            "--token" => { i += 1; token = args.get(i).cloned(); }
            "--suffix" => { i += 1; if let Some(v) = args.get(i) { suffix = v.clone(); } }
            other => files.push(other.to_string()),
        }
        i += 1;
    }

    if files.is_empty() {
        eprintln!("Usage: micompress <file...> [options]");
        exit(1);
    }

    let client = reqwest::blocking::Client::builder()
        .user_agent(UA)
        .build()
        .expect("http client");

    let mut failed = 0;
    for file in &files {
        match compress(&client, file, &preset, &api_base, &token) {
            Ok((bytes, meta)) => {
                let dest = out_path(file, &output, &suffix);
                if let Err(e) = std::fs::write(&dest, &bytes) {
                    failed += 1;
                    eprintln!("✗ {file}: {e}");
                    continue;
                }
                if !quiet {
                    println!(
                        "✔ {file} → {dest}  {} → {} (-{}%)",
                        human(meta.input_size),
                        human(meta.output_size),
                        meta.saved_percent
                    );
                }
            }
            Err(e) => {
                failed += 1;
                eprintln!("✗ {file}: {e}");
            }
        }
    }
    exit(if failed == 0 { 0 } else { 1 });
}
