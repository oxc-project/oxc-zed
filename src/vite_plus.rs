//! Vite+ project detection.
//!
//! Portable implementation of the rule defined in the Vite+ RFC
//! *"Vite+ Project Detection for Editor Extensions"*
//! ([voidzero-dev/vite-plus#1614]). A workspace is **Vite+** iff some
//! walked-up `package.json` directly declares `vite-plus` in `dependencies`
//! or `devDependencies`, bounded by the root workspace. The runnable `vp`
//! binary is resolved separately, also bounded by the root workspace, and
//! validated against a real `vite-plus` package (`name === "vite-plus"`).
//!
//! The core logic is intentionally free of any `zed_extension_api` resource
//! type so it can be unit-tested against the RFC conformance fixtures on the
//! host. At runtime the Zed extension backs the [`FileSystem`] trait with a
//! `Worktree` (see [`crate::lsp::WorktreeFs`]).
//!
//! [voidzero-dev/vite-plus#1614]: https://github.com/voidzero-dev/vite-plus/pull/1614

use std::path::{Path, PathBuf};
use zed_extension_api::serde_json::{Value, from_str};

/// Filesystem operations the detector needs.
///
/// Paths handed to these methods are absolute. The Zed adapter translates
/// them to worktree-relative reads and returns `None`/`false` for anything
/// outside the worktree (Zed's WASM API cannot read above the worktree root).
pub trait FileSystem {
    /// Returns the textual contents of the file at `path`, or `None` when it
    /// does not exist or cannot be read.
    fn read_text_file(&self, path: &Path) -> Option<String>;

    /// Whether a regular file exists at `path`.
    fn file_exists(&self, path: &Path) -> bool;

    /// Resolves a binary by name on `$PATH` (RFC Phase 3). `None` when the
    /// binary is absent or `$PATH` lookup is unavailable in this environment.
    fn which(&self, binary: &str) -> Option<PathBuf>;

    /// Whether reads inside `node_modules` are reliable.
    ///
    /// Zed's WASM API cannot reliably read there ([zed#10760]) and `Path::exists`
    /// does not work in the sandbox, so when this returns `false` the detector
    /// trusts the conventional `node_modules/vite-plus/bin/vp` path instead of
    /// read-verifying the install — exactly what the shipping extension already
    /// does for `oxlint`/`oxfmt`. A missing install then surfaces as a
    /// spawn-time error (the RFC's anticipated "upgrade hint" trigger).
    ///
    /// [zed#10760]: https://github.com/zed-industries/zed/issues/10760
    fn can_read_node_modules(&self) -> bool {
        true
    }
}

/// Outcome of [`detect_vite_plus_project`].
///
/// Mirrors the RFC's `{ root: string; vpPath?: string } | null`:
/// - `None` (from the detector) — not a Vite+ project.
/// - `Some` with `vp_path = Some(_)` — Vite+ and runnable.
/// - `Some` with `vp_path = None` — declared but no usable `vp` (install hint).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectResult {
    /// The ancestor whose `package.json` directly declares `vite-plus`.
    pub root: PathBuf,
    /// The runnable `vp` launcher, when one was found.
    pub vp_path: Option<PathBuf>,
}

fn read_package_json<F: FileSystem>(fs: &F, dir: &Path) -> Option<Value> {
    let contents = fs.read_text_file(&dir.join("package.json"))?;
    from_str(&contents).ok()
}

fn declares_vite_plus(pkg: &Value) -> bool {
    !pkg["dependencies"]["vite-plus"].is_null() || !pkg["devDependencies"]["vite-plus"].is_null()
}

/// A directory is a *root workspace* (monorepo root) if it carries a
/// `pnpm-workspace.yaml`, a `lerna.json`, or a `package.json` with a
/// top-level `workspaces` field. Mirrors `findWorkspaceRoot` in vite-plus.
fn is_root_workspace<F: FileSystem>(fs: &F, dir: &Path, pkg: Option<&Value>) -> bool {
    if fs.file_exists(&dir.join("pnpm-workspace.yaml")) {
        return true;
    }
    if fs.file_exists(&dir.join("lerna.json")) {
        return true;
    }
    pkg.is_some_and(|pkg| !pkg["workspaces"].is_null())
}

/// The conventional `vp` launcher path for a project rooted at `dir`.
///
/// Zed targets the real package launcher rather than the package manager's
/// `node_modules/.bin/vp` shell shim, because pnpm stores bash scripts there
/// that are not usable from Zed's headless WASM execution context.
fn vp_launcher_path(dir: &Path) -> PathBuf {
    dir.join("node_modules")
        .join("vite-plus")
        .join("bin")
        .join("vp")
}

