use clap::Parser;
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

fn main() {
  let args = Args::parse();

  println!("Watching file {}", args.colors_path);

  println!("{:?}", read_colors(&args.colors_path));
}

/// Read colors from a TOML file.
/// Panics on error.
fn read_colors(path: &str) -> Colors {
  let content =
    std::fs::read_to_string(path).expect(format!("Could not read file {}", path).as_str());
  let colors: Colors = toml::from_str(&content).expect("Could not parse colors file");
  colors
}
