use walkdir::WalkDir;
use std::path::PathBuf;
use std::fs::{self, metadata};
use chrono::{DateTime, Utc, Duration};
use std::io;
use tracing_subscriber::FmtSubscriber;


// Tool functions for the Local File Manager Agent

// Directory scanning and file Discovery
pub async fn collect_files(base_dir: &str) -> Vec<PathBuf> {
    WalkDir::new(base_dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.into_path())
        .collect()
}


// File Metadata Extraction
#[derive(Debug)]
pub struct FileMetadata {
    pub path: PathBuf,
    pub size: u64,
    pub modified: DateTime<Utc>,
    pub extension: Option<String>,
}

pub fn get_file_metadata(path: &PathBuf) -> Option<FileMetadata> {
    if let Ok(metadata) = fs::metadata(&path) {
        let modified = metadata.modified().ok().and_then(|time| {
            time.elapsed().ok()
            .map(|elapsed| Utc::now() - Duration::from_std(elapsed).unwrap())
        }).unwrap_or(Utc::now());
        Some(FileMetadata {
            path: path.to_path_buf(),
            size: metadata.len(),
            modified,
            extension: path.extension().and_then(|ext| ext.to_str().map(|s| s.to_lowercase())),
        })
    } else {
        None
    }
}

// Designing File Routing Rules
pub fn route_file(file: &FileMetadata) -> Option<PathBuf> {
    match file.extension.as_deref() {
        Some("txt") => Some(PathBuf::from("TextFiles")),
        Some("jpg") | Some("png") => Some(PathBuf::from("Images")),
        Some("mp4") => Some(PathBuf::from("Videos")),
        Some("pdf") | Some("docx") => Some(PathBuf::from("Documents")),
        Some(_) if file.modified < Utc::now() - chrono::Duration::days(90) => {
            Some(PathBuf::from("Archive"))
        }
        _ => None
    }
}

// Moving Files Safely
pub async fn move_file(metadata: &FileMetadata, target_dir: &PathBuf) -> io::Result<()> {
    let file_name = metadata.path.file_name().unwrap();
    let dest_path = target_dir.join(file_name);

    if !target_dir.exists() {
        fs::create_dir_all(target_dir)?;
    }

    if dest_path.exists() {
        let new_name = format!("{}_{}", Utc::now().timestamp(), file_name.to_string_lossy());
        let new_dest_path = target_dir.join(new_name);
        fs::rename(&metadata.path, &new_dest_path)
    } else {
        fs::rename(&metadata.path, &dest_path)
    }
}

/*
// Log every move
tracing::info!(
    from = metadata.path.to_string_lossy(),
    to = dest_path.to_string_lossy(),
    "File moved successfully"
);
*/

// Auditable Logging
// this ensutres is recorded with timestamp and content. output can
// be directed to JSON logs or OpenTelemetry for further analysis.
pub fn init_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
    .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
