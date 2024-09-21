{
  mkShell,
  callPackage,
  rustc,
  rust-analyzer,
  rustfmt,
  clippy,
  ...
}:
mkShell {
  inputsFrom = [ (callPackage ./build.nix { }) ];
  buildInputs = [
    rustc
    rust-analyzer
    rustfmt
    clippy
  ];
}
