use crate::lsp::ZedLspSupport;
use log::debug;
use std::path::Path;
use zed_extension_api::serde_json::Value;
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{Command, EnvVars, LanguageServerId, Result, Worktree};

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

    fn get_vite_plus_subcommand(&self) -> &'static str {
        "lint"
    }

    fn language_server_command(
        &self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;
        debug!("Oxlint language_server_command LspSettings: {settings:?}");

        self.get_command(language_server_id, worktree, settings.binary)
    }

    fn normalize_env(&self, env: &mut EnvVars, worktree: &Worktree) {
        // Resolve a relative `OXLINT_TSGOLINT_PATH` against the worktree root.
        if let Some((_, tsgolint_path)) =
            env.iter_mut().find(|(name, _)| name == "OXLINT_TSGOLINT_PATH")
        {
            let root_path = worktree.root_path();
            *tsgolint_path = Path::new(root_path.as_str())
                .join(tsgolint_path.as_str())
                .to_string_lossy()
                .to_string();
        }
    }

    fn language_server_initialization_options(
        &self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;
        debug!("Oxlint language_server_initialization_options LspSettings: {settings:?}");

        Ok(settings.initialization_options)
    }

    fn language_server_workspace_configuration(
        &self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;
        debug!("Oxfmt language_server_workspace_configuration LspSettings: {settings:?}");

        Ok(settings.initialization_options.as_ref().and_then(|v| v.get("settings").cloned()))
    }
}
