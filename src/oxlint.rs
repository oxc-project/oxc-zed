use crate::lsp::ZedLspSupport;
use log::debug;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use zed_extension_api::serde_json::Value;
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{Command, EnvVars, LanguageServerId, Result, Worktree, node_binary_path};

pub struct ZedOxlintLsp {}

impl ZedOxlintLsp {
    pub fn new() -> Self {
        ZedOxlintLsp {}
    }

    pub fn language_server_workspace_configuration(
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
            let env = normalize_tsgolint_env(binary.env, worktree)?;
            return Ok(Command {
                command: binary
                    .path
                    .expect("When supplying binary settings, the path must be supplied"),
                args: binary
                    .arguments
                    .expect("When supplying binary settings, the args must be supplied"),
                env,
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

fn normalize_tsgolint_env(
    env_map: Option<HashMap<String, String>>,
    worktree: &Worktree,
) -> Result<EnvVars> {
    let mut env: EnvVars = env_map.unwrap_or_default().into_iter().collect();

    if let Some((_, path)) = env
        .iter_mut()
        .find(|(key, _)| key == "OXLINT_TSGOLINT_PATH")
    {
        let root_path = worktree.root_path();
        let worktree_root = Path::new(root_path.as_str());
        let path_buf = PathBuf::from(path.as_str());

        let resolved = if path_buf.is_absolute() {
            path_buf
        } else {
            worktree_root.join(&path_buf)
        };

        *path = resolved.to_string_lossy().to_string();
    }

    Ok(env)
}
