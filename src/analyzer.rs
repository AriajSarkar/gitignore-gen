//! Project analyzer for technology detection.
//!
//! Scans project directories to detect technologies, frameworks, and tools
//! based on configuration files, extensions, and directory structures.

use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;

/// Detection rule defining how to identify a technology.
struct DetectionRule {
    /// Technology name (must match template name)
    name: &'static str,
    /// File patterns that indicate this technology
    files: &'static [&'static str],
    /// File extensions that indicate this technology
    extensions: &'static [&'static str],
    /// Directory names that indicate this technology
    directories: &'static [&'static str],
}

/// All detection rules for supported technologies.
/// Names must match template names exactly (case-insensitive).
const DETECTION_RULES: &[DetectionRule] = &[
    // === Languages ===
    DetectionRule {
        name: "Rust",
        files: &["Cargo.toml", "Cargo.lock"],
        extensions: &["rs"],
        directories: &["target"],
    },
    DetectionRule {
        name: "Go",
        files: &["go.mod", "go.sum"],
        extensions: &["go"],
        directories: &["vendor"],
    },
    DetectionRule {
        name: "Python",
        files: &["requirements.txt", "setup.py", "pyproject.toml", "Pipfile"],
        extensions: &["py", "pyw"],
        directories: &["venv", ".venv", "__pycache__"],
    },
    DetectionRule {
        name: "Node",
        files: &["package.json", "package-lock.json", "yarn.lock", "pnpm-lock.yaml"],
        extensions: &["js", "mjs", "cjs"],
        directories: &["node_modules"],
    },
    DetectionRule { name: "Java", files: &[], extensions: &["java"], directories: &[] },
    DetectionRule { name: "Kotlin", files: &[], extensions: &["kt", "kts"], directories: &[] },
    DetectionRule {
        name: "Swift",
        files: &["Package.swift"],
        extensions: &["swift"],
        directories: &[],
    },
    DetectionRule {
        name: "C",
        files: &["Makefile", "CMakeLists.txt"],
        extensions: &["c", "h"],
        directories: &[],
    },
    DetectionRule {
        name: "C++",
        files: &["CMakeLists.txt"],
        extensions: &["cpp", "cxx", "cc", "hpp", "hxx"],
        directories: &[],
    },
    DetectionRule {
        name: "Ruby",
        files: &["Gemfile", "Gemfile.lock", "Rakefile"],
        extensions: &["rb"],
        directories: &[],
    },
    DetectionRule {
        name: "Dart",
        files: &["pubspec.yaml", "pubspec.lock"],
        extensions: &["dart"],
        directories: &[".dart_tool"],
    },
    DetectionRule {
        name: "Elixir",
        files: &["mix.exs"],
        extensions: &["ex", "exs"],
        directories: &["_build", "deps"],
    },
    DetectionRule {
        name: "Scala",
        files: &["build.sbt"],
        extensions: &["scala", "sc"],
        directories: &[],
    },
    DetectionRule {
        name: "Haskell",
        files: &["stack.yaml", "cabal.project"],
        extensions: &["hs", "lhs"],
        directories: &[".stack-work"],
    },
    DetectionRule { name: "Lua", files: &[], extensions: &["lua"], directories: &[] },
    DetectionRule {
        name: "R",
        files: &["DESCRIPTION", ".Rproj"],
        extensions: &["r", "R", "rmd"],
        directories: &[],
    },
    DetectionRule {
        name: "Julia",
        files: &["Project.toml", "Manifest.toml"],
        extensions: &["jl"],
        directories: &[],
    },
    DetectionRule {
        name: "Nim",
        files: &[],
        extensions: &["nim", "nims"],
        directories: &["nimcache"],
    },
    DetectionRule {
        name: "Zig",
        files: &["build.zig"],
        extensions: &["zig"],
        directories: &["zig-cache"],
    },
    DetectionRule {
        name: "OCaml",
        files: &["dune", "dune-project"],
        extensions: &["ml", "mli"],
        directories: &["_build"],
    },
    // === Build Tools ===
    DetectionRule { name: "Maven", files: &["pom.xml"], extensions: &[], directories: &[] },
    DetectionRule {
        name: "Gradle",
        files: &["build.gradle", "build.gradle.kts", "settings.gradle"],
        extensions: &[],
        directories: &[".gradle"],
    },
    DetectionRule {
        name: "CMake",
        files: &["CMakeLists.txt"],
        extensions: &["cmake"],
        directories: &["CMakeFiles"],
    },
    // === Frameworks ===
    DetectionRule {
        name: "Rails",
        files: &["config/routes.rb", "bin/rails"],
        extensions: &[],
        directories: &["app/controllers", "app/models"],
    },
    DetectionRule {
        name: "Flutter",
        files: &["pubspec.yaml"],
        extensions: &[],
        directories: &["android", "ios", "lib"],
    },
    DetectionRule { name: "Angular", files: &["angular.json"], extensions: &[], directories: &[] },
    DetectionRule {
        name: "Laravel",
        files: &["artisan"],
        extensions: &[],
        directories: &["app/Http", "resources/views"],
    },
    DetectionRule { name: "Django", files: &["manage.py"], extensions: &[], directories: &[] },
    // === Game Engines ===
    DetectionRule {
        name: "Unity",
        files: &[],
        extensions: &["unity", "prefab", "asset"],
        directories: &["Assets", "ProjectSettings"],
    },
    DetectionRule {
        name: "UnrealEngine",
        files: &[],
        extensions: &["uproject"],
        directories: &["Content", "Source"],
    },
    DetectionRule {
        name: "Godot",
        files: &["project.godot"],
        extensions: &["gd", "tscn"],
        directories: &[".godot"],
    },
    // === DevOps/Infrastructure ===
    DetectionRule {
        name: "Terraform",
        files: &[],
        extensions: &["tf", "tfvars"],
        directories: &[".terraform"],
    },
    DetectionRule {
        name: "Ansible",
        files: &["ansible.cfg", "playbook.yml"],
        extensions: &[],
        directories: &["roles", "group_vars"],
    },
    // === IDEs ===
    DetectionRule {
        name: "VisualStudio",
        files: &[],
        extensions: &["sln", "csproj", "vbproj"],
        directories: &[".vs"],
    },
    DetectionRule { name: "JetBrains", files: &[], extensions: &[], directories: &[".idea"] },
];

