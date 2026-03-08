//! Kamon (家紋) — file type icon provider with Nerd Font support.
//!
//! A pure Rust library that maps filenames and extensions to Nerd Font icons
//! and their associated colors. Designed to be a data/lookup library with no
//! external dependencies beyond `std`.
//!
//! # Usage
//!
//! ```
//! use kamon::{get_icon, get_icon_by_extension, Icon};
//!
//! // Look up by full filename (handles special names like Dockerfile, Makefile)
//! let icon = get_icon("main.rs").unwrap();
//! assert_eq!(icon.name, "rust");
//! assert_eq!(icon.color, "#dea584");
//!
//! // Look up by extension directly
//! let icon = get_icon_by_extension("py").unwrap();
//! assert_eq!(icon.name, "python");
//!
//! // Returns a default icon for unknown file types
//! let icon = get_icon("mystery_file");
//! assert!(icon.is_some()); // always returns an icon (default fallback)
//! ```

pub mod detect;
pub mod icons;

use detect::FileKind;

/// A file type icon with its Nerd Font glyph, human-readable name, and color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Icon {
    /// The Nerd Font glyph character(s) to display.
    pub glyph: &'static str,
    /// A human-readable name for this icon (e.g., "rust", "python", "dockerfile").
    pub name: &'static str,
    /// The suggested hex color for rendering (e.g., "#dea584").
    pub color: &'static str,
}

impl Icon {
    /// Create a new icon.
    #[must_use]
    pub const fn new(glyph: &'static str, name: &'static str, color: &'static str) -> Self {
        Self { glyph, name, color }
    }
}

impl std::fmt::Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.glyph)
    }
}

/// Look up an icon for a filename or path.
///
/// Tries the following in order:
/// 1. Exact filename match (e.g., "Dockerfile", "Cargo.toml", ".gitignore")
/// 2. Compound extension match (e.g., ".d.ts", ".test.tsx")
/// 3. Simple extension match (e.g., ".rs", ".py")
/// 4. Dotfile-to-extension mapping (e.g., ".bashrc" -> "bash")
/// 5. Default fallback icon
///
/// This function always returns `Some` — use [`try_get_icon`] if you need to
/// distinguish between a matched icon and the default.
#[must_use]
pub fn get_icon(filename: &str) -> Option<Icon> {
    Some(try_get_icon(filename).unwrap_or_else(icons::default_icon))
}

/// Look up an icon for a filename or path, returning `None` if no match is found.
///
/// Unlike [`get_icon`], this does not fall back to a default icon.
#[must_use]
pub fn try_get_icon(filename: &str) -> Option<Icon> {
    match detect::detect(filename) {
        FileKind::SpecialName(name) => icons::by_filename(&name),
        FileKind::Extension(ext) => icons::by_extension(&ext),
        FileKind::Dotfile(name) => {
            // Try mapping the dotfile to an extension
            detect::dotfile_to_extension(&name).and_then(icons::by_extension)
        }
        FileKind::Unknown => None,
    }
}

/// Look up an icon by file extension (without the leading dot).
///
/// ```
/// use kamon::get_icon_by_extension;
///
/// let icon = get_icon_by_extension("rs").unwrap();
/// assert_eq!(icon.name, "rust");
/// ```
#[must_use]
pub fn get_icon_by_extension(ext: &str) -> Option<Icon> {
    icons::by_extension(&ext.to_ascii_lowercase())
}

/// Get the default fallback icon used for unknown file types.
#[must_use]
pub fn default_icon() -> Icon {
    icons::default_icon()
}

