//! Icon database: maps extensions and special filenames to Nerd Font icons.

use crate::Icon;

/// Lookup an icon by file extension (lowercase, without the leading dot).
pub fn by_extension(ext: &str) -> Option<Icon> {
    // Match on lowercase extension
    match ext {
        // ── Rust ──
        "rs" => Some(Icon::new("\u{e7a8}", "rust", "#dea584")),

        // ── Python ──
        "py" => Some(Icon::new("\u{e73c}", "python", "#ffbc03")),
        "pyc" | "pyo" => Some(Icon::new("\u{e73c}", "python-compiled", "#ffe291")),
        "pyd" => Some(Icon::new("\u{e73c}", "python-dll", "#ffe291")),
        "pyi" => Some(Icon::new("\u{e73c}", "python-stub", "#ffbc03")),
        "pyw" => Some(Icon::new("\u{e73c}", "python-window", "#ffbc03")),

        // ── JavaScript / TypeScript ──
        "js" => Some(Icon::new("\u{e74e}", "javascript", "#cbcb41")),
        "mjs" => Some(Icon::new("\u{e74e}", "javascript-module", "#cbcb41")),
        "cjs" => Some(Icon::new("\u{e74e}", "javascript-commonjs", "#cbcb41")),
        "ts" => Some(Icon::new("\u{e628}", "typescript", "#519aba")),
        "mts" => Some(Icon::new("\u{e628}", "typescript-module", "#519aba")),
        "cts" => Some(Icon::new("\u{e628}", "typescript-commonjs", "#519aba")),
        "tsx" => Some(Icon::new("\u{e7ba}", "typescriptreact", "#519aba")),
        "jsx" => Some(Icon::new("\u{e7ba}", "javascriptreact", "#cbcb41")),
        "d.ts" => Some(Icon::new("\u{e628}", "typescript-declaration", "#519aba")),

        // ── Go ──
        "go" => Some(Icon::new("\u{e627}", "go", "#519aba")),
        "mod" => Some(Icon::new("\u{e627}", "go-module", "#519aba")),
        "sum" => Some(Icon::new("\u{e627}", "go-checksum", "#519aba")),

        // ── C / C++ ──
        "c" => Some(Icon::new("\u{e61e}", "c", "#599eff")),
        "h" => Some(Icon::new("\u{e61e}", "c-header", "#a074c4")),
        "cpp" | "cc" | "cxx" => Some(Icon::new("\u{e61d}", "cpp", "#f34b7d")),
        "hpp" | "hh" | "hxx" => Some(Icon::new("\u{e61d}", "cpp-header", "#a074c4")),

        // ── C# ──
        "cs" => Some(Icon::new("\u{f031b}", "csharp", "#596706")),
        "csx" => Some(Icon::new("\u{f031b}", "csharp-script", "#596706")),

        // ── Java / Kotlin / Scala ──
        "java" => Some(Icon::new("\u{e738}", "java", "#cc3e44")),
        "jar" => Some(Icon::new("\u{e738}", "java-archive", "#cc3e44")),
        "class" => Some(Icon::new("\u{e738}", "java-class", "#cc3e44")),
        "kt" | "kts" => Some(Icon::new("\u{e634}", "kotlin", "#7f52ff")),
        "scala" | "sc" => Some(Icon::new("\u{e737}", "scala", "#cc3e44")),
        "clj" | "cljs" | "cljc" => Some(Icon::new("\u{e768}", "clojure", "#8dc149")),

        // ── Ruby ──
        "rb" => Some(Icon::new("\u{e739}", "ruby", "#701516")),
        "erb" => Some(Icon::new("\u{e739}", "ruby-erb", "#701516")),
        "gemspec" => Some(Icon::new("\u{e739}", "ruby-gemspec", "#701516")),

        // ── PHP ──
        "php" => Some(Icon::new("\u{e73d}", "php", "#a074c4")),

        // ── Swift / Objective-C ──
        "swift" => Some(Icon::new("\u{e755}", "swift", "#e37933")),
        "m" => Some(Icon::new("\u{e61e}", "objective-c", "#599eff")),
        "mm" => Some(Icon::new("\u{e61d}", "objective-cpp", "#f34b7d")),

        // ── Lua ──
        "lua" => Some(Icon::new("\u{e620}", "lua", "#51a0cf")),

        // ── Nix ──
        "nix" => Some(Icon::new("\u{f313}", "nix", "#7ebae4")),

        // ── Shell ──
        "sh" => Some(Icon::new("\u{e795}", "shell", "#4d5a5e")),
        "bash" => Some(Icon::new("\u{e795}", "bash", "#4d5a5e")),
        "zsh" => Some(Icon::new("\u{e795}", "zsh", "#4d5a5e")),
        "fish" => Some(Icon::new("\u{e795}", "fish", "#4d5a5e")),
        "ksh" | "csh" | "tcsh" => Some(Icon::new("\u{e795}", "shell-variant", "#4d5a5e")),
        "ps1" | "psm1" | "psd1" => Some(Icon::new("\u{e795}", "powershell", "#4273ca")),
        "bat" | "cmd" => Some(Icon::new("\u{e795}", "batch", "#c1f12e")),

        // ── Haskell / Elixir / Erlang ──
        "hs" | "lhs" => Some(Icon::new("\u{e777}", "haskell", "#a074c4")),
        "ex" | "exs" => Some(Icon::new("\u{e62d}", "elixir", "#a074c4")),
        "erl" | "hrl" => Some(Icon::new("\u{e7b1}", "erlang", "#b83998")),

        // ── Zig / Nim / V / D / R ──
        "zig" => Some(Icon::new("\u{e6a9}", "zig", "#f69a1b")),
        "nim" | "nims" | "nimble" => Some(Icon::new("\u{e677}", "nim", "#ffc200")),
        "v" => Some(Icon::new("\u{e6ac}", "vlang", "#5d87bf")),
        "d" => Some(Icon::new("\u{e7af}", "dlang", "#b03931")),
        "r" | "rdata" | "rds" => Some(Icon::new("\u{f25d}", "r", "#2266ba")),

        // ── Data / Config ──
        "json" => Some(Icon::new("\u{e60b}", "json", "#cbcb41")),
        "jsonc" => Some(Icon::new("\u{e60b}", "jsonc", "#cbcb41")),
        "json5" => Some(Icon::new("\u{e60b}", "json5", "#cbcb41")),
        "jsonl" | "ndjson" => Some(Icon::new("\u{e60b}", "jsonlines", "#cbcb41")),
        "yaml" | "yml" => Some(Icon::new("\u{e6a8}", "yaml", "#cb171e")),
        "toml" => Some(Icon::new("\u{e6b2}", "toml", "#9c4221")),
        "xml" => Some(Icon::new("\u{f05c0}", "xml", "#e37933")),
        "csv" => Some(Icon::new("\u{f1c3}", "csv", "#89e051")),
        "tsv" => Some(Icon::new("\u{f1c3}", "tsv", "#89e051")),
        "ini" | "cfg" | "conf" => Some(Icon::new("\u{e615}", "config", "#6d8086")),
        "env" => Some(Icon::new("\u{f462}", "env", "#faf743")),
        "properties" => Some(Icon::new("\u{e615}", "properties", "#6d8086")),

        // ── Markup / Documentation ──
        "md" | "markdown" => Some(Icon::new("\u{e73e}", "markdown", "#519aba")),
        "mdx" => Some(Icon::new("\u{e73e}", "mdx", "#519aba")),
        "rst" => Some(Icon::new("\u{f1c2}", "restructuredtext", "#519aba")),
        "adoc" | "asciidoc" => Some(Icon::new("\u{f1c2}", "asciidoc", "#519aba")),
        "tex" | "ltx" => Some(Icon::new("\u{e69b}", "latex", "#3d6117")),
        "bib" => Some(Icon::new("\u{e69b}", "bibtex", "#3d6117")),
        "txt" => Some(Icon::new("\u{f15c}", "text", "#89e051")),
        "org" => Some(Icon::new("\u{e633}", "org-mode", "#77aa99")),

        // ── Web ──
        "html" | "htm" => Some(Icon::new("\u{e736}", "html", "#e44d26")),
        "css" => Some(Icon::new("\u{e749}", "css", "#563d7c")),
        "scss" => Some(Icon::new("\u{e749}", "scss", "#f55385")),
        "sass" => Some(Icon::new("\u{e749}", "sass", "#f55385")),
        "less" => Some(Icon::new("\u{e749}", "less", "#563d7c")),
        "styl" | "stylus" => Some(Icon::new("\u{e749}", "stylus", "#563d7c")),
        "vue" => Some(Icon::new("\u{f0844}", "vue", "#8dc149")),
        "svelte" => Some(Icon::new("\u{e697}", "svelte", "#ff3e00")),
        "astro" => Some(Icon::new("\u{e6b8}", "astro", "#ff5d01")),
        "wasm" => Some(Icon::new("\u{e6a1}", "wasm", "#654ff0")),

        // ── SQL / Database ──
        "sql" => Some(Icon::new("\u{e706}", "sql", "#dad8d8")),
        "sqlite" | "sqlite3" | "db" => Some(Icon::new("\u{e706}", "database", "#dad8d8")),

        // ── GraphQL / Protobuf / gRPC ──
        "graphql" | "gql" => Some(Icon::new("\u{e662}", "graphql", "#e535ab")),
        "proto" => Some(Icon::new("\u{e6b1}", "protobuf", "#6d8086")),

        // ── Docker / Containers ──
        "dockerfile" => Some(Icon::new("\u{f308}", "dockerfile", "#384d54")),

        // ── Git ──
        "gitignore" | "gitattributes" | "gitmodules" | "gitconfig" => {
            Some(Icon::new("\u{e702}", "git", "#f14c28"))
        }

        // ── Lock files ──
        "lock" => Some(Icon::new("\u{f023}", "lock", "#6d8086")),

        // ── Images ──
        "png" => Some(Icon::new("\u{f1c5}", "png", "#a074c4")),
        "jpg" | "jpeg" => Some(Icon::new("\u{f1c5}", "jpeg", "#a074c4")),
        "gif" => Some(Icon::new("\u{f1c5}", "gif", "#a074c4")),
        "bmp" => Some(Icon::new("\u{f1c5}", "bmp", "#a074c4")),
        "webp" => Some(Icon::new("\u{f1c5}", "webp", "#a074c4")),
        "ico" => Some(Icon::new("\u{f1c5}", "ico", "#a074c4")),
        "svg" => Some(Icon::new("\u{f1c5}", "svg", "#ffb13b")),
        "tif" | "tiff" => Some(Icon::new("\u{f1c5}", "tiff", "#a074c4")),
        "avif" => Some(Icon::new("\u{f1c5}", "avif", "#a074c4")),
        "heic" | "heif" => Some(Icon::new("\u{f1c5}", "heic", "#a074c4")),
        "psd" => Some(Icon::new("\u{e7b8}", "photoshop", "#519aba")),
        "ai" => Some(Icon::new("\u{e7b4}", "illustrator", "#ffb13b")),
        "xd" => Some(Icon::new("\u{e7b4}", "xd", "#ff61f6")),
        "fig" | "figma" => Some(Icon::new("\u{e7b4}", "figma", "#a259ff")),
        "sketch" => Some(Icon::new("\u{e7b4}", "sketch", "#ffb13b")),

        // ── Video ──
        "mp4" | "m4v" => Some(Icon::new("\u{f1c8}", "mp4", "#fd971f")),
        "avi" => Some(Icon::new("\u{f1c8}", "avi", "#fd971f")),
        "mkv" => Some(Icon::new("\u{f1c8}", "mkv", "#fd971f")),
        "mov" => Some(Icon::new("\u{f1c8}", "mov", "#fd971f")),
        "webm" => Some(Icon::new("\u{f1c8}", "webm", "#fd971f")),
        "flv" => Some(Icon::new("\u{f1c8}", "flv", "#fd971f")),
        "wmv" => Some(Icon::new("\u{f1c8}", "wmv", "#fd971f")),

        // ── Audio ──
        "mp3" => Some(Icon::new("\u{f1c7}", "mp3", "#d39ede")),
        "wav" => Some(Icon::new("\u{f1c7}", "wav", "#d39ede")),
        "flac" => Some(Icon::new("\u{f1c7}", "flac", "#d39ede")),
        "ogg" => Some(Icon::new("\u{f1c7}", "ogg", "#d39ede")),
        "aac" => Some(Icon::new("\u{f1c7}", "aac", "#d39ede")),
        "m4a" => Some(Icon::new("\u{f1c7}", "m4a", "#d39ede")),
        "wma" => Some(Icon::new("\u{f1c7}", "wma", "#d39ede")),
        "opus" => Some(Icon::new("\u{f1c7}", "opus", "#d39ede")),
        "mid" | "midi" => Some(Icon::new("\u{f1c7}", "midi", "#d39ede")),

        // ── Archives ──
        "zip" => Some(Icon::new("\u{f1c6}", "zip", "#eca517")),
        "tar" => Some(Icon::new("\u{f1c6}", "tar", "#eca517")),
        "gz" | "gzip" => Some(Icon::new("\u{f1c6}", "gzip", "#eca517")),
        "bz2" => Some(Icon::new("\u{f1c6}", "bzip2", "#eca517")),
        "xz" => Some(Icon::new("\u{f1c6}", "xz", "#eca517")),
        "zst" | "zstd" => Some(Icon::new("\u{f1c6}", "zstd", "#eca517")),
        "rar" => Some(Icon::new("\u{f1c6}", "rar", "#eca517")),
        "7z" => Some(Icon::new("\u{f1c6}", "7zip", "#eca517")),
        "dmg" => Some(Icon::new("\u{f1c6}", "dmg", "#eca517")),
        "iso" => Some(Icon::new("\u{f1c6}", "iso", "#eca517")),
        "deb" => Some(Icon::new("\u{f1c6}", "deb", "#eca517")),
        "rpm" => Some(Icon::new("\u{f1c6}", "rpm", "#eca517")),

        // ── Fonts ──
        "ttf" => Some(Icon::new("\u{f031}", "truetype-font", "#a074c4")),
        "otf" => Some(Icon::new("\u{f031}", "opentype-font", "#a074c4")),
        "woff" => Some(Icon::new("\u{f031}", "woff", "#a074c4")),
        "woff2" => Some(Icon::new("\u{f031}", "woff2", "#a074c4")),
        "eot" => Some(Icon::new("\u{f031}", "eot", "#a074c4")),

        // ── Documents ──
        "pdf" => Some(Icon::new("\u{f1c1}", "pdf", "#b30b00")),
        "doc" | "docx" => Some(Icon::new("\u{f1c2}", "word", "#185abd")),
        "xls" | "xlsx" => Some(Icon::new("\u{f1c3}", "excel", "#207245")),
        "ppt" | "pptx" => Some(Icon::new("\u{f1c4}", "powerpoint", "#cb4a32")),
        "odt" => Some(Icon::new("\u{f1c2}", "odt", "#185abd")),
        "ods" => Some(Icon::new("\u{f1c3}", "ods", "#207245")),
        "odp" => Some(Icon::new("\u{f1c4}", "odp", "#cb4a32")),

        // ── Templating ──
        "hbs" | "handlebars" => Some(Icon::new("\u{e60f}", "handlebars", "#f0772b")),
        "ejs" => Some(Icon::new("\u{e618}", "ejs", "#cbcb41")),
        "pug" | "jade" => Some(Icon::new("\u{e60e}", "pug", "#a86454")),
        "mustache" => Some(Icon::new("\u{e60f}", "mustache", "#e37933")),
        "jinja" | "jinja2" | "j2" => Some(Icon::new("\u{e60f}", "jinja", "#b41717")),
        "njk" | "nunjucks" => Some(Icon::new("\u{e60f}", "nunjucks", "#1c4913")),

        // ── Build / CI ──
        "cmake" => Some(Icon::new("\u{e615}", "cmake", "#6d8086")),
        "make" | "mk" => Some(Icon::new("\u{e615}", "makefile", "#6d8086")),
        "gradle" => Some(Icon::new("\u{e660}", "gradle", "#02303a")),
        "sbt" => Some(Icon::new("\u{e737}", "sbt", "#cc3e44")),

        // ── Terraform / IaC ──
        "tf" | "tfvars" => Some(Icon::new("\u{e69a}", "terraform", "#5f43e9")),
        "hcl" => Some(Icon::new("\u{e69a}", "hcl", "#5f43e9")),

        // ── Jupyter ──
        "ipynb" => Some(Icon::new("\u{e678}", "jupyter", "#f37726")),

        // ── Binary / Object ──
        "o" => Some(Icon::new("\u{e6a8}", "object-file", "#6d8086")),
        "so" | "dylib" | "dll" => Some(Icon::new("\u{e6a8}", "shared-lib", "#6d8086")),
        "a" | "lib" => Some(Icon::new("\u{e6a8}", "static-lib", "#6d8086")),
        "exe" => Some(Icon::new("\u{f17a}", "executable", "#6d8086")),
        "whl" => Some(Icon::new("\u{e73c}", "python-wheel", "#ffbc03")),

        // ── Misc ──
        "log" => Some(Icon::new("\u{f18d}", "log", "#6d8086")),
        "diff" | "patch" => Some(Icon::new("\u{e728}", "diff", "#41535b")),
        "editorconfig" => Some(Icon::new("\u{e615}", "editorconfig", "#6d8086")),

        _ => None,
    }
}

