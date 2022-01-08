use crate::cli::download_spinner::DownloadSpinner;
use crate::cli::handlers::select;
use crate::cli::handlers::{HandlerError, HandlerResult};
use crate::github;
use crate::github::error::GithubError;
use crate::github::release::{Asset, Release};
use crate::github::tagged_asset::TaggedAsset;
use crate::github::Repository;
use std::fs::File;
use std::path::{Path, PathBuf};

pub struct DownloadHandler {
    repository: Repository,
    select: Option<String>,
    output: Option<PathBuf>,
}

impl DownloadHandler {
    pub fn new(repository: Repository, select: Option<String>, output: Option<PathBuf>) -> Self {
        DownloadHandler {
            repository,
            select,
            output,
        }
    }

    pub fn run(&self) -> HandlerResult {
        let release = self.fetch_latest_release()?;
        let selected_asset = self.autoselect_or_ask_asset(release)?;
        let output_path = Self::output_path_from(self.output.as_ref(), &selected_asset.name);
        Self::download_asset(&selected_asset, output_path)?;
        Ok(())
    }

    fn autoselect_or_ask_asset(&self, release: Release) -> Result<Asset, HandlerError> {
        if let Some(untagged) = self.select.as_ref() {
            Self::autoselect_asset(release, untagged)
        } else {
            Self::ask_select_asset(release.assets)
        }
    }

    fn autoselect_asset(release: Release, untagged: &str) -> Result<Asset, HandlerError> {
        let asset_name = TaggedAsset::tag(&release.tag, untagged);
        release
            .assets
            .into_iter()
            .find(|x| x.name == asset_name)
            .ok_or_else(|| HandlerError::new(format!("No asset found for {}", untagged)))
    }

    fn fetch_latest_release(&self) -> Result<Release, HandlerError> {
        github::get_release(&self.repository, None).map_err(Self::release_error)
    }

    fn ask_select_asset(assets: Vec<Asset>) -> select::AskSelectAssetResult {
        select::ask_select_asset(
            assets,
            select::Messages {
                select_prompt: "Pick the asset to download",
                quit_select: "No asset selected",
            },
        )
    }

    fn download_asset(selected_asset: &Asset, output_path: &Path) -> Result<(), HandlerError> {
        let spinner = DownloadSpinner::new(&selected_asset.name, output_path);
        spinner.start();
        let mut stream = github::download_asset(selected_asset).map_err(Self::download_error)?;
        let mut destination = Self::create_file(output_path)?;
        std::io::copy(&mut stream, &mut destination).unwrap();
        spinner.stop();
        Ok(())
    }

    fn output_path_from<'a>(output: Option<&'a PathBuf>, asset_name: &'a str) -> &'a Path {
        output
            .map(|x| x.as_path())
            .unwrap_or_else(|| Path::new(asset_name))
    }

    fn create_file(path: &Path) -> Result<File, HandlerError> {
        File::create(path).map_err(|e| {
            HandlerError::new(format!(
                "Failed to create the file {}: {}",
                path.display(),
                e.to_string()
            ))
        })
    }

    fn release_error(e: GithubError) -> HandlerError {
        HandlerError::new(format!("Error fetching latest release: {}", e))
    }

    fn download_error(e: GithubError) -> HandlerError {
        HandlerError::new(format!("Error downloading asset: {}", e))
    }
}
