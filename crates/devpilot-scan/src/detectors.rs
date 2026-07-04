//! Pure manifest detectors: given a manifest's name and text content, they
//! extract dependencies and recognize frameworks. No I/O, no async — trivial
//! to unit-test on string inputs.

use devpilot_core::entities::{Dependency, Detection, Ecosystem, Framework, FrameworkCategory};

/// Recognizes a framework from an npm package name.
fn npm_framework(name: &str) -> Option<(&'static str, FrameworkCategory)> {
    match name {
        "react" => Some(("React", FrameworkCategory::Frontend)),
        "vue" => Some(("Vue", FrameworkCategory::Frontend)),
        "svelte" => Some(("Svelte", FrameworkCategory::Frontend)),
        "@angular/core" => Some(("Angular", FrameworkCategory::Frontend)),
        "solid-js" => Some(("SolidJS", FrameworkCategory::Frontend)),
        "next" => Some(("Next.js", FrameworkCategory::Fullstack)),
        "nuxt" => Some(("Nuxt", FrameworkCategory::Fullstack)),
        "express" => Some(("Express", FrameworkCategory::Backend)),
        "@nestjs/core" => Some(("NestJS", FrameworkCategory::Backend)),
        "fastify" => Some(("Fastify", FrameworkCategory::Backend)),
        "@tauri-apps/api" => Some(("Tauri", FrameworkCategory::Desktop)),
        "electron" => Some(("Electron", FrameworkCategory::Desktop)),
        _ => None,
    }
}

/// Recognizes a framework from a Cargo crate name.
fn cargo_framework(name: &str) -> Option<(&'static str, FrameworkCategory)> {
    match name {
        "tauri" => Some(("Tauri", FrameworkCategory::Desktop)),
        "axum" => Some(("Axum", FrameworkCategory::Backend)),
        "actix-web" => Some(("Actix Web", FrameworkCategory::Backend)),
        "rocket" => Some(("Rocket", FrameworkCategory::Backend)),
        "warp" => Some(("Warp", FrameworkCategory::Backend)),
        "leptos" => Some(("Leptos", FrameworkCategory::Frontend)),
        "yew" => Some(("Yew", FrameworkCategory::Frontend)),
        _ => None,
    }
}

/// Recognizes a framework from a Python package name (lowercased).
fn python_framework(name: &str) -> Option<(&'static str, FrameworkCategory)> {
    match name {
        "django" => Some(("Django", FrameworkCategory::Backend)),
        "flask" => Some(("Flask", FrameworkCategory::Backend)),
        "fastapi" => Some(("FastAPI", FrameworkCategory::Backend)),
        _ => None,
    }
}

/// Recognizes a framework from a Go module path.
fn go_framework(path: &str) -> Option<(&'static str, FrameworkCategory)> {
    if path.contains("gin-gonic/gin") {
        Some(("Gin", FrameworkCategory::Backend))
    } else if path.contains("labstack/echo") {
        Some(("Echo", FrameworkCategory::Backend))
    } else if path.contains("gofiber/fiber") {
        Some(("Fiber", FrameworkCategory::Backend))
    } else {
        None
    }
}

