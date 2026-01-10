<p align="center">
  <img alt="OXC Logo" src="https://cdn.jsdelivr.net/gh/oxc-project/oxc-assets/preview-universal.png" width="700">
</p>

# Oxc extension for Zed

This extension adds support for [Oxc](https://github.com/oxc-project/oxc) in [Zed](https://zed.dev/).

The supported languages for Oxfmt and Oxlint can be seen within the [extension.toml](extension.toml) file.

## Installation

Requires Zed >= **v0.205.0**.

This extension is available in the extensions view inside the Zed editor. Open `zed: extensions` and search for _Oxc_.

## Configuration

Configuration is done within your settings.json file. Examples can be found [here](examples) for oxfmt and oxlint.
See https://github.com/oxc-project/oxc/tree/main/crates/oxc_language_server for the options that are supported by the
language server.
