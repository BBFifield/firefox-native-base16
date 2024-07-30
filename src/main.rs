use std::io::Write;
use std::path::Path;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use notify::event::{AccessKind, AccessMode};
use notify::{recommended_watcher, EventKind, Watcher};
use regex::Regex;
use serde_derive::{Deserialize, Serialize};

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
#[derive(Serialize, Deserialize, Debug)]
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
  let colors_path = shellexpand::full(&args.colors_path).unwrap().to_string();

  // Configure the file watcher
  let (tx, rx) = std::sync::mpsc::channel();
  let mut watcher = recommended_watcher(tx)?;
  watcher.watch(Path::new(&colors_path), notify::RecursiveMode::NonRecursive)?;

  // Send the colors immediately
  send_colors(&colors_path);

  // Read from the watcher
  for res in rx {
    match res {
      Ok(event) => match event.kind {
        EventKind::Access(AccessKind::Close(AccessMode::Write)) => send_colors(&colors_path),
        _ => continue,
      },
      Err(e) => eprintln!("Watch error: {:?}", e),
    }
  }
  Ok(())
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

/// Read colors from a file and send them to stdout.
/// Returns a Result.
fn try_send_colors(path: &str) -> Result<()> {
  // Read colors and validate them
  let content = std::fs::read_to_string(path).context(format!("Failed to read colors TOML file: {}", &path))?;
  let colors = toml::from_str(&content).context("Failed to parse colors TOML file")?;
  validate_hex_colors(&colors)?;

  // Write the colors to stdout
  let stdout = std::io::stdout();
  let mut writer = stdout.lock();

  let message = serde_json::to_string(&colors)?;
  let length = message.len() as u32;
  let length_bytes = length.to_ne_bytes();

  writer.write_all(&length_bytes)?;
  writer.write_all(message.as_bytes())?;
  writer.flush()?;

  Ok(())
}

/// Read colors from a file and send them to stdout.
/// Prints an error message to stderr if an error occurs.
fn send_colors(path: &str) {
  match try_send_colors(path) {
    Ok(_) => (),
    Err(e) => eprintln!("Error: {}", e),
  }
}
