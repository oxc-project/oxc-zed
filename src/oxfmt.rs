use crate::lsp::ZedLspSupport;
use log::debug;
use zed_extension_api::serde_json::Value;
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{Command, EnvVars, LanguageServerId, Result, Worktree, node_binary_path};

pub struct ZedOxfmtLsp {}

impl ZedOxfmtLsp {
    pub fn new() -> Self {
        ZedOxfmtLsp {}
    }
}

impl ZedLspSupport for ZedOxfmtLsp {
    fn get_package_name(&self) -> String {
        "oxfmt".to_string()
    }

    fn language_server_command(
        &self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;
        debug!("Oxfmt Settings: {settings:?}");

        let args: Vec<String>;
        let command: String;
        let env: EnvVars;
        if let Some(binary) = settings.binary {
            if (binary.path.is_some() && binary.arguments.is_none())
                || (binary.path.is_none() && binary.arguments.is_some())
            {
                return Err(
                    "When supplying binary.arguments, binary.path must be supplied (or vice-versa).".to_string(),
                );
            }

            args = binary.arguments.unwrap();
            command = binary.path.unwrap();
            env = binary.env.unwrap_or_default().into_iter().collect();
        } else {
            args = vec![
                self.get_resolved_exe_path(worktree)?
                    .to_string_lossy()
                    .to_string(),
                "--lsp".to_string(),
            ];
            command = node_binary_path()?;
            env = EnvVars::default();
        }

        Ok(Command { command, args, env })
    }

    fn language_server_initialization_options(
        &self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;
        debug!("Oxfmt Settings: {settings:?}");

        Ok(settings.initialization_options)
    }

    fn language_server_workspace_configuration(
        &self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        Ok(
            LspSettings::for_worktree(language_server_id.as_ref(), worktree)?
                .initialization_options
                .as_ref()
                .and_then(|v| v.get("settings").cloned()),
        )
    }
}