/// Probe [`vp_launcher_path`], validated against that package's own
/// `package.json` (`name === "vite-plus"`). Used only where `node_modules`
/// reads are reliable; see [`FileSystem::can_read_node_modules`].
fn resolve_vp_at<F: FileSystem>(fs: &F, dir: &Path) -> Option<PathBuf> {
    let bin = vp_launcher_path(dir);
    if !fs.file_exists(&bin) {
        return None;
    }
    let pkg_dir = dir.join("node_modules").join("vite-plus");
    let manifest = read_package_json(fs, &pkg_dir)?;
    (manifest["name"].as_str() == Some("vite-plus")).then_some(bin)
}

/// Walks `dir` up to the filesystem root, returning each ancestor and a flag
/// telling whether that ancestor is itself the root workspace (the bound: we
/// visit the root workspace, then stop). The first read `package.json` is
/// surfaced so the caller can avoid re-reading it.
fn walk_up<F, T>(
    fs: &F,
    start: &Path,
    mut visit: impl FnMut(&Path, Option<&Value>) -> Step<T>,
) -> Option<T>
where
    F: FileSystem,
{
    let mut dir = start.to_path_buf();
    loop {
        let pkg = read_package_json(fs, &dir);
        match visit(&dir, pkg.as_ref()) {
            Step::Found(value) => return Some(value),
            Step::Continue => {}
        }
        // Stop *at* the root workspace; never cross into its parent.
        if is_root_workspace(fs, &dir, pkg.as_ref()) {
            return None;
        }
        match dir.parent() {
            Some(parent) if parent != dir => dir = parent.to_path_buf(),
            _ => return None,
        }
    }
}

enum Step<T> {
    Found(T),
    Continue,
}

