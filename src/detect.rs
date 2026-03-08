//! Filename detection: extracts extension and handles special filenames.

use std::path::Path;

/// Result of analyzing a filename for icon lookup purposes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileKind {
    /// A special filename that should be matched exactly (e.g., "Dockerfile", "Makefile").
    SpecialName(String),
    /// A file with a recognized extension.
    Extension(String),
    /// A dotfile with no further extension (e.g., ".bashrc").
    Dotfile(String),
    /// No extension or special name detected.
    Unknown,
}

/// Detect the kind of a file given its full path or filename.
///
/// Returns a `FileKind` that can be used to look up an icon.
/// Checks for special filenames first, then falls back to extension detection.
pub fn detect(filename: &str) -> FileKind {
    let basename = Path::new(filename)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(filename);

    // Check for special filenames first (exact match, case-insensitive)
    if is_special_filename(basename) {
        return FileKind::SpecialName(basename.to_string());
    }

    // Handle compound extensions like ".d.ts", ".test.js", ".spec.ts"
    if let Some(compound) = compound_extension(basename) {
        return FileKind::Extension(compound);
    }

    // Try regular extension
    if let Some(ext) = Path::new(basename).extension().and_then(|e| e.to_str()) {
        return FileKind::Extension(ext.to_ascii_lowercase());
    }

    // Dotfiles without extension (e.g., ".bashrc", ".zshrc")
    if basename.starts_with('.') && !basename[1..].contains('.') {
        return FileKind::Dotfile(basename[1..].to_string());
    }

    FileKind::Unknown
}

/// Extract the extension from a filename, handling compound extensions.
///
/// Returns the extension in lowercase without the leading dot.
pub fn extension(filename: &str) -> Option<String> {
    match detect(filename) {
        FileKind::Extension(ext) => Some(ext),
        _ => None,
    }
}

/// Check if a basename is a known special filename.
fn is_special_filename(basename: &str) -> bool {
    let lower = basename.to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "dockerfile"
            | "dockerfile.dev"
            | "dockerfile.prod"
            | "containerfile"
            | "docker-compose.yml"
            | "docker-compose.yaml"
            | "compose.yml"
            | "compose.yaml"
            | ".dockerignore"
            | "makefile"
            | "gnumakefile"
            | "bsdmakefile"
            | "cmakelists.txt"
            | "justfile"
            | "rakefile"
            | "taskfile.yml"
            | "taskfile.yaml"
            | ".gitignore"
            | ".gitattributes"
            | ".gitmodules"
            | ".gitconfig"
            | ".gitkeep"
            | "cargo.toml"
            | "cargo.lock"
            | "build.rs"
            | "rust-toolchain"
            | "rust-toolchain.toml"
            | "clippy.toml"
            | ".clippy.toml"
            | "rustfmt.toml"
            | ".rustfmt.toml"
            | "package.json"
            | "package-lock.json"
            | "yarn.lock"
            | "pnpm-lock.yaml"
            | "bun.lockb"
            | "bun.lock"
            | ".npmrc"
            | ".nvmrc"
            | ".node-version"
            | "tsconfig.json"
            | "tsconfig.build.json"
            | "tsconfig.app.json"
            | "tsconfig.spec.json"
            | ".eslintrc"
            | ".eslintrc.js"
            | ".eslintrc.cjs"
            | ".eslintrc.json"
            | ".eslintrc.yml"
            | ".eslintrc.yaml"
            | "eslint.config.js"
            | "eslint.config.mjs"
            | "eslint.config.cjs"
            | "eslint.config.ts"
            | ".prettierrc"
            | ".prettierrc.json"
            | ".prettierrc.yml"
            | ".prettierrc.yaml"
            | ".prettierrc.js"
            | ".prettierrc.cjs"
            | ".prettierrc.toml"
            | "prettier.config.js"
            | "prettier.config.cjs"
            | ".prettierignore"
            | "webpack.config.js"
            | "webpack.config.ts"
            | "webpack.config.cjs"
            | "webpack.config.mjs"
            | "vite.config.js"
            | "vite.config.ts"
            | "vite.config.mjs"
            | "vite.config.mts"
            | "rollup.config.js"
            | "rollup.config.ts"
            | "rollup.config.mjs"
            | "babel.config.js"
            | "babel.config.cjs"
            | "babel.config.json"
            | ".babelrc"
            | ".babelrc.json"
            | "jest.config.js"
            | "jest.config.ts"
            | "jest.config.cjs"
            | "jest.config.mjs"
            | "jest.config.json"
            | "vitest.config.js"
            | "vitest.config.ts"
            | "vitest.config.mts"
            | "requirements.txt"
            | "constraints.txt"
            | "setup.py"
            | "setup.cfg"
            | "pyproject.toml"
            | "pipfile"
            | "pipfile.lock"
            | "tox.ini"
            | ".flake8"
            | ".pylintrc"
            | "pylintrc"
            | "poetry.lock"
            | "gemfile"
            | "gemfile.lock"
            | "flake.nix"
            | "flake.lock"
            | "default.nix"
            | "shell.nix"
            | "go.mod"
            | "go.sum"
            | ".editorconfig"
            | ".env"
            | ".env.local"
            | ".env.development"
            | ".env.production"
            | ".env.test"
            | ".env.staging"
            | ".env.example"
            | "license"
            | "licence"
            | "license.md"
            | "licence.md"
            | "license.txt"
            | "licence.txt"
            | "readme"
            | "readme.md"
            | "readme.txt"
            | "readme.rst"
            | "changelog"
            | "changelog.md"
            | "changes"
            | "changes.md"
            | "history.md"
            | "contributing"
            | "contributing.md"
            | "authors"
            | "authors.md"
            | "contributors"
            | "contributors.md"
            | ".mailmap"
            | ".travis.yml"
            | "jenkinsfile"
            | ".gitlab-ci.yml"
            | "renovate.json"
            | "renovate.json5"
            | ".renovaterc"
            | ".renovaterc.json"
            | "dependabot.yml"
            | "chart.yaml"
            | "chart.yml"
            | "helmfile.yaml"
            | "helmfile.yml"
            | "kustomization.yaml"
            | "kustomization.yml"
    )
}

