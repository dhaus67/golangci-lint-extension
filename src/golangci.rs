// Copyright 2024 dhaus67
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fs;

use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

struct GolangCiLintBinary {
    path: String,
}

struct GolangCiLintExtension {
    cached_binary_path: Option<String>,
}

impl GolangCiLintExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<GolangCiLintBinary> {
        let binary_settings = LspSettings::for_worktree("golang-ci", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(GolangCiLintBinary { path });
        }

        if let Some(path) = worktree.which("golang-ci-lint-langserver") {
            return Ok(GolangCiLintBinary { path });
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(GolangCiLintBinary { path: path.clone() });
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "nametake/golangci-lint-langserver",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();

        let asset_name = format!(
            "golangci-lint-langserver_{os}_{arch}.{suffix}",
            os = match platform {
                zed::Os::Linux => "Linux",
                zed::Os::Mac => "Darwin",
                zed::Os::Windows => "Windows",
            },
            arch = match arch {
                zed::Architecture::Aarch64 => "arm64",
                zed::Architecture::X86 => "i386",
                zed::Architecture::X8664 => "x86_64",
            },
            suffix = match platform {
                zed::Os::Windows => "zip",
                _ => "tar.gz",
            }
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("golangci-lint-langserver-{}", release.version);
        let binary_path = format!("{version_dir}/golangci-lint-langserver");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                &language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                match platform {
                    zed::Os::Windows => zed::DownloadedFileType::Zip,
                    _ => zed::DownloadedFileType::GzipTar,
                },
            )?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(GolangCiLintBinary { path: binary_path })
    }
}

impl zed::Extension for GolangCiLintExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<zed_extension_api::Command> {
        let golang_ci_lint_binary = self.language_server_binary(language_server_id, worktree)?;

        Ok(zed::Command {
            command: golang_ci_lint_binary.path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(GolangCiLintExtension);