/// Lookup an icon by exact filename (case-insensitive).
pub fn by_filename(name: &str) -> Option<Icon> {
    let lower = name.to_ascii_lowercase();
    match lower.as_str() {
        // ── Docker ──
        "dockerfile" | "dockerfile.dev" | "dockerfile.prod" | "containerfile" => {
            Some(Icon::new("\u{f308}", "dockerfile", "#384d54"))
        }
        "docker-compose.yml"
        | "docker-compose.yaml"
        | "compose.yml"
        | "compose.yaml" => Some(Icon::new("\u{f308}", "docker-compose", "#384d54")),
        ".dockerignore" => Some(Icon::new("\u{f308}", "dockerignore", "#384d54")),

        // ── Makefile ──
        "makefile" | "gnumakefile" | "bsdmakefile" => {
            Some(Icon::new("\u{e615}", "makefile", "#6d8086"))
        }
        "cmakelists.txt" => Some(Icon::new("\u{e615}", "cmake", "#6d8086")),
        "justfile" => Some(Icon::new("\u{e615}", "justfile", "#6d8086")),
        "rakefile" => Some(Icon::new("\u{e739}", "rakefile", "#701516")),
        "taskfile.yml" | "taskfile.yaml" => Some(Icon::new("\u{e615}", "taskfile", "#6d8086")),

        // ── Git ──
        ".gitignore" => Some(Icon::new("\u{e702}", "gitignore", "#f14c28")),
        ".gitattributes" => Some(Icon::new("\u{e702}", "gitattributes", "#f14c28")),
        ".gitmodules" => Some(Icon::new("\u{e702}", "gitmodules", "#f14c28")),
        ".gitconfig" => Some(Icon::new("\u{e702}", "gitconfig", "#f14c28")),
        ".gitkeep" => Some(Icon::new("\u{e702}", "gitkeep", "#f14c28")),

        // ── Rust ──
        "cargo.toml" => Some(Icon::new("\u{e7a8}", "cargo-toml", "#dea584")),
        "cargo.lock" => Some(Icon::new("\u{e7a8}", "cargo-lock", "#dea584")),
        "build.rs" => Some(Icon::new("\u{e7a8}", "rust-build-script", "#dea584")),
        "rust-toolchain" | "rust-toolchain.toml" => {
            Some(Icon::new("\u{e7a8}", "rust-toolchain", "#dea584"))
        }
        "clippy.toml" | ".clippy.toml" => Some(Icon::new("\u{e7a8}", "clippy-config", "#dea584")),
        "rustfmt.toml" | ".rustfmt.toml" => {
            Some(Icon::new("\u{e7a8}", "rustfmt-config", "#dea584"))
        }

        // ── Node / JS ecosystem ──
        "package.json" => Some(Icon::new("\u{e71e}", "npm-package", "#cb3837")),
        "package-lock.json" => Some(Icon::new("\u{e71e}", "npm-lock", "#cb3837")),
        "yarn.lock" => Some(Icon::new("\u{e6a7}", "yarn-lock", "#2c8ebb")),
        "pnpm-lock.yaml" => Some(Icon::new("\u{e71e}", "pnpm-lock", "#f69220")),
        "bun.lockb" | "bun.lock" => Some(Icon::new("\u{e71e}", "bun-lock", "#fbf0df")),
        ".npmrc" => Some(Icon::new("\u{e71e}", "npmrc", "#cb3837")),
        ".nvmrc" => Some(Icon::new("\u{e71e}", "nvmrc", "#8cc84b")),
        ".node-version" => Some(Icon::new("\u{e71e}", "node-version", "#8cc84b")),
        "tsconfig.json"
        | "tsconfig.build.json"
        | "tsconfig.app.json"
        | "tsconfig.spec.json" => Some(Icon::new("\u{e628}", "tsconfig", "#519aba")),
        ".eslintrc"
        | ".eslintrc.js"
        | ".eslintrc.cjs"
        | ".eslintrc.json"
        | ".eslintrc.yml"
        | ".eslintrc.yaml"
        | "eslint.config.js"
        | "eslint.config.mjs"
        | "eslint.config.cjs"
        | "eslint.config.ts" => Some(Icon::new("\u{e60c}", "eslint", "#4b32c3")),
        ".prettierrc"
        | ".prettierrc.json"
        | ".prettierrc.yml"
        | ".prettierrc.yaml"
        | ".prettierrc.js"
        | ".prettierrc.cjs"
        | ".prettierrc.toml"
        | "prettier.config.js"
        | "prettier.config.cjs" => Some(Icon::new("\u{e615}", "prettier", "#56b3b4")),
        ".prettierignore" => Some(Icon::new("\u{e615}", "prettierignore", "#56b3b4")),
        "webpack.config.js"
        | "webpack.config.ts"
        | "webpack.config.cjs"
        | "webpack.config.mjs" => Some(Icon::new("\u{f072b}", "webpack", "#8dd6f9")),
        "vite.config.js"
        | "vite.config.ts"
        | "vite.config.mjs"
        | "vite.config.mts" => Some(Icon::new("\u{e6b4}", "vite", "#646cff")),
        "rollup.config.js"
        | "rollup.config.ts"
        | "rollup.config.mjs" => Some(Icon::new("\u{e615}", "rollup", "#ec4a3f")),
        "babel.config.js"
        | "babel.config.cjs"
        | "babel.config.json"
        | ".babelrc"
        | ".babelrc.json" => Some(Icon::new("\u{e615}", "babel", "#f5da55")),
        "jest.config.js"
        | "jest.config.ts"
        | "jest.config.cjs"
        | "jest.config.mjs"
        | "jest.config.json" => Some(Icon::new("\u{e615}", "jest", "#99424f")),
        "vitest.config.js"
        | "vitest.config.ts"
        | "vitest.config.mts" => Some(Icon::new("\u{e615}", "vitest", "#729b1b")),

        // ── Python ──
        "requirements.txt" | "constraints.txt" => {
            Some(Icon::new("\u{e73c}", "python-requirements", "#ffbc03"))
        }
        "setup.py" | "setup.cfg" => Some(Icon::new("\u{e73c}", "python-setup", "#ffbc03")),
        "pyproject.toml" => Some(Icon::new("\u{e73c}", "pyproject", "#ffbc03")),
        "pipfile" | "pipfile.lock" => Some(Icon::new("\u{e73c}", "pipfile", "#ffbc03")),
        "tox.ini" => Some(Icon::new("\u{e73c}", "tox", "#ffbc03")),
        ".flake8" => Some(Icon::new("\u{e73c}", "flake8", "#ffbc03")),
        ".pylintrc" | "pylintrc" => Some(Icon::new("\u{e73c}", "pylint", "#ffbc03")),
        "poetry.lock" => Some(Icon::new("\u{e73c}", "poetry-lock", "#ffbc03")),

        // ── Ruby ──
        "gemfile" | "gemfile.lock" => Some(Icon::new("\u{e739}", "gemfile", "#701516")),

        // ── Nix ──
        "flake.nix" => Some(Icon::new("\u{f313}", "flake-nix", "#7ebae4")),
        "flake.lock" => Some(Icon::new("\u{f313}", "flake-lock", "#7ebae4")),
        "default.nix" => Some(Icon::new("\u{f313}", "default-nix", "#7ebae4")),
        "shell.nix" => Some(Icon::new("\u{f313}", "shell-nix", "#7ebae4")),

        // ── Go ──
        "go.mod" => Some(Icon::new("\u{e627}", "go-mod", "#519aba")),
        "go.sum" => Some(Icon::new("\u{e627}", "go-sum", "#519aba")),

        // ── Misc config ──
        ".editorconfig" => Some(Icon::new("\u{e615}", "editorconfig", "#6d8086")),
        ".env"
        | ".env.local"
        | ".env.development"
        | ".env.production"
        | ".env.test"
        | ".env.staging"
        | ".env.example" => Some(Icon::new("\u{f462}", "env", "#faf743")),
        "license" | "licence" | "license.md" | "licence.md" | "license.txt"
        | "licence.txt" => Some(Icon::new("\u{e60a}", "license", "#d0bf41")),
        "readme" | "readme.md" | "readme.txt" | "readme.rst" => {
            Some(Icon::new("\u{e73e}", "readme", "#519aba"))
        }
        "changelog" | "changelog.md" | "changes" | "changes.md" | "history.md" => {
            Some(Icon::new("\u{e609}", "changelog", "#519aba"))
        }
        "contributing" | "contributing.md" => {
            Some(Icon::new("\u{e609}", "contributing", "#519aba"))
        }
        "authors" | "authors.md" | "contributors" | "contributors.md" => {
            Some(Icon::new("\u{e609}", "authors", "#519aba"))
        }
        ".mailmap" => Some(Icon::new("\u{e609}", "mailmap", "#519aba")),

        // ── CI/CD ──
        ".travis.yml" => Some(Icon::new("\u{e615}", "travis-ci", "#cb3837")),
        "jenkinsfile" => Some(Icon::new("\u{e767}", "jenkins", "#6d8086")),
        ".gitlab-ci.yml" => Some(Icon::new("\u{f296}", "gitlab-ci", "#e24329")),
        "renovate.json" | "renovate.json5" | ".renovaterc" | ".renovaterc.json" => {
            Some(Icon::new("\u{e615}", "renovate", "#1a8cff"))
        }
        "dependabot.yml" => Some(Icon::new("\u{e615}", "dependabot", "#0366d6")),

        // ── Kubernetes / Helm ──
        "chart.yaml" | "chart.yml" => {
            Some(Icon::new("\u{f286}", "helm-chart", "#0f1689"))
        }
        "helmfile.yaml" | "helmfile.yml" => {
            Some(Icon::new("\u{f286}", "helmfile", "#0f1689"))
        }
        "kustomization.yaml" | "kustomization.yml" => {
            Some(Icon::new("\u{f286}", "kustomize", "#326ce5"))
        }

        _ => None,
    }
}

