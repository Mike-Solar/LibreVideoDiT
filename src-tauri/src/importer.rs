use std::collections::HashSet;
use std::fs;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};

use exif::{In, Reader, Tag};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::camera::CameraProfile;
use crate::config::{Config, SdCardMapping, CONFIG};
use crate::hash;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportReport {
    pub copied: usize,
    pub skipped: usize,
    pub failed: usize,
    pub target_base: String,
    pub camera_fallback: String,
    pub errors: Vec<String>,
}

pub fn import_sd_card(sd_card_path: String) -> Result<ImportReport, Box<dyn std::error::Error>> {
    let sd_root = PathBuf::from(sd_card_path);
    if !sd_root.exists() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            "SD card path not found",
        )));
    }
    let config = CONFIG.read()?.clone();
    if config.destination_root.trim().is_empty() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "destination_root is empty in config.json",
        )));
    }

    let mapping = find_card_mapping(&sd_root, &config);
    let camera_profile = detect_camera(&sd_root, &config);
    if camera_profile.is_none() && mapping.as_ref().and_then(|m| m.camera_override.as_ref()).is_none() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "SD card structure did not match any configured camera",
        )));
    }
    let dest_root = PathBuf::from(config.destination_root);
    let target_base = match &mapping {
        Some(m) if !m.target_subdir.trim().is_empty() => dest_root.join(&m.target_subdir),
        _ => dest_root,
    };

    let exts = normalized_exts(&config);
    let media_files = list_media_files(&sd_root, &camera_profile, &exts)?;
    let fallback_camera = mapping
        .as_ref()
        .and_then(|m| m.camera_override.as_ref())
        .cloned()
        .or_else(|| camera_profile.as_ref().map(|c| c.name.clone()))
        .unwrap_or_else(|| "UnknownCamera".to_string());

    let mut report = ImportReport {
        copied: 0,
        skipped: 0,
        failed: 0,
        target_base: target_base.to_string_lossy().to_string(),
        camera_fallback: fallback_camera.clone(),
        errors: Vec::new(),
    };

    for file_path in media_files {
        let rel_path = file_path
            .strip_prefix(&sd_root)
            .unwrap_or(&file_path)
            .to_path_buf();

        let camera_name = resolve_camera_name(&file_path, &fallback_camera);
        let camera_dir = sanitize_component(&camera_name);
        let dest_path = target_base.join(camera_dir).join(rel_path);

        match copy_with_dedup(&file_path, &dest_path) {
            Ok(CopyOutcome::Copied) => report.copied += 1,
            Ok(CopyOutcome::Skipped) => report.skipped += 1,
            Err(err) => {
                report.failed += 1;
                report.errors.push(format!(
                    "{}: {}",
                    file_path.to_string_lossy(),
                    err
                ));
            }
        }
    }

    Ok(report)
}

fn normalized_exts(config: &Config) -> HashSet<String> {
    let mut exts = HashSet::new();
    for ext in config.video_exts.iter().chain(config.photo_exts.iter()) {
        exts.insert(ext.trim().trim_start_matches('.').to_ascii_lowercase());
    }
    exts
}

fn list_media_files(
    sd_root: &Path,
    camera_profile: &Option<CameraProfile>,
    exts: &HashSet<String>,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    let roots = match camera_profile {
        Some(camera) if !camera.media_roots.is_empty() => camera
            .media_roots
            .iter()
            .map(|root| sd_root.join(root))
            .collect::<Vec<_>>(),
        _ => vec![sd_root.to_path_buf()],
    };

    for root in roots {
        if !root.exists() {
            continue;
        }
        for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
            if !entry.file_type().is_file() {
                continue;
            }
            let ext = entry
                .path()
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_ascii_lowercase();
            if exts.contains(&ext) {
                files.push(entry.path().to_path_buf());
            }
        }
    }

    Ok(files)
}

