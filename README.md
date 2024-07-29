# Firefox Native Base16

A simple native application that can watch any base16 TOML color file for changes and communicate them to your browser.

## Installation

### Building the binary

You must first build and install the native application, which requires `cargo` or `rustup` to be installed:

```bash
cargo install --git https://github.com/GnRlLeclerc/firefox-native-base16
```

On Linux systems, the binary will be installed to `~/.cargo/bin/firefox-native-base16`.

### Installing the native app

In order for the browser to indentify the native application, it needs a `manifest.json` file that points to the binary.

NOTE: instructions for Linux / MacOS only. See [here](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Native_manifests#manifest_location) for more information on where to put the app manifest.

#### Default installation

By default, the binary application looks for a `colors.toml` file in the mozilla `~/.mozilla` folder.
If this is fine for you, you can specify the **full** path to your binary in the `manifest.json` file.

```bash
mkdir ~/.mozilla/native-messaging-hosts
jq ".path = \"$HOME/.cargo/bin/firefox-native-base16\"" manifest.json > ~/.mozilla/native-messaging-hosts/firefox_native_base16.json
```

#### Custom installation

You might want to watch a different file instead. The binary application can be run with the `--colors-path` flag to specify a different path.
However, browsers can only run scripts, not commands, so you will have to create an auxiliary script like so:

`~/.local/bin/custom-firefox-native-base16`

```bash
#!/bin/bash

firefox-native-base16 --colors-path /custom/path/colors.toml
```

Move this script in a directory present in your `PATH` (like `~/.local/bin`), and indicate this path instead in your native application manifest.

```bash
mkdir ~/.mozilla/native-messaging-hosts
jq ".path = \"$HOME/.local/bin/custom-firefox-native-base16\"" manifest.json > ~/.mozilla/native-messaging-hosts/firefox_native_base16.json
```

## Related Extensions

Once your native application is setup, install the relevant browser extension.

### Firefox

See the [Firefox Base16 Extension](https://github.com/GnRlLeclerc/firefox-dynamic-base16).

### Chrome

Coming soon.

You can now update your `colors.toml` based on the [mustache template](./template.mustache) and watch your browser theme change dynamically!