/// Analyzes a project directory and returns detected technologies.
///
/// Walks the directory tree and matches files/directories against detection rules.
/// Automatically skips hidden directories, submodules, and build outputs after detecting.
pub fn analyze_project(path: &Path) -> Vec<String> {
    use std::cell::RefCell;

    let detected = RefCell::new(HashSet::new());
    let max_depth = 3;

    let walker = WalkDir::new(path).max_depth(max_depth).into_iter();

    for entry in walker.filter_entry(|e| should_visit(e, &detected)).filter_map(Result::ok) {
        let name = entry.file_name().to_string_lossy();
        let is_dir = entry.file_type().is_dir();

        // Get relative path for path-based pattern matching
        let rel_path = entry
            .path()
            .strip_prefix(path)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_default();

        for rule in DETECTION_RULES {
            if matches_rule(rule, &name, &rel_path, is_dir) {
                detected.borrow_mut().insert(rule.name.to_string());
            }
        }
    }

    let mut result: Vec<_> = detected.into_inner().into_iter().collect();
    result.sort();
    result
}

/// Build output directories that indicate specific technologies
const BUILD_DIR_TECH: &[(&str, &str)] = &[
    ("target", "Rust"),
    ("node_modules", "Node"),
    ("__pycache__", "Python"),
    // Note: "vendor" removed - ambiguous (Go/PHP/Ruby). Rely on go.mod detection.
    ("_build", "Elixir"),
    ("deps", "Elixir"),
];

/// Smart directory filter - detects technology from build dirs, then skips them.
fn should_visit(entry: &walkdir::DirEntry, detected: &std::cell::RefCell<HashSet<String>>) -> bool {
    // Always visit files
    if !entry.file_type().is_dir() {
        return true;
    }

    let name = entry.file_name().to_string_lossy();
    let lower = name.to_lowercase();

    // Skip hidden directories (start with .)
    if name.starts_with('.') && entry.depth() > 0 {
        return false;
    }

    // Skip submodules
    let git_file = entry.path().join(".git");
    if git_file.exists() && entry.depth() > 0 {
        return false;
    }

    // Detect technology from build directories, then skip them
    for (dir, tech) in BUILD_DIR_TECH {
        if lower == *dir {
            detected.borrow_mut().insert(tech.to_string());
            return false; // Skip traversal but we detected!
        }
    }

    // Skip other common build outputs
    if matches!(lower.as_str(), "build" | "dist" | "out") {
        return false;
    }

    true
}

/// Check if a file/directory matches a detection rule.
/// Supports both base-name matching and path-based patterns (e.g., "config/routes.rb").
fn matches_rule(rule: &DetectionRule, name: &str, rel_path: &str, is_dir: bool) -> bool {
    if is_dir {
        // Check directory name
        if rule.directories.contains(&name) {
            return true;
        }
        // Check if relative path ends with any directory pattern (e.g., "app/controllers")
        for dir_pattern in rule.directories {
            if dir_pattern.contains('/') && rel_path.ends_with(dir_pattern) {
                return true;
            }
        }
        false
    } else {
        // Check exact file matches by name
        if rule.files.contains(&name) {
            return true;
        }
        // Check path-based patterns (e.g., "config/routes.rb")
        for file_pattern in rule.files {
            if file_pattern.contains('/') && rel_path.ends_with(file_pattern) {
                return true;
            }
        }
        // Check extension matches
        if let Some(ext) = name.rsplit('.').next() {
            if rule.extensions.contains(&ext) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_detect_rust() {
        let tmp = tempdir().unwrap();
        fs::write(tmp.path().join("Cargo.toml"), "").unwrap();

        let detected = analyze_project(tmp.path());
        assert!(detected.contains(&"Rust".to_string()));
    }

    #[test]
    fn test_detect_multiple() {
        let tmp = tempdir().unwrap();
        fs::write(tmp.path().join("Cargo.toml"), "").unwrap();
        fs::write(tmp.path().join("package.json"), "{}").unwrap();
        fs::write(tmp.path().join("go.mod"), "").unwrap();

        let detected = analyze_project(tmp.path());
        assert!(detected.contains(&"Rust".to_string()));
        assert!(detected.contains(&"Node".to_string()));
        assert!(detected.contains(&"Go".to_string()));
    }

    #[test]
    fn test_detect_by_extension() {
        let tmp = tempdir().unwrap();
        fs::write(tmp.path().join("main.py"), "").unwrap();

        let detected = analyze_project(tmp.path());
        assert!(detected.contains(&"Python".to_string()));
    }
}