fn detect_camera(sd_root: &Path, config: &Config) -> Option<CameraProfile> {
    for camera in &config.cameras {
        if camera.signature_paths.is_empty() {
            continue;
        }
        let mut matched = true;
        for rel in &camera.signature_paths {
            if !sd_root.join(rel).exists() {
                matched = false;
                break;
            }
        }
        if matched {
            return Some(camera.clone());
        }
    }
    None
}

fn find_card_mapping(sd_root: &Path, config: &Config) -> Option<SdCardMapping> {
    let normalized_root = fs::canonicalize(sd_root).ok();
    for mapping in &config.sd_cards {
        let map_path = PathBuf::from(&mapping.root);
        let normalized_map = fs::canonicalize(&map_path).ok();
        if normalized_root.is_some() && normalized_root == normalized_map {
            return Some(mapping.clone());
        }
        if normalized_root.is_none() && map_path == sd_root {
            return Some(mapping.clone());
        }
    }
    None
}

fn resolve_camera_name(file_path: &Path, fallback: &str) -> String {
    if let Some(model) = read_camera_model(file_path) {
        return model;
    }
    fallback.to_string()
}

fn read_camera_model(file_path: &Path) -> Option<String> {
    let ext = file_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    if is_photo_ext(&ext) {
        if let Ok(model) = read_exif_model(file_path) {
            return Some(model);
        }
    } else if is_video_ext(&ext) {
        if let Some(model) = read_xmp_model(file_path) {
            return Some(model);
        }
    }

    None
}

fn is_photo_ext(ext: &str) -> bool {
    matches!(ext, "jpg" | "jpeg" | "tif" | "tiff" | "dng" | "heic" | "png")
}

fn is_video_ext(ext: &str) -> bool {
    matches!(ext, "mp4" | "mov" | "mxf" | "avi" | "mkv" | "mts" | "m2ts")
}

fn read_exif_model(file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let file = fs::File::open(file_path)?;
    let mut bufreader = BufReader::new(&file);
    let exif = Reader::new().read_from_container(&mut bufreader)?;
    let model = exif
        .get_field(Tag::Model, In::PRIMARY)
        .map(|f| f.display_value().with_unit(&exif).to_string());
    let make = exif
        .get_field(Tag::Make, In::PRIMARY)
        .map(|f| f.display_value().with_unit(&exif).to_string());

    match (make, model) {
        (Some(make), Some(model)) => Ok(format!("{} {}", make.trim(), model.trim())),
        (_, Some(model)) => Ok(model.trim().to_string()),
        (Some(make), None) => Ok(make.trim().to_string()),
        _ => Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            "EXIF model not found",
        ))),
    }
}

fn read_xmp_model(file_path: &Path) -> Option<String> {
    let xmp_path = file_path.with_extension("xmp");
    let contents = fs::read(&xmp_path).ok()?;
    let text = String::from_utf8_lossy(&contents);

    let model = extract_xmp_value(&text, "Model");
    let make = extract_xmp_value(&text, "Make");
    match (make, model) {
        (Some(make), Some(model)) => Some(format!("{} {}", make.trim(), model.trim())),
        (_, Some(model)) => Some(model.trim().to_string()),
        (Some(make), None) => Some(make.trim().to_string()),
        _ => None,
    }
}

fn extract_xmp_value(text: &str, tag: &str) -> Option<String> {
    let attr_key = format!("tiff:{}=\"", tag);
    if let Some(start) = text.find(&attr_key) {
        let rest = &text[start + attr_key.len()..];
        if let Some(end) = rest.find('"') {
            return Some(rest[..end].to_string());
        }
    }

    let open_tag = format!("<tiff:{}>", tag);
    let close_tag = format!("</tiff:{}>", tag);
    if let Some(start) = text.find(&open_tag) {
        let rest = &text[start + open_tag.len()..];
        if let Some(end) = rest.find(&close_tag) {
            return Some(rest[..end].to_string());
        }
    }
    None
}

