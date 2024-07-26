use std::path::Path;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use notify::event::{AccessKind, AccessMode};
use notify::{recommended_watcher, EventKind, Watcher};
use regex::Regex;
use serde_derive::Deserialize;

/// Simple native program to watch and send base16 color schemes to browsers
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// TOML colors file to watch
  #[arg(short, long, default_value = "~/.mozilla/colors.toml")]
  colors_path: String,
}

/// Base16 color scheme
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Colors {
  base00: String,
  base01: String,
  base02: String,
  base03: String,
  base04: String,
  base05: String,
  base06: String,
  base07: String,
  base08: String,
  base09: String,
  base0A: String,
  base0B: String,
  base0C: String,
  base0D: String,
  base0E: String,
  base0F: String,
}

fn main() -> Result<()> {
  let args = Args::parse();

  // Configure the file watcher
  let (tx, rx) = std::sync::mpsc::channel();
  let mut watcher = recommended_watcher(tx)?;
  watcher.watch(Path::new(&args.colors_path), notify::RecursiveMode::NonRecursive)?;

  // Read from the watcher
  for res in rx {
    match res {
      Ok(event) => match event.kind {
        EventKind::Access(AccessKind::Close(AccessMode::Write)) => {
          // Debug
          println!("{:?}", read_colors(&args.colors_path))
        }
        _ => continue,
      },
      Err(e) => eprintln!("watch error: {:?}", e),
    }
  }
  Ok(())
}

/// Read colors from a TOML file and validate them.
fn read_colors(path: &str) -> Result<Colors> {
  let content = std::fs::read_to_string(path).context(format!("Failed to read colors TOML file: {}", &path))?;
  let colors = toml::from_str(&content).context("Failed to parse colors TOML file")?;
  validate_hex_colors(&colors)?;

  Ok(colors)
}

/// Check that all colors are valid hex colors
fn validate_hex_colors(colors: &Colors) -> Result<()> {
  let re = Regex::new(r"^#(?:[0-9a-fA-F]{3}){1,2}$|^#(?:[0-9a-fA-F]{4}){1,2}$").unwrap();

  let hex_colors = [
    ("base00", &colors.base00),
    ("base01", &colors.base01),
    ("base02", &colors.base02),
    ("base03", &colors.base03),
    ("base04", &colors.base04),
    ("base05", &colors.base05),
    ("base06", &colors.base06),
    ("base07", &colors.base07),
    ("base08", &colors.base08),
    ("base09", &colors.base09),
    ("base0A", &colors.base0A),
    ("base0B", &colors.base0B),
    ("base0C", &colors.base0C),
    ("base0D", &colors.base0D),
    ("base0E", &colors.base0E),
    ("base0F", &colors.base0F),
  ];

  for (label, hex) in hex_colors {
    if !re.is_match(hex) {
      return Err(anyhow!("Invalid hex color for {}: {}", label, hex));
    }
  }

  Ok(())
}