/// Default icon when nothing else matches.
pub fn default_icon() -> Icon {
    Icon::new("\u{f15b}", "default", "#6d8086")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extension_coverage() {
        // Verify we have a substantial number of extensions covered
        let extensions = [
            "rs", "py", "js", "ts", "tsx", "jsx", "go", "c", "cpp", "h", "java", "rb", "lua",
            "nix", "sh", "bash", "zsh", "json", "yaml", "yml", "toml", "md", "html", "css",
            "scss", "sql", "graphql", "proto", "png", "jpg", "mp4", "mp3", "zip", "tar", "gz",
            "pdf", "svg", "vue", "svelte", "swift", "kt", "scala", "hs", "ex", "zig", "r",
            "php", "cs", "tf", "ipynb", "wasm", "dockerfile", "lock", "log", "diff", "ttf",
            "otf", "woff", "woff2", "exe", "dll", "so", "bat", "ini", "env", "csv", "xml",
        ];
        for ext in &extensions {
            assert!(
                by_extension(ext).is_some(),
                "Missing icon for extension: {ext}"
            );
        }
    }

    #[test]
    fn filename_coverage() {
        let filenames = [
            "Dockerfile",
            "Makefile",
            ".gitignore",
            "Cargo.toml",
            "Cargo.lock",
            "package.json",
            "yarn.lock",
            "flake.nix",
            "go.mod",
            ".editorconfig",
            ".env",
            "LICENSE",
            "README.md",
        ];
        for name in &filenames {
            assert!(
                by_filename(name).is_some(),
                "Missing icon for filename: {name}"
            );
        }
    }

    #[test]
    fn default_icon_is_valid() {
        let icon = default_icon();
        assert!(!icon.glyph.is_empty());
        assert!(!icon.name.is_empty());
        assert!(icon.color.starts_with('#'));
    }
}