/// Builds a framework entry attributed to `source`.
fn framework(entry: (&'static str, FrameworkCategory), source: &str) -> Framework {
    Framework {
        name: entry.0.to_string(),
        category: entry.1,
        source: source.to_string(),
    }
}

/// Detects dependencies and frameworks from a `package.json`.
pub fn detect_npm(content: &str) -> Detection {
    let mut detection = Detection::default();
    let Ok(value) = serde_json::from_str::<serde_json::Value>(content) else {
        return detection;
    };

    for section in ["dependencies", "devDependencies"] {
        let Some(map) = value.get(section).and_then(|value| value.as_object()) else {
            continue;
        };
        for (name, version) in map {
            detection.dependencies.push(Dependency {
                name: name.clone(),
                version: version.as_str().map(|value| value.to_string()),
                ecosystem: Ecosystem::Npm,
            });
            if let Some(entry) = npm_framework(name) {
                detection.frameworks.push(framework(entry, "package.json"));
            }
        }
    }
    detection
}

/// Detects dependencies and frameworks from a `Cargo.toml`.
pub fn detect_cargo(content: &str) -> Detection {
    let mut detection = Detection::default();
    let Ok(value) = content.parse::<toml::Table>() else {
        return detection;
    };

    let Some(deps) = value.get("dependencies").and_then(|value| value.as_table()) else {
        return detection;
    };
    for (name, spec) in deps {
        let version = match spec {
            toml::Value::String(version) => Some(version.clone()),
            toml::Value::Table(table) => table
                .get("version")
                .and_then(|value| value.as_str())
                .map(|value| value.to_string()),
            _ => None,
        };
        detection.dependencies.push(Dependency {
            name: name.clone(),
            version,
            ecosystem: Ecosystem::Cargo,
        });
        if let Some(entry) = cargo_framework(name) {
            detection.frameworks.push(framework(entry, "Cargo.toml"));
        }
    }
    detection
}

/// Splits a PyPI requirement line into a name and optional version.
fn parse_requirement(line: &str) -> Option<(String, Option<String>)> {
    let line = line.split('#').next().unwrap_or("").trim();
    if line.is_empty() || line.starts_with('-') {
        return None;
    }
    // Name ends at the first version operator, extras bracket or whitespace.
    let split_at = line
        .find(|c: char| "=<>!~ [".contains(c))
        .unwrap_or(line.len());
    let name = line[..split_at].trim().to_ascii_lowercase();
    if name.is_empty() {
        return None;
    }
    let version = line[split_at..].trim();
    let version = if version.is_empty() {
        None
    } else {
        Some(version.to_string())
    };
    Some((name, version))
}

/// Records a Python dependency and any framework it implies.
fn push_python(detection: &mut Detection, name: String, version: Option<String>, source: &str) {
    if let Some(entry) = python_framework(&name) {
        detection.frameworks.push(framework(entry, source));
    }
    detection.dependencies.push(Dependency {
        name,
        version,
        ecosystem: Ecosystem::PyPI,
    });
}

/// Detects dependencies and frameworks from a `requirements.txt`.
pub fn detect_requirements(content: &str) -> Detection {
    let mut detection = Detection::default();
    for line in content.lines() {
        if let Some((name, version)) = parse_requirement(line) {
            push_python(&mut detection, name, version, "requirements.txt");
        }
    }
    detection
}

/// Detects dependencies and frameworks from a `pyproject.toml`.
pub fn detect_pyproject(content: &str) -> Detection {
    let mut detection = Detection::default();
    let Ok(value) = content.parse::<toml::Table>() else {
        return detection;
    };

    // PEP 621: [project] dependencies = ["django>=4", ...]
    if let Some(array) = value
        .get("project")
        .and_then(|project| project.get("dependencies"))
        .and_then(|deps| deps.as_array())
    {
        for item in array {
            if let Some((name, version)) = item.as_str().and_then(parse_requirement) {
                push_python(&mut detection, name, version, "pyproject.toml");
            }
        }
    }

    // Poetry: [tool.poetry.dependencies] as a table.
    if let Some(table) = value
        .get("tool")
        .and_then(|tool| tool.get("poetry"))
        .and_then(|poetry| poetry.get("dependencies"))
        .and_then(|deps| deps.as_table())
    {
        for (name, spec) in table {
            if name.eq_ignore_ascii_case("python") {
                continue;
            }
            let version = spec.as_str().map(|value| value.to_string());
            push_python(
                &mut detection,
                name.to_ascii_lowercase(),
                version,
                "pyproject.toml",
            );
        }
    }
    detection
}

/// Detects dependencies and frameworks from a `go.mod`.
pub fn detect_gomod(content: &str) -> Detection {
    let mut detection = Detection::default();
    let mut in_block = false;

    for raw in content.lines() {
        let line = raw.split("//").next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with("require (") {
            in_block = true;
            continue;
        }
        if in_block && line == ")" {
            in_block = false;
            continue;
        }

        let spec = if in_block {
            line
        } else if let Some(rest) = line.strip_prefix("require ") {
            rest.trim()
        } else {
            continue;
        };

        let mut parts = spec.split_whitespace();
        if let Some(path) = parts.next() {
            let version = parts.next().map(|value| value.to_string());
            if let Some(entry) = go_framework(path) {
                detection.frameworks.push(framework(entry, "go.mod"));
            }
            detection.dependencies.push(Dependency {
                name: path.to_string(),
                version,
                ecosystem: Ecosystem::Go,
            });
        }
    }
    detection
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn npm_detects_react_and_deps() {
        let content = r#"{
            "dependencies": { "react": "^18.0.0", "lodash": "^4" },
            "devDependencies": { "vite": "^6" }
        }"#;
        let detection = detect_npm(content);
        assert_eq!(detection.dependencies.len(), 3);
        assert_eq!(detection.frameworks.len(), 1);
        assert_eq!(detection.frameworks[0].name, "React");
        assert_eq!(
            detection.frameworks[0].category,
            FrameworkCategory::Frontend
        );
    }

    #[test]
    fn npm_ignores_malformed_json() {
        assert_eq!(detect_npm("{ not json"), Detection::default());
    }

    #[test]
    fn cargo_detects_tauri_and_versions() {
        let content = r#"
            [dependencies]
            tauri = { version = "2", features = [] }
            serde = "1"
        "#;
        let detection = detect_cargo(content);
        assert_eq!(detection.dependencies.len(), 2);
        assert!(detection
            .frameworks
            .iter()
            .any(|f| f.name == "Tauri" && f.category == FrameworkCategory::Desktop));
        let serde = detection
            .dependencies
            .iter()
            .find(|d| d.name == "serde")
            .unwrap();
        assert_eq!(serde.version.as_deref(), Some("1"));
    }

    #[test]
    fn requirements_parses_names_and_frameworks() {
        let content = "Django==4.2.1\nrequests>=2\n# comment\nflask\n";
        let detection = detect_requirements(content);
        assert_eq!(detection.dependencies.len(), 3);
        let names: Vec<&str> = detection
            .frameworks
            .iter()
            .map(|f| f.name.as_str())
            .collect();
        assert!(names.contains(&"Django"));
        assert!(names.contains(&"Flask"));
        let django = &detection.dependencies[0];
        assert_eq!(django.name, "django");
        assert_eq!(django.version.as_deref(), Some("==4.2.1"));
    }

    #[test]
    fn pyproject_handles_pep621_and_poetry() {
        let pep = r#"
            [project]
            dependencies = ["fastapi>=0.100", "uvicorn"]
        "#;
        let detection = detect_pyproject(pep);
        assert!(detection.frameworks.iter().any(|f| f.name == "FastAPI"));
        assert_eq!(detection.dependencies.len(), 2);
    }

    #[test]
    fn gomod_parses_block_and_frameworks() {
        let content = "module example\n\ngo 1.22\n\nrequire (\n\tgithub.com/gin-gonic/gin v1.9.1\n\tgithub.com/stretchr/testify v1.8.0 // indirect\n)\n";
        let detection = detect_gomod(content);
        assert_eq!(detection.dependencies.len(), 2);
        assert!(detection.frameworks.iter().any(|f| f.name == "Gin"));
    }

    #[test]
    fn gomod_parses_single_require() {
        let content = "module x\nrequire github.com/labstack/echo/v4 v4.11.0\n";
        let detection = detect_gomod(content);
        assert_eq!(detection.dependencies.len(), 1);
        assert!(detection.frameworks.iter().any(|f| f.name == "Echo"));
    }
}
