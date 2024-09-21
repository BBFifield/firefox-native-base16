# Build using rustplatform
{
  lib,
  rustPlatform,
}:

rustPlatform.buildRustPackage {
  pname = "firefox-native-base16";
  version = "0.1.0";

  src = ../.;

  cargoLock.lockFile = ../Cargo.lock;

  meta = {
    description = "A simple native application to dynamically change your browser theme using base16.";
    homepage = "https://github.com/GnRlLeclerc/firefox-native-base16";
    license = lib.licenses.mit;
    maintainers = [ ];
  };
}
