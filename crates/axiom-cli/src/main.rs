use axiom_content::manifest::LessonManifest;
use axiom_content::validator::validate_manifest;
use std::fs;
use std::path::Path;

fn main() {
    println!("=== AXIOM Mathematics Academy — Content Manifest Verification ===");

    let paths = [
        "content/lessons/F04/foundation.fractions.compare/manifest.yaml",
        "content/lessons/A01/abacus.orientation.place_value/manifest.yaml",
    ];

    let mut all_ok = true;
    for p in &paths {
        let path = Path::new(p);
        if !path.exists() {
            eprintln!("FAIL: Missing file {p}");
            all_ok = false;
            continue;
        }

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
