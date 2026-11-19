<p align="center">
  <br>
  <br>
  <a href="https://oxc.rs" target="_blank" rel="noopener noreferrer">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://oxc.rs/oxc-light.svg">
      <source media="(prefers-color-scheme: light)" srcset="https://oxc.rs/oxc-dark.svg">
      <img alt="Oxc logo" src="https://oxc.rs/oxc-dark.svg" height="60">
    </picture>
  </a>
  <br>
  <br>
  <br>
</p>

# Oxc extension for Zed

This extension adds support for [Oxc](https://github.com/oxc-project/oxc) in [Zed](https://zed.dev/).

The supported languages for Oxfmt and Oxlint can be seen within the [extension.toml](extension.toml) file.

## Installation

Requires Zed >= **v0.205.0** and oxlint >= **v1.35.0**.

This extension is available in the extensions view inside the Zed editor. Open `zed: extensions` and search for _Oxc_.

## Configuration

Configuration is managed in your `.zed/settings.json` file. Examples are available in the [examples](./examples) directory.

See https://github.com/oxc-project/oxc/tree/main/crates/oxc_language_server for the options that are supported by the language server.

### Formatter selection

Zed formats a buffer with the formatter the language's `formatter` setting selects. Its default, `auto`, uses Prettier when the language allows it, and otherwise the first language server that advertises formatting support. For JavaScript, TypeScript, TSX and Vue.js, Zed allows Prettier and starts the built-in `vtsls`, which formats as well, so under `auto` Oxfmt formats none of those files and `.oxfmtrc.json` has no effect. Naming oxfmt for a language is what makes it format:

```json
{
  "languages": {
    "TypeScript": {
      "format_on_save": "on",
      "prettier": {
        "allowed": false
      },
      "formatter": [
        {
          "language_server": {
            "name": "oxfmt"
          }
        }
      ]
    }
  }
}
```

The settings in the [examples](./examples) directory apply the same block to each of the languages they cover.

# [Sponsored By](https://oxc.rs/sponsor)

<p align="center">
  <a href="https://oxc.rs/sponsor">
    <img src="https://raw.githubusercontent.com/oxc-project/sponsors/main/sponsors.svg" alt="Our sponsors" />
  </a>
</p>