fn sanitize_component(value: &str) -> String {
    let mut out = String::new();
    let mut last_was_sep = false;
    for ch in value.chars() {
        let mapped = match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' => {
                last_was_sep = false;
                Some(ch)
            }
            _ => Some('_'),
        };
        if let Some(ch) = mapped {
            if ch == '_' {
                if last_was_sep {
                    continue;
                }
                last_was_sep = true;
            }
            out.push(ch);
        }
    }
    let trimmed = out.trim_matches('_').to_string();
    if trimmed.is_empty() {
        "UnknownCamera".to_string()
    } else {
        trimmed
    }
}

enum CopyOutcome {
    Copied,
    Skipped,
}

fn copy_with_dedup(src: &Path, dest: &Path) -> Result<CopyOutcome, Box<dyn std::error::Error>> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut final_dest = dest.to_path_buf();
    if final_dest.exists() {
        let hash_match = hash::check_file(src.to_path_buf(), final_dest.clone()).is_ok();
        if hash_match {
            return Ok(CopyOutcome::Skipped);
        }
        final_dest = unique_path(dest);
    }

    fs::copy(src, &final_dest)?;
    hash::check_file(src.to_path_buf(), final_dest)?;
    Ok(CopyOutcome::Copied)
}

fn unique_path(original: &Path) -> PathBuf {
    let parent = original.parent().unwrap_or_else(|| Path::new(""));
    let stem = original
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("file");
    let ext = original.extension().and_then(|s| s.to_str()).unwrap_or("");
    for i in 1..=9999 {
        let candidate = if ext.is_empty() {
            parent.join(format!("{}_{}", stem, i))
        } else {
            parent.join(format!("{}_{}.{}", stem, i, ext))
        };
        if !candidate.exists() {
            return candidate;
        }
    }
    original.to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use tempfile::tempdir;

    #[test]
    fn sanitize_component_collapses_non_alnum() {
        let value = "Sony A7SIII/Body";
        let sanitized = sanitize_component(value);
        assert_eq!(sanitized, "Sony_A7SIII_Body");
    }

    #[test]
    fn extract_xmp_value_reads_attribute() {
        let text = r#"<rdf:Description tiff:Model="FX3" tiff:Make="Sony"></rdf:Description>"#;
        let model = extract_xmp_value(text, "Model");
        assert_eq!(model, Some("FX3".to_string()));
    }

    #[test]
    fn unique_path_adds_suffix_when_exists() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("clip.mp4");
        fs::write(&path, b"data").expect("write file");
        let unique = unique_path(&path);
        assert_eq!(unique.file_name().unwrap().to_string_lossy(), "clip_1.mp4");
    }

    #[test]
    fn list_media_files_scans_roots() {
        let dir = tempdir().expect("tempdir");
        let dcim = dir.path().join("DCIM");
        let private = dir.path().join("PRIVATE/M4ROOT");
        fs::create_dir_all(&dcim).expect("mkdir");
        fs::create_dir_all(&private).expect("mkdir");
        fs::write(dcim.join("photo.JPG"), b"photo").expect("write photo");
        fs::write(private.join("video.MP4"), b"video").expect("write video");
        fs::write(dir.path().join("ignore.txt"), b"skip").expect("write text");

        let profile = CameraProfile {
            name: "TestCam".to_string(),
            signature_paths: vec![],
            media_roots: vec!["DCIM".to_string(), "PRIVATE/M4ROOT".to_string()],
        };
        let mut exts = HashSet::new();
        exts.insert("jpg".to_string());
        exts.insert("mp4".to_string());

        let files = list_media_files(dir.path(), &Some(profile), &exts).expect("scan files");
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn read_xmp_model_from_sidecar() {
        let dir = tempdir().expect("tempdir");
        let video = dir.path().join("clip.mp4");
        fs::write(&video, b"video").expect("write video");
        let xmp = dir.path().join("clip.xmp");
        let xmp_content = r#"<rdf:Description tiff:Make="Canon" tiff:Model="R5"></rdf:Description>"#;
        fs::write(&xmp, xmp_content).expect("write xmp");

        let model = read_xmp_model(&video).expect("model");
        assert_eq!(model, "Canon R5");
    }
}
