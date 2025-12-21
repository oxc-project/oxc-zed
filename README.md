<p align="center">
  <img alt="OXC Logo" src="https://cdn.jsdelivr.net/gh/oxc-project/oxc-assets/preview-universal.png" width="700">
</p>

# Oxc extension for Zed

This extension adds support for [Oxc](https://github.com/oxc-project/oxc) in [Zed](https://zed.dev/).

Languages currently supported:

- **JavaScript**
- **TypeScript**
- **JSX**
- **TSX**
- **Vue.js**
- **Astro**
- **Svelte**

## Installation

Requires Zed >= **v0.205.0**.

This extension is available in the extensions view inside the Zed editor. Open `zed: extensions` and search for _Oxc_.

## Configuration

Configuration is done within your settings.json file. Below are some example configurations.
See https://github.com/oxc-project/oxc/tree/main/crates/oxc_language_server for the options that are supported by the
language server.

### Oxfmt

```json
{
  "languages": {
    "JavaScript": {
      "format_on_save": "on",
      "formatter": [
        {
          "language_server": {
            "name": "oxfmt"
          }
        }
      ]
    }
  },
  "lsp": {
    "oxfmt": {
      "initialization_options": {
        "settings": {
          "configPath": null,
          "flags": {},
          "fmt.configPath": null,
          "fmt.experimental": true,
          "run": "onSave",
          "typeAware": false,
          "unusedDisableDirectives": false
        }
      }
    }
  }
}
```

### Oxlint

```json
{
  "languages": {
    "JavaScript": {
      "format_on_save": "on",
      "formatter": [
        {
          "code_action": "source.fixAll.oxc"
        }
      ]
    }
  },
  "lsp": {
    "oxlint": {
      "initialization_options": {
        "settings": {
          "disableNestedConfig": false,
          "fixKind": "safe_fix",
          "run": "onType",
          "typeAware": true,
          "unusedDisableDirectives": "deny"
        }
      }
    }
  }
}
```

### Oxfmt and Oxlint

```json
{
  "languages": {
    "JavaScript": {
      "format_on_save": "on",
      "formatter": [
        {
          "language_server": {
            "name": "oxfmt"
          }
        },
        {
          "code_action": "source.fixAll.oxc"
        }
      ]
    }
  },
  "lsp": {
    "oxlint": {
      "initialization_options": {
        "settings": {
          "disableNestedConfig": false,
          "fixKind": "safe_fix",
          "run": "onType",
          "typeAware": true,
          "unusedDisableDirectives": "deny"
        }
      }
    },
    "oxfmt": {
      "initialization_options": {
        "settings": {
          "configPath": null,
          "flags": {},
          "fmt.configPath": null,
          "fmt.experimental": true,
          "run": "onSave",
          "typeAware": false,
          "unusedDisableDirectives": false
        }
      }
    }
  }
}
```
