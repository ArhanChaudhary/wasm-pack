use anyhow::{anyhow, Result};
use command::utils::{find_pkg_directory, get_crate_path};
use log::info;
use npm;
use std::path::PathBuf;
use PBAR;

/// Executes the 'npm pack' command on the 'pkg' directory
/// which creates a tarball that can be published to the NPM registry
pub fn pack(path: Option<PathBuf>) -> Result<()> {
    let crate_path = get_crate_path(path)?;

    info!("Packing up the npm package...");
    let pkg_directory = find_pkg_directory(&crate_path).ok_or_else(|| {
        anyhow!(
            "Unable to find the pkg directory at path {:#?}, or in a child directory of {:#?}",
            &crate_path,
            &crate_path
        )
    })?;
    npm::npm_pack(&pkg_directory.to_string_lossy())?;
    info!("Your package is located at {:#?}", crate_path.join("pkg"));

    PBAR.info("🎒  packed up your package!");
    Ok(())
}
