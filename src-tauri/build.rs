use std::fs;
use std::path::Path;

fn main() {
    // Single source of truth for the document extension: tauri.conf.json's
    // bundle.fileAssociations[0].ext[0]. Compile it in so Rust can recognize
    // files the OS hands us to open. Change the extension THERE and rebuild.
    println!("cargo:rerun-if-changed=tauri.conf.json");
    let ext = read_doc_ext().unwrap_or_else(|| "appdoc".to_string());
    println!("cargo:rustc-env=APP_DOC_EXT={ext}");

    // Build date for the About dialog.
    let build_date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    println!("cargo:rustc-env=BUILD_DATE={build_date}");

    tauri_build::build()
}

fn read_doc_ext() -> Option<String> {
    let raw = fs::read_to_string(Path::new("tauri.conf.json")).ok()?;
    let v: serde_json::Value = serde_json::from_str(&raw).ok()?;
    v.get("bundle")?
        .get("fileAssociations")?
        .get(0)?
        .get("ext")?
        .get(0)?
        .as_str()
        .map(str::to_string)
}
