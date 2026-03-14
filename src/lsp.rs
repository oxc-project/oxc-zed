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
    fn get_workspace_exe_path(&self, worktree: &Worktree) -> Result<Option<PathBuf>> {
        // Reading files from node_modules doesn't seem to be possible now,
        // https://github.com/zed-industries/zed/issues/10760.
        // Instead we try to read the `package.json`, see if the package is installed
        let package_json = worktree
            .read_text_file("package.json")
            .unwrap_or(String::from(r#"{}"#));

        let package_json: Option<Value> = from_str(package_json.as_str()).ok();
        let package_name = self.get_package_name();
        let workspace_root_path = worktree.root_path();
        let workspace_root = Path::new(workspace_root_path.as_str());

        for package_dir in [package_name.as_str(), "vite-plus"] {
            if package_json
                .as_ref()
                .is_some_and(|package_json| package_exists(package_json, package_dir))
            {
                return self
                    .get_exe_path_from(workspace_root, package_dir, package_name.as_str())
                    .map(Some);
            }
        }

        Ok(None)
    }

    fn exe_exists(&self, worktree: &Worktree) -> Result<bool> {
        Ok(self.get_workspace_exe_path(worktree)?.is_some())
    }

    fn get_exe_path_from(&self, from: &Path, package_dir: &str, exe_name: &str) -> Result<PathBuf> {
        // Doesn't use `node_modules/.bin` due to PNPM storing bash scripts there
        // instead of Node.js scripts.
        Ok(from
            .join("node_modules")
            .join(package_dir)
            .join("bin")
            .join(exe_name))
    }

    fn get_resolved_exe_path(&self, worktree: &Worktree) -> Result<PathBuf> {
        if let Some(path) = self.get_workspace_exe_path(worktree)? {
            debug!("Found exe installation in worktree at path {path:?}");
            return Ok(path);
        }

        let package_name = self.get_package_name();
        let path = self.get_exe_path_from(
            env::current_dir().map_err(|err| err.to_string())?.as_path(),
            package_name.as_str(),
            package_name.as_str(),
        );
        debug!("Using exe installation from extension at path {path:?}");
        path
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

fn package_exists(package_json: &Value, package_name: &str) -> bool {
    !package_json["dependencies"][package_name].is_null()
        || !package_json["devDependencies"][package_name].is_null()
}
