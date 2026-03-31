use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

// keyword pattern
const SECRET_KEYWORDS: &[&str] = &[
    "secret",
    "password",
    "passwd",
    "token",
    "apikey",
    "api_key",
    "credential",
];

// value patterns that look like real secrets
const SECRET_PATTERNS: &[&str] = &[
    "sk_live_", "pk_live_", "ghp_", "gho_", "xoxb-", "xoxp-", "AKIA",
];

// files to scan
const TARGET_FILES: &[&str] = &[
    ".env.local",
    ".env.production",
    ".env.staging",
    ".env.development",
    ".env.test",
    ".env.backup",
];

// files to skip
const EXCLUDED_FILES: &[&str] = &[
    "README.md",
    "readme.md",
    ".env",
    ".env.example",
    "guardstack.config.toml",
    "package.json",
    "package-lock.json",
    "Cargo.toml",
    "Cargo.lock",
    ".gitIgnore",
];

// extensions to skip
const EXCLUDED_EXTENSIONS: &[&str] = &[".md", ".toml", ".lock", ".json", ".txt", ".yaml", ".yml"];

// directories to skip
const EXCLUDED_DIRS: &[&str] = &[
    "node_modules",
    ".git",
    "dist",
    "build",
    "target",
    "src",
    ".next",
    "coverage",
    ".cache",
];

pub struct ScanResult {
    pub file: String,
    pub line_number: usize,
    pub line: String,
    pub reason: String,
}

pub fn scan_files(custom_path: Option<&str>) -> Vec<ScanResult> {
    let mut results = Vec::new();
    match custom_path {
        Some(path) => {
            scan_directory(path, &mut results);
        }
        None => {
            // scan default target files
            for file in TARGET_FILES {
                if Path::new(file).exists() {
                    scan_single_file(file, &mut results);
                }
            }
        }
    }
    results
}

fn scan_directory(path: &str, results: &mut Vec<ScanResult>) {
    let dir = match std::fs::read_dir(path) {
        Ok(d) => d,
        Err(_) => return,
    };

    for entry in dir {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let entry_path = entry.path();
        let name = entry_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("default");

        // skip excluded directories
        if entry_path.is_dir() {
            if EXCLUDED_DIRS.iter().any(|d| *d == name) {
                continue;
            }

            // recurse into subdirectory!!
            if let Some(path_str) = entry_path.to_str() {
                scan_directory(path_str, results);
            }
            continue;
        }

        if let Some(path_str) = entry_path.to_str() {
            scan_single_file(path_str, results);
        }
    }
}

fn scan_single_file(path: &str, results: &mut Vec<ScanResult>) {
    // check excluded files
    let filename = Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    if EXCLUDED_FILES.iter().any(|f| *f == filename) {
        return;
    }

    // check excluded extensions
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    if EXCLUDED_EXTENSIONS
        .iter()
        .any(|e| e == &format!(".{}", ext).as_str())
    {
        return;
    }

    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return,
    };

    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        let line_number = index + 1;
        let lower = line.to_lowercase();
        let trimmed = line.trim();

        // skip empty lines
        if trimmed.is_empty() {
            continue;
        }

        // skip Rust specific definitions only
        let is_rust_definition = trimmed.starts_with("pub struct")
            || trimmed.starts_with("struct ")
            || trimmed.starts_with("pub fn")
            || trimmed.starts_with("fn ")
            || trimmed.starts_with("impl ")
            || trimmed.starts_with("use ");

        if is_rust_definition {
            continue;
        }

        // check if line has assignment
        let has_assignment = trimmed.contains('=') || trimmed.contains(':');

        // skip lines without assignment
        if !has_assignment {
            continue;
        }

        // detect comments
        let is_comment =
            trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.starts_with('*');

        // check keywords
        let keyword_match = SECRET_KEYWORDS.iter().find(|k| lower.contains(*k));

        // check patterns
        let pattern_match = SECRET_PATTERNS.iter().find(|p| line.contains(*p));

        if let Some(keyword) = keyword_match {
            results.push(ScanResult {
                file: path.to_string(),
                line_number,
                line: line.clone(),
                reason: if is_comment {
                    format!("possible leak in comment - keyword '{}'", keyword)
                } else {
                    format!("contains keyword '{}'", keyword)
                },
            });
        } else if let Some(pattern) = pattern_match {
            results.push(ScanResult {
                file: path.to_string(),
                line_number,
                line: line.clone(),
                reason: if is_comment {
                    format!("possible leak in comment - pattern '{}'", pattern)
                } else {
                    format!("contains secret pattern '{}'", pattern)
                },
            });
        }
    }
}
