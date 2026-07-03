use serde::{Deserialize, Serialize};

/// Programming language of a source file.
///
/// The initial set matches the languages DevPilot ships analysis grammars
/// for. Files in other languages are `Unknown`: they still receive
/// line-based metrics, but no AST metrics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    /// Rust (`.rs`).
    Rust,
    /// TypeScript, including TSX (`.ts`, `.tsx`).
    TypeScript,
    /// JavaScript, including JSX and module variants (`.js`, `.jsx`, `.mjs`, `.cjs`).
    JavaScript,
    /// Python (`.py`).
    Python,
    /// Go (`.go`).
    Go,
    /// Any language DevPilot has no grammar for.
    Unknown,
}

impl Language {
    /// Detects a language from a file extension (without the leading dot).
    ///
    /// Matching is case-insensitive. Unrecognized extensions yield
    /// [`Language::Unknown`].
    ///
    /// # Examples
    ///
    /// ```
    /// use devpilot_core::entities::Language;
    ///
    /// assert_eq!(Language::from_extension("rs"), Language::Rust);
    /// assert_eq!(Language::from_extension("TSX"), Language::TypeScript);
    /// assert_eq!(Language::from_extension("weird"), Language::Unknown);
    /// ```
    pub fn from_extension(extension: &str) -> Self {
        match extension.to_ascii_lowercase().as_str() {
            "rs" => Self::Rust,
            "ts" | "tsx" => Self::TypeScript,
            "js" | "jsx" | "mjs" | "cjs" => Self::JavaScript,
            "py" => Self::Python,
            "go" => Self::Go,
            _ => Self::Unknown,
        }
    }

    /// Human-readable language name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Rust => "Rust",
            Self::TypeScript => "TypeScript",
            Self::JavaScript => "JavaScript",
            Self::Python => "Python",
            Self::Go => "Go",
            Self::Unknown => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_known_extensions_case_insensitively() {
        assert_eq!(Language::from_extension("rs"), Language::Rust);
        assert_eq!(Language::from_extension("Py"), Language::Python);
        assert_eq!(Language::from_extension("MJS"), Language::JavaScript);
        assert_eq!(Language::from_extension("go"), Language::Go);
    }

    #[test]
    fn unknown_extension_maps_to_unknown() {
        assert_eq!(Language::from_extension("toml"), Language::Unknown);
        assert_eq!(Language::from_extension(""), Language::Unknown);
    }
}
