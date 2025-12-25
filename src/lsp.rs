use log::debug;
use std::env;
use std::path::{Path, PathBuf};
use zed_extension_api::serde_json::{Value, from_str};
use zed_extension_api::{
    Command, LanguageServerId, LanguageServerInstallationStatus, Result, Worktree,
    npm_install_package, npm_package_installed_version, npm_package_latest_version,
    set_language_server_installation_status,
};

pub const OXLINT_SERVER_ID: &str = "oxlint";
pub const OXFMT_SERVER_ID: &str = "oxfmt";

pub trait ZedLspSupport: Send + Sync {
    fn exe_exists(&self, worktree: &Worktree) -> Result<bool> {
        // Reading files from node_modules doesn't seem to be possible now,
        // https://github.com/zed-industries/zed/issues/10760.
        // Instead we try to read the `package.json`, see if the package is installed
        let package_json = worktree
            .read_text_file("package.json")
            .unwrap_or(String::from(r#"{}"#));

        let package_json: Option<Value> = from_str(package_json.as_str()).ok();

        let package_name = self.get_package_name();
        Ok(package_json.is_some_and(|f| {
            !f["dependencies"][&package_name].is_null()
                || !f["devDependencies"][&package_name].is_null()
        }))
    }

    fn get_extension_exe_path(&self) -> Result<PathBuf> {
        let extension_work_dir = env::current_dir().map_err(|err| err.to_string())?;

        Ok(Path::new(&extension_work_dir)
            .join("node_modules")
            .join(".bin")
            .join(self.get_package_name()))
    }

    fn get_workspace_exe_path(&self, worktree: &Worktree) -> Result<PathBuf> {
        let package_name = self.get_package_name();
        Ok(Path::new(worktree.root_path().as_str())
            .join("node_modules")
            .join(&package_name)
            .join("bin")
            .join(&package_name))
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

    fn language_server_workspace_configuration(
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
