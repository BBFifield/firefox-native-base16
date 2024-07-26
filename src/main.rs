use clap::Parser;

/// Simple native program to watch and send base16 color schemes to browsers
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// TOML colors file to watch
  #[arg(short, long, default_value = "~/.mozilla/colors.toml")]
  colors_path: String,
}

fn main() {
  let args = Args::parse();

  println!("Watching file {}", args.colors_path);
}
