use axiom_content::manifest::LessonManifest;
use axiom_content::validator::validate_manifest;
use std::fs;
use std::path::Path;

fn main() {
    println!("=== AXIOM Mathematics Academy — Content Manifest Verification ===");

    fn find_manifests(dir: &Path, list: &mut Vec<std::path::PathBuf>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    find_manifests(&p, list);
                } else if p.file_name().and_then(|n| n.to_str()) == Some("manifest.yaml") {
                    list.push(p);
                }
            }
        }
    }

    let mut paths = Vec::new();
    find_manifests(Path::new("content/lessons"), &mut paths);
    if paths.is_empty() {
        find_manifests(Path::new("../../content/lessons"), &mut paths);
    }

    println!("Discovered {} lesson manifests.", paths.len());
    let mut all_ok = true;
    for path in &paths {
        let p = path.display().to_string();
        let raw = match fs::read_to_string(path) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("FAIL: Cannot read {p}: {e}");
                all_ok = false;
                continue;
            }
        };

        let manifest: LessonManifest = match serde_yaml::from_str(&raw) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("FAIL: YAML parse error in {p}: {e}");
                all_ok = false;
                continue;
            }
        };

        match validate_manifest(&manifest) {
            Ok(hash) => {
                println!("OK: [{}] {} (hash: {})", manifest.course_id, manifest.id, hash);
            }
            Err(err) => {
                eprintln!("FAIL: Validation error in {p}: {err}");
                all_ok = false;
            }
        }
    }

    if !all_ok {
        std::process::exit(1);
    }
    println!("=== All manifests verified successfully ===");
}
