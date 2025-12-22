use log::debug;
use std::env;
use std::path::{Path, PathBuf};
use zed_extension_api::serde_json::Value;
use zed_extension_api::{
    npm_install_package, npm_package_installed_version, npm_package_latest_version, set_language_server_installation_status, Command,
    LanguageServerId, LanguageServerInstallationStatus, Result,
    Worktree,
};

pub const OXLINT_SERVER_ID: &str = "oxlint";
pub const OXFMT_SERVER_ID: &str = "oxfmt";

pub trait ZedLspSupport: Send + Sync {
    fn exe_exists(&self, worktree: &Worktree) -> Result<bool> {
        let exe_path = self.get_workspace_exe_path(worktree)?;

        Ok(exe_path.is_file())
    }

    fn get_extension_exe_path(&self) -> Result<PathBuf> {
        let extension_work_dir = env::current_dir().map_err(|err| err.to_string())?;

        Ok(Path::new(&extension_work_dir)
            .join("node_modules")
            .join(".bin")
            .join(self.get_package_name()))
    }

    fn get_workspace_exe_path(&self, worktree: &Worktree) -> Result<PathBuf> {
        Ok(Path::new(worktree.root_path().as_str())
            .join("node_modules")
            .join(".bin")
            .join(self.get_package_name()))
    }

    fn get_resolved_exe_path(&self, worktree: &Worktree) -> Result<PathBuf> {
        if self.exe_exists(worktree)? {
            debug!(
                "Found exe installation in worktree {}",
                worktree.root_path()
            );
            return self.get_workspace_exe_path(worktree);
        }

        debug!("Using exe installation from extension");
        self.get_extension_exe_path()
    }

    fn get_package_name(&self) -> String;

    fn language_server_command(
        &self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command>;

    fn language_server_initialization_options(
        &self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>>;

    fn update_extension_language_server_if_outdated(
        &self,
        language_server_id: &LanguageServerId,
    ) -> Result<()> {
        set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let package_name = self.get_package_name();
        let current_version = npm_package_installed_version(package_name.as_str())?;
        let latest_version = npm_package_latest_version(package_name.as_str())?;
        debug!(
            "Package {package_name:?} versions - Current: {current_version:?}, Latest: {latest_version:?}",
        );
        if current_version.is_some_and(|version| version == latest_version) {
            // Do nothing.
        } else {
            set_language_server_installation_status(
                language_server_id,
                &LanguageServerInstallationStatus::Downloading,
            );
            npm_install_package(package_name.as_str(), &latest_version)?;
            set_language_server_installation_status(
                language_server_id,
                &LanguageServerInstallationStatus::None,
            );
        }

        Ok(())
    }
}