/// Convenience: get an `(icon_glyph, color_hex)` tuple for a filename.
///
/// Always returns a value (falls back to default icon for unknown types).
#[must_use]
pub fn icon_and_color(filename: &str) -> (&'static str, &'static str) {
    let icon = get_icon(filename).unwrap_or_else(icons::default_icon);
    (icon.glyph, icon.color)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── get_icon: programming languages ──

    #[test]
    fn rust_file() {
        let icon = get_icon("main.rs").unwrap();
        assert_eq!(icon.name, "rust");
        assert_eq!(icon.color, "#dea584");
    }

    #[test]
    fn python_file() {
        let icon = get_icon("script.py").unwrap();
        assert_eq!(icon.name, "python");
        assert_eq!(icon.color, "#ffbc03");
    }

    #[test]
    fn javascript_file() {
        let icon = get_icon("app.js").unwrap();
        assert_eq!(icon.name, "javascript");
    }

    #[test]
    fn typescript_file() {
        let icon = get_icon("index.ts").unwrap();
        assert_eq!(icon.name, "typescript");
        assert_eq!(icon.color, "#519aba");
    }

    #[test]
    fn tsx_file() {
        let icon = get_icon("Component.tsx").unwrap();
        assert_eq!(icon.name, "typescriptreact");
    }

    #[test]
    fn jsx_file() {
        let icon = get_icon("Component.jsx").unwrap();
        assert_eq!(icon.name, "javascriptreact");
    }

    #[test]
    fn go_file() {
        let icon = get_icon("main.go").unwrap();
        assert_eq!(icon.name, "go");
    }

    #[test]
    fn c_file() {
        let icon = get_icon("main.c").unwrap();
        assert_eq!(icon.name, "c");
    }

    #[test]
    fn cpp_file() {
        let icon = get_icon("main.cpp").unwrap();
        assert_eq!(icon.name, "cpp");
    }

    #[test]
    fn java_file() {
        let icon = get_icon("App.java").unwrap();
        assert_eq!(icon.name, "java");
    }

    #[test]
    fn ruby_file() {
        let icon = get_icon("app.rb").unwrap();
        assert_eq!(icon.name, "ruby");
    }

    #[test]
    fn lua_file() {
        let icon = get_icon("init.lua").unwrap();
        assert_eq!(icon.name, "lua");
    }

    #[test]
    fn nix_file() {
        let icon = get_icon("configuration.nix").unwrap();
        assert_eq!(icon.name, "nix");
    }

    #[test]
    fn shell_files() {
        assert_eq!(get_icon("script.sh").unwrap().name, "shell");
        assert_eq!(get_icon("script.bash").unwrap().name, "bash");
        assert_eq!(get_icon("script.zsh").unwrap().name, "zsh");
    }

    #[test]
    fn swift_file() {
        let icon = get_icon("main.swift").unwrap();
        assert_eq!(icon.name, "swift");
    }

    #[test]
    fn kotlin_file() {
        let icon = get_icon("Main.kt").unwrap();
        assert_eq!(icon.name, "kotlin");
    }

    #[test]
    fn haskell_file() {
        let icon = get_icon("Main.hs").unwrap();
        assert_eq!(icon.name, "haskell");
    }

    #[test]
    fn elixir_file() {
        let icon = get_icon("app.ex").unwrap();
        assert_eq!(icon.name, "elixir");
    }

    #[test]
    fn zig_file() {
        let icon = get_icon("build.zig").unwrap();
        assert_eq!(icon.name, "zig");
    }

    #[test]
    fn php_file() {
        let icon = get_icon("index.php").unwrap();
        assert_eq!(icon.name, "php");
    }

    #[test]
    fn csharp_file() {
        let icon = get_icon("Program.cs").unwrap();
        assert_eq!(icon.name, "csharp");
    }

    // ── get_icon: data/config formats ──

    #[test]
    fn json_file() {
        let icon = get_icon("data.json").unwrap();
        assert_eq!(icon.name, "json");
    }

    #[test]
    fn yaml_file() {
        let icon = get_icon("config.yaml").unwrap();
        assert_eq!(icon.name, "yaml");

        let icon2 = get_icon("config.yml").unwrap();
        assert_eq!(icon2.name, "yaml");
    }

    #[test]
    fn toml_file() {
        let icon = get_icon("config.toml").unwrap();
        assert_eq!(icon.name, "toml");
    }

    #[test]
    fn xml_file() {
        let icon = get_icon("pom.xml").unwrap();
        assert_eq!(icon.name, "xml");
    }

    // ── get_icon: markup ──

    #[test]
    fn markdown_file() {
        let icon = get_icon("notes.md").unwrap();
        assert_eq!(icon.name, "markdown");
    }

    #[test]
    fn html_file() {
        let icon = get_icon("index.html").unwrap();
        assert_eq!(icon.name, "html");
    }

    #[test]
    fn css_file() {
        let icon = get_icon("styles.css").unwrap();
        assert_eq!(icon.name, "css");
    }

    #[test]
    fn scss_file() {
        let icon = get_icon("styles.scss").unwrap();
        assert_eq!(icon.name, "scss");
    }

    // ── get_icon: database/query ──

    #[test]
    fn sql_file() {
        let icon = get_icon("migration.sql").unwrap();
        assert_eq!(icon.name, "sql");
    }

    #[test]
    fn graphql_file() {
        let icon = get_icon("schema.graphql").unwrap();
        assert_eq!(icon.name, "graphql");
    }

    #[test]
    fn proto_file() {
        let icon = get_icon("service.proto").unwrap();
        assert_eq!(icon.name, "protobuf");
    }

    // ── get_icon: special filenames ──

    #[test]
    fn dockerfile() {
        let icon = get_icon("Dockerfile").unwrap();
        assert_eq!(icon.name, "dockerfile");
    }

    #[test]
    fn makefile() {
        let icon = get_icon("Makefile").unwrap();
        assert_eq!(icon.name, "makefile");
    }

    #[test]
    fn gitignore() {
        let icon = get_icon(".gitignore").unwrap();
        assert_eq!(icon.name, "gitignore");
    }

    #[test]
    fn cargo_toml() {
        let icon = get_icon("Cargo.toml").unwrap();
        assert_eq!(icon.name, "cargo-toml");
    }

    #[test]
    fn cargo_lock() {
        let icon = get_icon("Cargo.lock").unwrap();
        assert_eq!(icon.name, "cargo-lock");
    }

    #[test]
    fn package_json() {
        let icon = get_icon("package.json").unwrap();
        assert_eq!(icon.name, "npm-package");
    }

    #[test]
    fn yarn_lock() {
        let icon = get_icon("yarn.lock").unwrap();
        assert_eq!(icon.name, "yarn-lock");
    }

    #[test]
    fn flake_nix() {
        let icon = get_icon("flake.nix").unwrap();
        assert_eq!(icon.name, "flake-nix");
    }

    #[test]
    fn go_mod() {
        let icon = get_icon("go.mod").unwrap();
        assert_eq!(icon.name, "go-mod");
    }

    #[test]
    fn license() {
        let icon = get_icon("LICENSE").unwrap();
        assert_eq!(icon.name, "license");
    }

    #[test]
    fn readme() {
        let icon = get_icon("README.md").unwrap();
        assert_eq!(icon.name, "readme");
    }

    #[test]
    fn docker_compose() {
        let icon = get_icon("docker-compose.yml").unwrap();
        assert_eq!(icon.name, "docker-compose");
    }

    #[test]
    fn editorconfig() {
        let icon = get_icon(".editorconfig").unwrap();
        assert_eq!(icon.name, "editorconfig");
    }

    #[test]
    fn env_file() {
        let icon = get_icon(".env").unwrap();
        assert_eq!(icon.name, "env");
    }

    #[test]
    fn env_variants() {
        assert_eq!(get_icon(".env.local").unwrap().name, "env");
        assert_eq!(get_icon(".env.production").unwrap().name, "env");
        assert_eq!(get_icon(".env.development").unwrap().name, "env");
    }

    #[test]
    fn tsconfig() {
        let icon = get_icon("tsconfig.json").unwrap();
        assert_eq!(icon.name, "tsconfig");
    }

    #[test]
    fn rust_toolchain() {
        let icon = get_icon("rust-toolchain.toml").unwrap();
        assert_eq!(icon.name, "rust-toolchain");
    }

    #[test]
    fn helm_chart() {
        let icon = get_icon("Chart.yaml").unwrap();
        assert_eq!(icon.name, "helm-chart");
    }

    #[test]
    fn kustomization() {
        let icon = get_icon("kustomization.yaml").unwrap();
        assert_eq!(icon.name, "kustomize");
    }

    // ── get_icon: media files ──

    #[test]
    fn image_files() {
        assert_eq!(get_icon("photo.png").unwrap().name, "png");
        assert_eq!(get_icon("photo.jpg").unwrap().name, "jpeg");
        assert_eq!(get_icon("logo.svg").unwrap().name, "svg");
        assert_eq!(get_icon("icon.ico").unwrap().name, "ico");
    }

    #[test]
    fn video_files() {
        assert_eq!(get_icon("video.mp4").unwrap().name, "mp4");
        assert_eq!(get_icon("clip.mkv").unwrap().name, "mkv");
        assert_eq!(get_icon("movie.mov").unwrap().name, "mov");
    }

    #[test]
    fn audio_files() {
        assert_eq!(get_icon("song.mp3").unwrap().name, "mp3");
        assert_eq!(get_icon("track.flac").unwrap().name, "flac");
        assert_eq!(get_icon("sound.wav").unwrap().name, "wav");
    }

    // ── get_icon: archives ──

    #[test]
    fn archive_files() {
        assert_eq!(get_icon("file.zip").unwrap().name, "zip");
        assert_eq!(get_icon("file.tar").unwrap().name, "tar");
        assert_eq!(get_icon("file.gz").unwrap().name, "gzip");
        assert_eq!(get_icon("file.7z").unwrap().name, "7zip");
    }

    // ── get_icon: fonts ──

    #[test]
    fn font_files() {
        assert_eq!(get_icon("font.ttf").unwrap().name, "truetype-font");
        assert_eq!(get_icon("font.otf").unwrap().name, "opentype-font");
        assert_eq!(get_icon("font.woff2").unwrap().name, "woff2");
    }

    // ── get_icon: documents ──

    #[test]
    fn document_files() {
        assert_eq!(get_icon("paper.pdf").unwrap().name, "pdf");
        assert_eq!(get_icon("report.docx").unwrap().name, "word");
        assert_eq!(get_icon("data.xlsx").unwrap().name, "excel");
    }

    // ── get_icon: web frameworks ──

    #[test]
    fn vue_file() {
        let icon = get_icon("App.vue").unwrap();
        assert_eq!(icon.name, "vue");
    }

    #[test]
    fn svelte_file() {
        let icon = get_icon("App.svelte").unwrap();
        assert_eq!(icon.name, "svelte");
    }

    // ── get_icon: dotfiles ──

    #[test]
    fn dotfile_bashrc() {
        let icon = get_icon(".bashrc").unwrap();
        assert_eq!(icon.name, "bash");
    }

    #[test]
    fn dotfile_zshrc() {
        let icon = get_icon(".zshrc").unwrap();
        assert_eq!(icon.name, "zsh");
    }

    // ── get_icon: paths ──

    #[test]
    fn full_path() {
        let icon = get_icon("/home/user/project/src/main.rs").unwrap();
        assert_eq!(icon.name, "rust");
    }

    #[test]
    fn relative_path() {
        let icon = get_icon("./src/lib.rs").unwrap();
        assert_eq!(icon.name, "rust");
    }

    // ── get_icon: edge cases ──

    #[test]
    fn unknown_file_returns_default() {
        let icon = get_icon("mystery_file").unwrap();
        assert_eq!(icon.name, "default");
    }

    #[test]
    fn empty_string() {
        let icon = get_icon("").unwrap();
        assert_eq!(icon.name, "default");
    }

    // ── try_get_icon ──

    #[test]
    fn try_get_returns_none_for_unknown() {
        assert!(try_get_icon("mystery_file").is_none());
    }

    #[test]
    fn try_get_returns_some_for_known() {
        assert!(try_get_icon("main.rs").is_some());
    }

    // ── get_icon_by_extension ──

    #[test]
    fn by_extension_rs() {
        let icon = get_icon_by_extension("rs").unwrap();
        assert_eq!(icon.name, "rust");
    }

    #[test]
    fn by_extension_case_insensitive() {
        let icon = get_icon_by_extension("RS").unwrap();
        assert_eq!(icon.name, "rust");
    }

    #[test]
    fn by_extension_unknown() {
        assert!(get_icon_by_extension("xyz123").is_none());
    }

    // ── icon_and_color ──

    #[test]
    fn icon_and_color_tuple() {
        let (glyph, color) = icon_and_color("main.rs");
        assert!(!glyph.is_empty());
        assert!(color.starts_with('#'));
    }

    // ── Icon Display ──

    #[test]
    fn icon_display() {
        let icon = get_icon("main.rs").unwrap();
        let displayed = format!("{icon}");
        assert_eq!(displayed, icon.glyph);
    }

    // ── default_icon ──

    #[test]
    fn default_icon_valid() {
        let icon = default_icon();
        assert_eq!(icon.name, "default");
        assert!(!icon.glyph.is_empty());
        assert!(icon.color.starts_with('#'));
    }

    // ── Terraform / IaC ──

    #[test]
    fn terraform_file() {
        let icon = get_icon("main.tf").unwrap();
        assert_eq!(icon.name, "terraform");
    }

    // ── Jupyter ──

    #[test]
    fn jupyter_notebook() {
        let icon = get_icon("analysis.ipynb").unwrap();
        assert_eq!(icon.name, "jupyter");
    }

    // ── WASM ──

    #[test]
    fn wasm_file() {
        let icon = get_icon("module.wasm").unwrap();
        assert_eq!(icon.name, "wasm");
    }

    // ── Lock files ──

    #[test]
    fn generic_lock_file() {
        let icon = get_icon("something.lock").unwrap();
        assert_eq!(icon.name, "lock");
    }

    // ── CI/CD ──

    #[test]
    fn gitlab_ci() {
        let icon = get_icon(".gitlab-ci.yml").unwrap();
        assert_eq!(icon.name, "gitlab-ci");
    }

    #[test]
    fn jenkinsfile() {
        let icon = get_icon("Jenkinsfile").unwrap();
        assert_eq!(icon.name, "jenkins");
    }

    // ── Config tool files ──

    #[test]
    fn eslint_config() {
        let icon = get_icon("eslint.config.js").unwrap();
        assert_eq!(icon.name, "eslint");
    }

    #[test]
    fn prettier_config() {
        let icon = get_icon(".prettierrc").unwrap();
        assert_eq!(icon.name, "prettier");
    }

    #[test]
    fn vite_config() {
        let icon = get_icon("vite.config.ts").unwrap();
        assert_eq!(icon.name, "vite");
    }

    // ── All colors are valid hex ──

    #[test]
    fn all_extension_colors_are_hex() {
        let extensions = [
            "rs", "py", "js", "ts", "tsx", "jsx", "go", "c", "cpp", "h", "java", "rb", "lua",
            "nix", "sh", "json", "yaml", "toml", "md", "html", "css", "scss", "sql", "graphql",
            "proto", "png", "jpg", "svg", "mp4", "mp3", "zip", "pdf", "ttf", "vue", "svelte",
            "swift", "kt", "hs", "ex", "zig", "php", "cs", "tf", "ipynb", "wasm",
        ];
        for ext in &extensions {
            let icon = get_icon_by_extension(ext).unwrap();
            assert!(
                icon.color.starts_with('#') && (icon.color.len() == 4 || icon.color.len() == 7),
                "Invalid hex color for {ext}: {}",
                icon.color
            );
        }
    }

    // ── All glyphs are non-empty ──

    #[test]
    fn all_extension_glyphs_nonempty() {
        let extensions = [
            "rs", "py", "js", "ts", "go", "c", "cpp", "java", "rb", "lua", "nix", "sh", "json",
            "yaml", "toml", "md", "html", "css", "sql", "png", "mp4", "mp3", "zip", "pdf",
        ];
        for ext in &extensions {
            let icon = get_icon_by_extension(ext).unwrap();
            assert!(
                !icon.glyph.is_empty(),
                "Empty glyph for extension: {ext}"
            );
        }
    }
}
