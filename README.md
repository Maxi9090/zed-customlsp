# LSP Addon

A Zed extension that lets you register arbitrary LSP servers without writing custom extension code.

## Usage

1. Edit `extension.toml` to add your LSP server under `[language_servers.*]`:

```toml
[language_servers.taplo]
name = "taplo"
language = "TOML"

[language_servers.tsgo]
name = "tsgo"
languages = ["TypeScript", "JavaScript", "TSX"]
```

Each entry requires:
- `name` — the binary name (must be on `$PATH`)
- `language` / `languages` — one or more language scopes to associate

2. Reimport the extension in Zed (`zed: extensions` → find LSP Addon → reinstall or reload).

The LSP server binary must be installed and available on your system `$PATH` for this to work.
