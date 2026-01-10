# Maintenance Guide

## Creating a New Release

To publish a new version of the Oxc extension for Zed, you need to submit a PR to the official Zed extensions repository.

### Steps

1. **Merge the latest release PR** in https://github.com/oxc-project/oxc-zed/pulls
   - This will automatically update the version in both `Cargo.toml` and `extension.toml`

2. **Create a PR to the Zed extensions repository**:
   - Fork https://github.com/zed-industries/extensions if you haven't already
   - Clone your fork locally
   - Update the `oxc` extension submodule to point to the latest commit from this repository
     - General steps that have been used for this before for those unfamiliar with submodules.
       1. Open your forked repo.
       2. `cd extensions/oxc`
       3. `git fetch origin && git pull origin`
       4. `cd ../../`
       5. `git -C extensions/oxc checkout <oxc-zed-commit-sha-to-release>`
       6. Manually edit `extensions.toml` with the new oxc-zed version.
       7. `git add extensions.toml extensions/oxc`
       8. `git commit -m 'Update oxc to v<oxc-zed-version>`
       9. `git push --set-upstream origin oxc-v<oxc-zed-version>`
   - Create a PR with your changes

### Example PR

See https://github.com/zed-industries/extensions/pull/4246 for an example of a release PR.

The PR should:
- Update the submodule reference to point to the new version
- Include a clear description of what changed
- Follow the Zed extensions repository's contribution guidelines
