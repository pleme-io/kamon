//! kamon — CLI for rendering pleme-io design tokens.
//!
//! ```text
//! kamon render --target css --out tokens.css
//! kamon render --target tailwind --out tailwind.config.js
//! kamon render-all --out-dir generated/
//! kamon hash                   # print BLAKE3-esque content hash
//! kamon targets                # list supported targets
//! ```

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use kamon_render::Target;
use kamon_tokens::TokenSet;

#[derive(Parser, Debug)]
#[command(name = "kamon", version, about = "pleme-io design system renderer")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Render the token set to a single target.
    Render {
        #[arg(long)]
        target: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Render every supported target to a directory, one file per target.
    RenderAll {
        #[arg(long, default_value = "generated")]
        out_dir: PathBuf,
    },
    /// Print the content hash of the current token set.
    Hash,
    /// List every supported render target.
    Targets,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let tokens = TokenSet::pleme();

    match cli.cmd {
        Cmd::Render { target, out } => {
            let target = Target::from_str(&target)
                .ok_or_else(|| anyhow!("unknown target: {target}"))?;
            let content = target.render(&tokens);
            emit(content, out)?;
        }
        Cmd::RenderAll { out_dir } => {
            fs::create_dir_all(&out_dir).context("mkdir out_dir")?;
            for target in Target::all() {
                let path = out_dir.join(filename(target));
                let content = target.render(&tokens);
                fs::write(&path, content).with_context(|| format!("write {path:?}"))?;
                eprintln!("wrote {}", path.display());
            }
        }
        Cmd::Hash => {
            let h = tokens.content_hash();
            for b in h {
                print!("{b:02x}");
            }
            println!();
        }
        Cmd::Targets => {
            for t in Target::all() {
                println!("{}", target_name(t));
            }
        }
    }
    Ok(())
}

fn emit(content: String, out: Option<PathBuf>) -> Result<()> {
    match out {
        Some(path) => {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&path, content).with_context(|| format!("write {path:?}"))?;
            eprintln!("wrote {}", path.display());
        }
        None => {
            print!("{content}");
        }
    }
    Ok(())
}

fn filename(t: Target) -> &'static str {
    match t {
        Target::Css => "kamon.css",
        Target::Tailwind => "tailwind.config.js",
        Target::Scss => "_kamon.scss",
        Target::Rust => "kamon.rs",
        Target::Json => "kamon.tokens.json",
        Target::Glsl => "kamon.glsl",
        Target::Ghostty => "kamon.ghostty",
        Target::Tui => "kamon_tui.rs",
        Target::Svg => "mark.svg",
    }
}

fn target_name(t: Target) -> &'static str {
    match t {
        Target::Css => "css",
        Target::Tailwind => "tailwind",
        Target::Scss => "scss",
        Target::Rust => "rust",
        Target::Json => "json",
        Target::Glsl => "glsl",
        Target::Ghostty => "ghostty",
        Target::Tui => "tui",
        Target::Svg => "svg",
    }
}
