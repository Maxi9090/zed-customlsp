use zed_extension_api::{self as zed};

struct LSPExtension {}

impl zed::Extension for LSPExtension {
    fn new() -> Self {
        Self {}
    }
    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command, String> {
        Ok(zed::Command {
            command: "".to_string(),
            args: Vec::new(),
            env: Vec::new(),
        })
    }
}

zed::register_extension!(LSPExtension);
