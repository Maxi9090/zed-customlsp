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
- `name` — the binary name (used as the LSP server ID)
- `language` / `languages` — one or more language scopes to associate

2. Reimport the extension in Zed (`zed: extensions` → find LSP Addon → reinstall or reload).
3. Ensure the LSP server binary is installed and available on your system.
4. Configure it in your Zed settings (`~/.config/zed/settings.json`) as needed:

```json
{
  "lsp": {
    "tsgo": {
      "binary": {
        "path": "~/.local/bin/tsgo",
        "arguments": ["--lsp", "--stdio"]
      }
    }
  }
}
```

The LSP server ID in settings must match the `name` field in `extension.toml`. After these steps, the language selector in Zed will recognize the new LSP server for the associated languages.
You can put it to ~/.local/share/zed/extensions/installed/lspaddon/ to install it or the button in zed install dev extension to install it 
and edit  ~/.local/share/zed/extensions/installed/lspaddon/extension.toml to extend allowed lsp servers
