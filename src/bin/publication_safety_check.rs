use std::fs;
use std::path::Path;
use std::process::Command;

const BINARY_EXTENSIONS: &[&str] = &[
    ".png", ".jpg", ".jpeg", ".gif", ".webp", ".wasm", ".zip", ".gz", ".tar", ".7z", ".pdf",
    ".exe", ".dll", ".node", ".vhd", ".vhdx", ".avhdx",
];

fn main() {
    let tracked_files = match git_ls_files() {
        Ok(files) => files,
        Err(error) => {
            eprintln!("publication safety check failed: {error}");
            std::process::exit(1);
        }
    };

    let mut failures = Vec::new();

    for file in &tracked_files {
        let normalized = normalize_path(file);
        if let Some(reason) = blocked_path_reason(&normalized) {
            failures.push(format!("blocked tracked path: {file} ({reason})"));
        }
    }

    for file in tracked_files.iter().filter(|file| is_text_candidate(file)) {
        let content = match fs::read_to_string(file) {
            Ok(content) => content,
            Err(_) => continue,
        };

        let lowered = content.to_lowercase();
        for pattern in blocked_content_patterns() {
            if lowered.contains(&pattern.to_lowercase()) {
                failures.push(format!(
                    "sensitive-looking content pattern '{pattern}' in tracked file: {file}"
                ));
            }
        }
    }

    if failures.is_empty() {
        println!("publication safety check passed.");
        return;
    }

    println!("publication safety check failed:");
    for failure in failures {
        println!("- {failure}");
    }
    std::process::exit(1);
}

fn git_ls_files() -> Result<Vec<String>, String> {
    let safe_directory = std::env::current_dir()
        .map_err(|error| format!("failed to resolve current directory: {error}"))?
        .display()
        .to_string()
        .replace('\\', "/");

    let output = Command::new("git")
        .arg("-c")
        .arg(format!("safe.directory={safe_directory}"))
        .arg("ls-files")
        .output()
        .map_err(|error| format!("failed to execute git ls-files: {error}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_owned());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect())
}

fn normalize_path(path: &str) -> String {
    path.replace('\\', "/").to_lowercase()
}

fn blocked_path_reason(path: &str) -> Option<&'static str> {
    let filename = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(path);

    let blocked_dirs = [
        "backups", "evidence", "refs", "reports", "patches", "tools", "state", "wallet",
    ];

    if path
        .split('/')
        .any(|segment| blocked_dirs.contains(&segment))
    {
        return Some("private operational directory");
    }

    let blocked_exact = [
        "wallet.db",
        "drk-testnet.toml",
        "darkfid-testnet.toml",
        "wsl_storage_file_search_raw.txt",
    ];

    if blocked_exact.contains(&filename) {
        return Some("private wallet, config, or raw discovery file");
    }

    let blocked_prefixes = ["hcs_"];
    let blocked_suffixes = [
        ".vhd",
        ".vhdx",
        ".avhdx",
        ".sqlite",
        ".sqlite3",
        ".db",
        ".tx",
        ".call",
        ".calls-map",
        ".seed",
        ".key",
        ".pem",
        ".secret",
        ".log",
    ];

    if blocked_prefixes
        .iter()
        .any(|prefix| filename.starts_with(prefix) && filename.contains("_raw"))
    {
        return Some("raw host/runtime discovery output");
    }

    if blocked_suffixes
        .iter()
        .any(|suffix| filename.ends_with(suffix))
    {
        return Some("private binary, database, transaction, key, or log artifact");
    }

    if filename.contains("darkfid-running") || filename.contains("xmrig-running") {
        return Some("local daemon/miner log");
    }

    None
}

fn is_text_candidate(path: &str) -> bool {
    let lowered = path.to_lowercase();
    !BINARY_EXTENSIONS
        .iter()
        .any(|extension| lowered.ends_with(extension))
}

fn blocked_content_patterns() -> Vec<String> {
    vec![
        concat!("github", "_pat_").to_owned(),
        concat!("gh", "p_").to_owned(),
        concat!("gh", "o_").to_owned(),
        concat!("gh", "u_").to_owned(),
        concat!("gh", "s_").to_owned(),
        concat!("begin", " rsa", " private", " key").to_owned(),
        concat!("begin", " openssh", " private", " key").to_owned(),
        concat!("begin", " private", " key").to_owned(),
        concat!("seed", " phrase:").to_owned(),
        concat!("seed", " phrase=").to_owned(),
        concat!("mnemonic", "=").to_owned(),
        concat!("mnemonic", ":").to_owned(),
        concat!("wallet", "_pass=").to_owned(),
        concat!("wallet", "_pass =").to_owned(),
        concat!("pass", "word=").to_owned(),
        concat!("pass", "word =").to_owned(),
        concat!("private", "_key=").to_owned(),
        concat!("private", "_key =").to_owned(),
        concat!("secret", "_key=").to_owned(),
        concat!("secret", "_key =").to_owned(),
        concat!("admin", "_secret=").to_owned(),
        concat!("admin", "_secret =").to_owned(),
        concat!("address", "_hash", "_secret=").to_owned(),
        concat!("address", "_hash", "_secret =").to_owned(),
        concat!("ip", "_hash", "_secret=").to_owned(),
        concat!("ip", "_hash", "_secret =").to_owned(),
        concat!("secret", " key:").to_owned(),
        concat!("x-access", "-token:").to_owned(),
        concat!("authorization:", " basic").to_owned(),
        concat!("authorization:", " bearer").to_owned(),
    ]
}
