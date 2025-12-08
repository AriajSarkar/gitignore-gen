use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;

/// Analyzes a project directory and returns detected technologies
pub fn analyze_project(path: &Path) -> Vec<String> {
    let mut detected: Vec<String> = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let name = entry.file_name().to_string_lossy();

        if entry.file_type().is_dir() {
            match name.as_ref() {
                "node_modules" => detected.push("Node".to_string()),
                "venv" | ".venv" => detected.push("Python".to_string()),
                "vendor" => detected.push("Go".to_string()),
                _ => {}
            }
            continue;
        }

        // File-based detection
        if name.ends_with(".js") {
            detected.push("Node".to_string());
        } else if name.ends_with(".py") {
            detected.push("Python".to_string());
        } else if name.ends_with(".go") {
            detected.push("Go".to_string());
        } else if name.ends_with(".java") {
            detected.push("Java".to_string());
        } else if name.ends_with(".cs") {
            detected.push("VisualStudio".to_string());
        } else if name == "package.json" {
            detected.push("Node".to_string());
        } else if name == "requirements.txt" {
            detected.push("Python".to_string());
        } else if name == "go.mod" {
            detected.push("Go".to_string());
        } else if name == "pom.xml" {
            detected.push("Maven".to_string());
        } else if name == "Cargo.toml" {
            detected.push("Rust".to_string());
        }
    }

    unique(detected)
}

/// Removes duplicates from a vector while preserving order
fn unique(elements: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for v in elements {
        if seen.insert(v.clone()) {
            result.push(v);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_analyze_project() {
        let tmp_dir = tempdir().unwrap();
        let tmp_path = tmp_dir.path();

        // Create test files
        fs::write(tmp_path.join("main.go"), "").unwrap();
        fs::write(tmp_path.join("package.json"), "{}").unwrap();
        fs::write(tmp_path.join("app.py"), "").unwrap();
        fs::write(tmp_path.join("pom.xml"), "").unwrap();
        fs::write(tmp_path.join("Cargo.toml"), "").unwrap();

        let mut detected = analyze_project(tmp_path);
        detected.sort();

        let mut expected = vec!["Go", "Node", "Python", "Maven", "Rust"];
        expected.sort();

        assert_eq!(detected, expected);
    }

    #[test]
    fn test_unique() {
        let input = vec![
            "Go".to_string(),
            "Node".to_string(),
            "Go".to_string(),
            "Python".to_string(),
            "Node".to_string(),
        ];

        let result = unique(input);
        assert_eq!(result, vec!["Go", "Node", "Python"]);
    }
}
