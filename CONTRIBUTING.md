# Contributing

Thanks for your interest in improving TraceDraw Launcher. Contributions can include bug reports, usability feedback, Linux distribution instructions, documentation, or code.

## Before opening an issue

- Search existing issues for the same problem.
- Include your Linux distribution and version, desktop environment, CPU architecture, and the launcher version or Git commit.
- For a bug, describe the steps to reproduce it and include the complete error message. Remove personal paths or other private information if needed.
- Do not attach private TraceDraw drawings or other files unless you have permission to share them.

## Pull requests

1. Fork the repository and create a focused branch.
2. Explain the user-visible problem and the change in the pull request description.
3. Keep Linux-specific behavior explicit. Changes for Windows or macOS should include the platform-specific installation and file-association approach; the current implementation uses Linux XDG paths and tools.
4. Update the README or this guide when behavior or setup requirements change.
5. Run formatting and build checks before submitting:

   ```bash
   cargo fmt -- --check
   cargo check --release
   ```

Please do not bundle TraceDraw release binaries in pull requests or project releases. The launcher fetches official binaries from the upstream TraceDraw repository.

## Licensing contributions

By submitting a contribution, you agree that it may be distributed under the project license. The repository is dual-licensed under MIT or Apache-2.0, at your option. See the license files for details.
