use crate::lsp::ZedLspSupport;
use log::debug;
use std::collections::HashMap;
use zed_extension_api::serde_json::Value;
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{Command, EnvVars, LanguageServerId, Result, Worktree, node_binary_path};

pub struct ZedOxlintLsp {}

impl ZedOxlintLsp {
    pub fn new() -> Self {
        ZedOxlintLsp {}
    }
}

impl ZedLspSupport for ZedOxlintLsp {
    fn get_package_name(&self) -> String {
        "oxlint".to_string()
    }

    fn language_server_command(
        &self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;
        debug!("Oxlint Settings: {settings:?}");

        if let Some(binary) = settings.binary {
            return Ok(Command {
                command: binary
                    .path
                    .expect("When supplying binary settings, the path must be supplied"),
                args: binary
                    .arguments
                    .expect("When supplying binary settings, the args must be supplied"),
                env: binary
                    .env
                    .unwrap_or(HashMap::default())
                    .into_iter()
                    .collect(),
            });
        }

        Ok(Command {
            command: node_binary_path()?,
            args: vec![
                self.get_resolved_exe_path(worktree)?
                    .to_string_lossy()
                    .to_string(),
                "--lsp".to_string(),
            ],
            env: EnvVars::default(),
        })
    }

    fn language_server_initialization_options(
        &self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;
        debug!("Oxlint Settings: {settings:?}");

        Ok(settings.initialization_options)
    }
}