/// Detects whether `start` lives in a Vite+ project and, if so, locates a
/// runnable `vp`. Implements the three RFC phases:
///
/// 1. Walk up from `start` to the root workspace looking for the first
///    `package.json` that directly declares `vite-plus`. If none, return
///    `None` (not Vite+).
/// 2. Walk up from that declaring directory (again bounded by the root
///    workspace) for a project-scoped `node_modules/vite-plus/bin/vp`. When
///    `node_modules` reads are unreliable (Zed; see
///    [`FileSystem::can_read_node_modules`]) the conventional path at the
///    declaring directory is trusted instead of probed.
/// 3. Fall back to a `vp` on `$PATH` now that Vite+ is confirmed.
///
/// Returns `Some(DetectResult { root, vp_path })`; `vp_path` is `None` when
/// `vite-plus` is declared but no runnable `vp` exists anywhere reachable.
pub fn detect_vite_plus_project<F: FileSystem>(fs: &F, start: &Path) -> Option<DetectResult> {
    // Phase 1: find the package.json that DIRECTLY declares vite-plus.
    let root = walk_up(fs, start, |dir, pkg| {
        if pkg.is_some_and(declares_vite_plus) {
            Step::Found(dir.to_path_buf())
        } else {
            Step::Continue
        }
    })?;

    // Phase 2: resolve a project-scoped binary, bounded by the root workspace.
    let vp_path = if fs.can_read_node_modules() {
        // Reliable FS (Node extensions, tests): walk up read-verifying each
        // candidate and validating it is a real `vite-plus` package.
        walk_up(fs, &root, |dir, _| match resolve_vp_at(fs, dir) {
            Some(vp) => Step::Found(vp),
            None => Step::Continue,
        })
    } else {
        // Zed: node_modules reads are unreliable, so trust the conventional
        // path at the declaring directory. We cannot probe ancestors for a
        // hoisted install (no reliable existence check), so we target the
        // declaring directory only — the same `<root>/node_modules/...` path
        // the extension uses today.
        Some(vp_launcher_path(&root))
    };
    if let Some(vp_path) = vp_path {
        return Some(DetectResult {
            root,
            vp_path: Some(vp_path),
        });
    }

    // Phase 3: fall back to a global vp now that Vite+ is confirmed.
    // Unlike the RFC's note, Zed *can* do this: `Worktree::which` exposes a
    // `$PATH` lookup. See the module-level docs.
    if let Some(vp) = fs.which("vp") {
        return Some(DetectResult {
            root,
            vp_path: Some(vp),
        });
    }

    Some(DetectResult {
        root,
        vp_path: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    /// In-memory filesystem for the conformance fixtures. A path is present
    /// in `files` iff it exists; readable files also carry contents. `which`
    /// models a `vp` on `$PATH`.
    #[derive(Default)]
    struct FakeFs {
        files: BTreeMap<PathBuf, String>,
        path_vp: Option<PathBuf>,
        unreliable_node_modules: bool,
    }

    impl FakeFs {
        fn file(mut self, path: &str, contents: &str) -> Self {
            self.files.insert(PathBuf::from(path), contents.to_string());
            self
        }

        /// A package.json declaring vite-plus as a devDependency.
        fn declares(self, dir: &str) -> Self {
            self.file(
                &format!("{dir}/package.json"),
                r#"{ "devDependencies": { "vite-plus": "^1.0.0" } }"#,
            )
        }

        /// A valid installed vite-plus package at `<dir>/node_modules/vite-plus`.
        fn install(self, dir: &str) -> Self {
            self.file(
                &format!("{dir}/node_modules/vite-plus/bin/vp"),
                "#!/usr/bin/env node\n",
            )
            .file(
                &format!("{dir}/node_modules/vite-plus/package.json"),
                r#"{ "name": "vite-plus", "version": "1.0.0" }"#,
            )
        }

        fn marker(self, dir: &str, file: &str) -> Self {
            self.file(&format!("{dir}/{file}"), "")
        }

        fn vp_on_path(mut self, path: &str) -> Self {
            self.path_vp = Some(PathBuf::from(path));
            self
        }

        /// Model Zed: reads inside `node_modules` are unreliable (zed#10760).
        fn unreliable_node_modules(mut self) -> Self {
            self.unreliable_node_modules = true;
            self
        }
    }

    impl FileSystem for FakeFs {
        fn read_text_file(&self, path: &Path) -> Option<String> {
            self.files.get(path).cloned()
        }

        fn file_exists(&self, path: &Path) -> bool {
            self.files.contains_key(path)
        }

        fn which(&self, binary: &str) -> Option<PathBuf> {
            (binary == "vp").then(|| self.path_vp.clone()).flatten()
        }

        fn can_read_node_modules(&self) -> bool {
            !self.unreliable_node_modules
        }
    }

    fn detect(fs: &FakeFs, start: &str) -> Option<DetectResult> {
        detect_vite_plus_project(fs, Path::new(start))
    }

    fn runnable(root: &str, vp: &str) -> Option<DetectResult> {
        Some(DetectResult {
            root: PathBuf::from(root),
            vp_path: Some(PathBuf::from(vp)),
        })
    }

    fn declared_only(root: &str) -> Option<DetectResult> {
        Some(DetectResult {
            root: PathBuf::from(root),
            vp_path: None,
        })
    }

    // -- RFC conformance fixtures ------------------------------------------

    #[test]
    fn root_declared_and_installed() {
        let fs = FakeFs::default().declares("/repo").install("/repo");
        assert_eq!(
            detect(&fs, "/repo"),
            runnable("/repo", "/repo/node_modules/vite-plus/bin/vp")
        );
    }

    #[test]
    fn pnpm_subpackage_declared_root_hoisted() {
        let fs = FakeFs::default()
            .marker("/repo", "pnpm-workspace.yaml")
            .file("/repo/package.json", "{}")
            .declares("/repo/packages/app")
            .install("/repo"); // hoisted to the workspace root
        assert_eq!(
            detect(&fs, "/repo/packages/app"),
            runnable("/repo/packages/app", "/repo/node_modules/vite-plus/bin/vp"),
        );
    }

    #[test]
    fn npm_subpackage_direct_dep_unhoisted() {
        let fs = FakeFs::default()
            .file("/repo/package.json", r#"{ "workspaces": ["packages/*"] }"#)
            .declares("/repo/packages/app")
            .install("/repo/packages/app"); // unhoisted, local to the subpackage
        assert_eq!(
            detect(&fs, "/repo/packages/app"),
            runnable(
                "/repo/packages/app",
                "/repo/packages/app/node_modules/vite-plus/bin/vp"
            ),
        );
    }

    #[test]
    fn root_declared_no_local_no_global() {
        let fs = FakeFs::default().declares("/repo");
        assert_eq!(detect(&fs, "/repo"), declared_only("/repo"));
    }

    #[test]
    fn root_declared_no_local_global_on_path() {
        let fs = FakeFs::default()
            .declares("/repo")
            .vp_on_path("/usr/local/bin/vp");
        assert_eq!(detect(&fs, "/repo"), runnable("/repo", "/usr/local/bin/vp"));
    }

    #[test]
    fn transitive_install() {
        // node_modules/vite-plus exists but no package.json declares it.
        let fs = FakeFs::default()
            .file("/repo/package.json", "{}")
            .install("/repo");
        assert_eq!(detect(&fs, "/repo"), None);
    }

    #[test]
    fn global_vp_without_declaration() {
        // Phase 1 fails, so Phase 3 (the $PATH vp) must never run.
        let fs = FakeFs::default()
            .file("/repo/package.json", "{}")
            .vp_on_path("/usr/local/bin/vp");
        assert_eq!(detect(&fs, "/repo"), None);
    }

    #[test]
    fn parent_vite_plus_nested_repo() {
        // Outer repo declares + installs vite-plus; the inner directory is its
        // own root workspace and does not. Detection must not cross the bound.
        let fs = FakeFs::default()
            .declares("/repo")
            .install("/repo")
            .marker("/repo/nested", "pnpm-workspace.yaml")
            .file("/repo/nested/package.json", "{}");
        assert_eq!(detect(&fs, "/repo/nested"), None);
    }

    #[test]
    fn plain_non_vite_plus() {
        let fs = FakeFs::default().file("/repo/package.json", r#"{ "dependencies": {} }"#);
        assert_eq!(detect(&fs, "/repo"), None);
    }

    #[test]
    fn yarn4_pnp() {
        // Berry/PnP: declared, but no node_modules and no global vp.
        let fs = FakeFs::default().declares("/repo");
        assert_eq!(detect(&fs, "/repo"), declared_only("/repo"));
    }

    // -- Zed-specific runtime boundary -------------------------------------

    #[test]
    fn zed_worktree_root_misses_subpackage_declaration() {
        // Zed's start path is always the worktree root and it cannot read
        // above it. When a user opens the monorepo root but only a subpackage
        // declares vite-plus, Phase 1 stops at the root-workspace marker and
        // returns null. This is the acknowledged single-root limitation.
        let fs = FakeFs::default()
            .marker("/repo", "pnpm-workspace.yaml")
            .file("/repo/package.json", "{}")
            .declares("/repo/packages/app")
            .install("/repo");
        assert_eq!(detect(&fs, "/repo"), None);
    }

    #[test]
    fn zed_mode_trusts_constructed_vp_path() {
        // Zed cannot reliably read node_modules, so even with NO visible
        // install files the detector trusts the conventional path once the
        // root package.json (reliably readable) declares vite-plus.
        let fs = FakeFs::default()
            .declares("/repo")
            .unreliable_node_modules();
        assert_eq!(
            detect(&fs, "/repo"),
            runnable("/repo", "/repo/node_modules/vite-plus/bin/vp"),
        );
    }

    #[test]
    fn zed_mode_plain_project_is_still_null() {
        // Phase 1 reads the root package.json, which IS reliable in Zed; a
        // plain project is still correctly rejected.
        let fs = FakeFs::default()
            .file("/repo/package.json", r#"{ "dependencies": {} }"#)
            .unreliable_node_modules();
        assert_eq!(detect(&fs, "/repo"), None);
    }

    #[test]
    fn stale_install_without_valid_manifest_is_not_runnable() {
        // bin/vp present but the package.json doesn't validate as vite-plus.
        let fs = FakeFs::default()
            .declares("/repo")
            .file(
                "/repo/node_modules/vite-plus/bin/vp",
                "#!/usr/bin/env node\n",
            )
            .file(
                "/repo/node_modules/vite-plus/package.json",
                r#"{ "name": "imposter" }"#,
            );
        assert_eq!(detect(&fs, "/repo"), declared_only("/repo"));
    }

    #[test]
    fn phase2_walks_up_to_intermediate_install() {
        // Declared in a deep subpackage; install hoisted to an intermediate
        // (non-root) directory. Phase 2 must find it on the way up.
        let fs = FakeFs::default()
            .file("/repo/package.json", r#"{ "workspaces": ["packages/*"] }"#)
            .file("/repo/packages/app/package.json", "{}")
            .declares("/repo/packages/app/sub")
            .install("/repo/packages/app");
        assert_eq!(
            detect(&fs, "/repo/packages/app/sub"),
            runnable(
                "/repo/packages/app/sub",
                "/repo/packages/app/node_modules/vite-plus/bin/vp"
            ),
        );
    }
}
