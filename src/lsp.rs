use crate::vite_plus::{DetectResult, FileSystem, detect_vite_plus_project};
use log::debug;
use std::env;
use std::path::{Path, PathBuf};
use zed_extension_api::serde_json::{Value, from_str};
use zed_extension_api::{
    Command, LanguageServerId, LanguageServerInstallationStatus, Result, Worktree,
    node_binary_path, npm_install_package, npm_package_installed_version,
    npm_package_latest_version, set_language_server_installation_status,
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

        // Vite+ detection is handled separately (see `detect_vite_plus`); this
        // only locates a plain oxlint/oxfmt install declared at the worktree root.
        if package_json
            .as_ref()
            .is_some_and(|package_json| package_exists(package_json, package_name.as_str()))
        {
            return self
                .get_exe_path_from(workspace_root, package_name.as_str(), package_name.as_str())
                .map(Some);
        }

        Ok(None)
    }

    /// Run the Vite+ detector against this worktree. Returns `None` when the
    /// project is not Vite+; see [`DetectResult`] for the two `Some` cases.
    fn detect_vite_plus(&self, worktree: &Worktree) -> Option<DetectResult> {
        let root = worktree.root_path();
        let fs = WorktreeFs::new(worktree);
        detect_vite_plus_project(&fs, Path::new(root.as_str()))
    }

    /// The `vp` subcommand this language server maps to (`lint` / `fmt`).
    fn vite_plus_subcommand(&self) -> &'static str;

    /// Resolve the `(command, args)` to launch the language server, choosing
    /// `vp <subcommand> --lsp` for a runnable Vite+ project and plain
    /// `<tool> --lsp` otherwise.
    fn resolve_lsp_invocation(&self, worktree: &Worktree) -> Result<(String, Vec<String>)> {
        if let Some(result) = self.detect_vite_plus(worktree) {
            if let Some(vp_path) = result.vp_path {
                debug!(
                    "Vite+ project detected at {:?}; launching {vp_path:?}",
                    result.root
                );
                return Ok((
                    node_binary_path()?,
                    vec![
                        vp_path.to_string_lossy().to_string(),
                        self.vite_plus_subcommand().to_string(),
                        "--lsp".to_string(),
                    ],
                ));
            }
            // Declared but not installed: no runnable `vp` reachable. Fall back
            // to plain oxlint/oxfmt rather than spawning a bare `vp`.
            debug!(
                "Vite+ declared at {:?} but no runnable vp found; falling back to plain {}",
                result.root,
                self.get_package_name(),
            );
        }

        Ok((
            node_binary_path()?,
            vec![
                self.get_resolved_exe_path(worktree)?
                    .to_string_lossy()
                    .to_string(),
                "--lsp".to_string(),
            ],
        ))
    }

    fn exe_exists(&self, worktree: &Worktree) -> Result<bool> {
        // A runnable Vite+ project supplies its own `vp`; the extension does
        // not need to download oxlint/oxfmt from npm in that case.
        if let Some(DetectResult {
            vp_path: Some(_), ..
        }) = self.detect_vite_plus(worktree)
        {
            return Ok(true);
        }
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

/// [`FileSystem`] backed by a Zed [`Worktree`].
///
/// Zed's WASM API can only read files inside the worktree, relative to its
/// root, so absolute paths are translated to worktree-relative ones and any
/// path above the root resolves to "missing". This naturally bounds the
/// detector's upward walk at the worktree root — the single-root limitation
/// noted in the RFC.
pub struct WorktreeFs<'a> {
    worktree: &'a Worktree,
    root: PathBuf,
}

impl<'a> WorktreeFs<'a> {
    pub fn new(worktree: &'a Worktree) -> Self {
        let root = PathBuf::from(worktree.root_path());
        Self { worktree, root }
    }
}

impl FileSystem for WorktreeFs<'_> {
    fn read_text_file(&self, path: &Path) -> Option<String> {
        let rel = path.strip_prefix(&self.root).ok()?;
        self.worktree
            .read_text_file(rel.to_string_lossy().as_ref())
            .ok()
    }

    fn file_exists(&self, path: &Path) -> bool {
        // The API exposes no stat; a successful text read is our existence
        // probe. `bin/vp` and the marker files are small text launchers.
        self.read_text_file(path).is_some()
    }

    fn which(&self, binary: &str) -> Option<PathBuf> {
        // Zed *does* expose a `$PATH` lookup, contrary to the RFC's note that
        // Phase 3 is unimplementable here.
        self.worktree.which(binary).map(PathBuf::from)
    }
}
