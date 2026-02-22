use std::fs;
use std::path::PathBuf;

pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
    pub modified: std::time::SystemTime,
}

impl FileEntry {
    pub fn from_path(path: PathBuf) -> Option<Self> {
        let metadata = fs::metadata(&path).ok()?;
        let name = path.file_name()?.to_string_lossy().to_string();

        Some(Self {
            is_dir: path.is_dir(),
            size: metadata.len(),
            modified: metadata.modified().ok()?,
            path,
            name,
        })
    }

    pub fn size_display(&self) -> String {
        if self.is_dir {
            return "—".to_string();
        }
        match self.size {
            0 => "0 B".to_string(),
            1..=1023 => format!("{} B", self.size),
            1024..=1048575 => format!("{:.1} KB", self.size as f64 / 1024.0),
            1048576..=1073741823 => format!("{:.1} MB", self.size as f64 / 1_048_576.0),
            _ => format!("{:.1} GB", self.size as f64 / 1_073_741_824.0),
        }
    }

    pub fn kind_display(&self) -> String {
        if self.is_dir {
            return "Folder".to_string();
        }
        match self.path.extension().and_then(|e| e.to_str()) {
            Some("txt") => "Text".to_string(),
            Some("png") => "PNG Image".to_string(),
            Some("jpg") | Some("jpeg") => "JPEG Image".to_string(),
            Some("pdf") => "PDF".to_string(),
            Some("zip") => "ZIP Archive".to_string(),
            Some("tar") => "TAR Archive".to_string(),
            Some("gz") => "GZ Archive".to_string(),
            Some("rs") => "Rust Source".to_string(),
            Some("toml") => "TOML File".to_string(),
            Some(ext) => format!("{} File", ext.to_uppercase()),
            None => "Unknown".to_string(),
        }
    }

    pub fn date_display(&self) -> String {
        use std::time::UNIX_EPOCH;
        let sec = self
            .modified
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let (y, mo, d, h, mi) = secs_to_date(sec);
        format!("{:04}-{:02}-{:02} {:02}:{:02}", y, mo, d, h, mi)
    }

    pub fn icon_name(&self) -> &'static str {
        if self.is_dir {
            return "folder-symbolic";
        }
        match self.path.extension().and_then(|e| e.to_str()) {
            Some("txt") => "text-x-generic",
            Some("png") | Some("jpg") | Some("jpeg") => "image-x-generic",
            Some("pdf") => "application-pdf",
            Some("zip") | Some("tar") | Some("gz") => "package-x-generic",
            Some("rs") | Some("py") | Some("js") => "text-x-script",
            _ => "application-x-generic",
        }
    }

    pub fn list_directory(path: &std::path::Path) -> Vec<FileEntry> {
        let Ok(entries) = fs::read_dir(path) else {
            return vec![];
        };

        let files: Vec<FileEntry> = entries
            .filter_map(|f| f.ok())
            .map(|e| e.path())
            .filter_map(FileEntry::from_path)
            .collect();

        files
    }
}

fn secs_to_date(secs: u64) -> (u64, u64, u64, u64, u64) {
    let min = secs / 60;
    let hour = min / 60;
    let days = hour / 24;
    let mi = min % 60;
    let h = hour % 24;

    let z = days + 719468;
    let era = z / 146097;
    let doe = z % 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let mo = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if mo <= 2 { y + 1 } else { y };

    (y, mo, d, h, mi)
}