/// Check for compound extensions like ".d.ts".
fn compound_extension(basename: &str) -> Option<String> {
    // Strip leading dot for dotfiles
    let name = if basename.starts_with('.') {
        &basename[1..]
    } else {
        basename
    };

    // Known compound extensions (order matters — check longest first)
    let compounds = [
        ".d.ts",
        ".test.ts",
        ".test.tsx",
        ".test.js",
        ".test.jsx",
        ".spec.ts",
        ".spec.tsx",
        ".spec.js",
        ".spec.jsx",
        ".stories.tsx",
        ".stories.jsx",
        ".stories.ts",
        ".stories.js",
        ".module.css",
        ".module.scss",
        ".module.less",
        ".config.js",
        ".config.ts",
        ".config.mjs",
        ".config.cjs",
    ];

    let lower = name.to_ascii_lowercase();
    for compound in &compounds {
        if lower.ends_with(compound) {
            // Return without leading dot
            return Some(compound[1..].to_string());
        }
    }

    None
}

/// Map a dotfile name (without leading dot) to an extension for icon lookup.
pub fn dotfile_to_extension(dotfile: &str) -> Option<&'static str> {
    match dotfile {
        "bashrc" | "bash_profile" | "bash_aliases" | "bash_logout" => Some("bash"),
        "zshrc" | "zshenv" | "zprofile" | "zlogin" | "zlogout" => Some("zsh"),
        "profile" => Some("sh"),
        "vimrc" | "gvimrc" => Some("vim"),
        "tmux.conf" => Some("conf"),
        "inputrc" => Some("conf"),
        "curlrc" | "wgetrc" => Some("conf"),
        "screenrc" => Some("conf"),
        "direnvrc" => Some("sh"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_regular_extension() {
        assert_eq!(detect("main.rs"), FileKind::Extension("rs".to_string()));
        assert_eq!(
            detect("script.py"),
            FileKind::Extension("py".to_string())
        );
        assert_eq!(
            detect("style.css"),
            FileKind::Extension("css".to_string())
        );
    }

    #[test]
    fn detect_case_insensitive_extension() {
        assert_eq!(
            detect("README.MD"),
            FileKind::SpecialName("README.MD".to_string())
        );
        assert_eq!(
            detect("photo.JPG"),
            FileKind::Extension("jpg".to_string())
        );
    }

    #[test]
    fn detect_special_filenames() {
        assert_eq!(
            detect("Dockerfile"),
            FileKind::SpecialName("Dockerfile".to_string())
        );
        assert_eq!(
            detect("Makefile"),
            FileKind::SpecialName("Makefile".to_string())
        );
        assert_eq!(
            detect(".gitignore"),
            FileKind::SpecialName(".gitignore".to_string())
        );
        assert_eq!(
            detect("Cargo.toml"),
            FileKind::SpecialName("Cargo.toml".to_string())
        );
        assert_eq!(
            detect("package.json"),
            FileKind::SpecialName("package.json".to_string())
        );
    }

    #[test]
    fn detect_compound_extensions() {
        assert_eq!(
            detect("types.d.ts"),
            FileKind::Extension("d.ts".to_string())
        );
        assert_eq!(
            detect("app.test.tsx"),
            FileKind::Extension("test.tsx".to_string())
        );
        assert_eq!(
            detect("Button.stories.tsx"),
            FileKind::Extension("stories.tsx".to_string())
        );
        assert_eq!(
            detect("styles.module.css"),
            FileKind::Extension("module.css".to_string())
        );
    }

    #[test]
    fn detect_dotfiles() {
        assert_eq!(
            detect(".bashrc"),
            FileKind::Dotfile("bashrc".to_string())
        );
        assert_eq!(
            detect(".zshrc"),
            FileKind::Dotfile("zshrc".to_string())
        );
        assert_eq!(
            detect(".vimrc"),
            FileKind::Dotfile("vimrc".to_string())
        );
    }

    #[test]
    fn detect_with_path() {
        assert_eq!(
            detect("/home/user/project/src/main.rs"),
            FileKind::Extension("rs".to_string())
        );
        assert_eq!(
            detect("./src/Dockerfile"),
            FileKind::SpecialName("Dockerfile".to_string())
        );
    }

    #[test]
    fn detect_unknown() {
        assert_eq!(detect("noextension"), FileKind::Unknown);
    }

    #[test]
    fn extension_helper() {
        assert_eq!(extension("main.rs"), Some("rs".to_string()));
        assert_eq!(extension("Dockerfile"), None); // special name, not extension
        assert_eq!(extension("noext"), None);
    }

    #[test]
    fn dotfile_mapping() {
        assert_eq!(dotfile_to_extension("bashrc"), Some("bash"));
        assert_eq!(dotfile_to_extension("zshrc"), Some("zsh"));
        assert_eq!(dotfile_to_extension("unknown_dotfile"), None);
    }
}
