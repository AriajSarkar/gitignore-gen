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
/// Automatically skips hidden directories, submodules, and build outputs.
pub fn analyze_project(path: &Path) -> Vec<String> {
    let mut detected = HashSet::new();
    let max_depth = 3;

    for entry in WalkDir::new(path)
        .max_depth(max_depth)
        .into_iter()
        .filter_entry(should_visit)
        .filter_map(Result::ok)
    {
        let name = entry.file_name().to_string_lossy();
        let is_dir = entry.file_type().is_dir();

        for rule in DETECTION_RULES {
            if matches_rule(rule, &name, is_dir) {
                detected.insert(rule.name.to_string());
            }
        }
    }

    let mut result: Vec<_> = detected.into_iter().collect();
    result.sort();
    result
}

/// Smart directory filter - skips directories that shouldn't be part of project detection.
fn should_visit(entry: &walkdir::DirEntry) -> bool {
    // Always visit files
    if !entry.file_type().is_dir() {
        return true;
    }

    let name = entry.file_name().to_string_lossy();

    // Skip hidden directories (start with .)
    if name.starts_with('.') && entry.depth() > 0 {
        return false;
    }

    // Skip if directory contains a .git file (submodule indicator)
    let git_file = entry.path().join(".git");
    if git_file.exists() && entry.depth() > 0 {
        return false;
    }

    // Skip common build/dependency outputs by pattern
    let lower = name.to_lowercase();
    if matches!(
        lower.as_str(),
        "node_modules"
            | "target"
            | "build"
            | "dist"
            | "out"
            | "__pycache__"
            | "vendor"
            | "deps"
            | "_build"
    ) {
        return false;
    }

    true
}

/// Check if a file/directory matches a detection rule.
fn matches_rule(rule: &DetectionRule, name: &str, is_dir: bool) -> bool {
    if is_dir {
        rule.directories.contains(&name)
    } else {
        // Check exact file matches
        if rule.files.contains(&name) {
            return true;
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
