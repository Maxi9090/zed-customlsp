# LSP Addon

Register any external LSP server in Zed without writing extension code.

## Requirements

- Read Zed extension documentation ([link](https://zed.dev/docs/extensions/developing-extensions)).
- **[Rust](https://rust-lang.github.io/rustup/installation/index.html) installed.** Zed compiles dev extensions from source.
- Your LSP server binaries are installed and available on your system.

## Install

1. In Zed, open `Extensions`, click `Install Dev Extensions`. Or run `zed: install dev extension` from the command palette.
2. Select this folder.
3. See that it is installed as `LSP Addon` in the extensions tab.

## Add LSP server

Edit `extension.toml` to add your LSP server under `[language_servers.*]`:

```toml
[language_servers.taplo]
name = "taplo"
language = "TOML"

[language_servers.tsgo]
name = "tsgo"
languages = ["TypeScript", "JavaScript", "TSX"]
```

To apply your changes: Open `Extensions` in Zed, find `LSP Addon` and click `Rebuild`. Or run `zed: rebuild dev extension` from the command palette.

Each entry requires:
- `name` — must match the key you use in your Zed `settings.json`.
- `language` / `languages` — one or more language scopes to associate.

Edit your Zed `settings.json` to configure the LSP server:

```json
{
  "lsp": {
    "taplo": {
      "binary": {
        "path": "~/.local/bin/taplo",
        "arguments": ["lsp", "stdio"]
      }
    }
  }
}
```

Enable it for the language:

```json
{
  "languages": {
    "TOML": {
      "enable_language_server": true,
      "language_servers": ["taplo"],
      "formatter": "language_server"
    }
  }
}
```

Restart Zed and open a file in that language.
