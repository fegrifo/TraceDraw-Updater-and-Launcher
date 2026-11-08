# Publishing this project on GitHub

This guide publishes the **source repository**. It does not publish a ready-to-run executable or upload TraceDraw binaries.

## 1. Prepare the repository

Before making the repository public:

- Review the README and remove any local-only or private information.
- Confirm the project name and GitHub username you want to use.
- Review the `LICENSE-MIT` copyright holder and change it if needed. The project is dual-licensed under MIT or Apache-2.0, matching `Cargo.toml`.
- Check that the TraceDraw name and logo are used only to describe compatibility. This launcher is an independent project.
- Build from a clean checkout and review the files that will be committed.

The `.gitignore` excludes Cargo build output and local editor state.

## 2. Create an empty GitHub repository

On GitHub, choose **New repository**. Pick a repository name such as `tracedraw-launcher`. Leave GitHub's README, `.gitignore`, and license initialization options unchecked so the remote repository starts empty.

GitHub's official guide: [Adding locally hosted code to GitHub](https://docs.github.com/en/migrations/importing-source-code/using-the-command-line-to-import-source-code/adding-locally-hosted-code-to-github).

## 3. Initialize and push the source

Create a clean local folder, copy the project files into it, then initialize Git. Replace `YOUR-USERNAME`, the repository name, and `PROJECT_SOURCE` with your GitHub account, repository name, and the folder containing this project. This avoids copying local build output or the workspace's Git metadata.

```bash
mkdir -p ~/Projects/tracedraw-launcher
cd ~/Projects/tracedraw-launcher
PROJECT_SOURCE="/path/to/current/project"
cp "$PROJECT_SOURCE"/Cargo.toml "$PROJECT_SOURCE"/Cargo.lock \
  "$PROJECT_SOURCE"/README.md "$PROJECT_SOURCE"/CONTRIBUTING.md \
  "$PROJECT_SOURCE"/LICENSE-MIT "$PROJECT_SOURCE"/LICENSE-APACHE \
  "$PROJECT_SOURCE"/.gitignore .
cp -a "$PROJECT_SOURCE"/docs "$PROJECT_SOURCE"/src .
git init -b main
git add Cargo.toml Cargo.lock README.md CONTRIBUTING.md LICENSE-MIT LICENSE-APACHE .gitignore docs src
git status
git commit -m "Initial TraceDraw Launcher source"
git remote add origin https://github.com/YOUR-USERNAME/tracedraw-launcher.git
git push -u origin main
```

Review `git status` before committing. If Git asks you to set an author identity, use the name and email you want associated with your public commits:

```bash
git config --global user.name "Your Name"
git config --global user.email "you@example.com"
```

If you prefer not to publish your email address, configure the private commit email shown in your GitHub account settings.

## 4. Publish a source release

After the source is pushed and you are ready to mark the first version, create and push a tag matching the version in `Cargo.toml`:

```bash
git tag -a v0.1.0 -m "TraceDraw Launcher v0.1.0"
git push origin v0.1.0
```

Then open **Releases** on GitHub, choose the `v0.1.0` tag, add a short description, and publish the release. This is a source release unless you separately build and attach launcher binaries. See GitHub's guide to [managing releases](https://docs.github.com/en/repositories/releasing-projects-on-github/managing-releases-in-a-repository).

## 5. Later updates

Make and review changes locally, commit them, and push to `main`:

```bash
git add .
git status
git commit -m "Describe the change"
git push
```

For a new release, update `version` in `Cargo.toml`, commit that change, then create and push a matching version tag. If you publish executable builds, identify the target OS and architecture in each asset name and document how users can verify its source.

## Important distinction

The current launcher obtains TraceDraw from `tracedraw/tracedraw` releases. A GitHub release for this launcher should contain its source and, later if desired, launcher builds only. Do not upload TraceDraw's executables as if they were produced by this project.
