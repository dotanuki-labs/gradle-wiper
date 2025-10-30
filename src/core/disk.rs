// Copyright 2025 Dotanuki Labs
// SPDX-License-Identifier: MIT

mod locations;
mod operations;
mod resources;

use directories::BaseDirs;
pub use locations::find_all_gradle_projects;
pub use locations::find_associated_filepaths;
pub use locations::find_gradle_home;
pub use locations::find_konan_caches;
pub use locations::find_maven_local_repository;
pub use operations::cleanup_resources;
pub use resources::resources_used_by_gradle_home;
pub use resources::resources_used_by_gradle_projects;
pub use resources::resources_used_by_konan;
pub use resources::resources_used_by_maven_local_repository;
use std::path::PathBuf;

pub fn user_home_locator() -> PathBuf {
    let base_dirs = BaseDirs::new().expect("Cannot retrieve standard system dirs");
    let home_dir = base_dirs.home_dir();
    home_dir.to_path_buf()
}
