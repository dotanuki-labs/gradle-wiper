// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use std::fs;
use std::path::PathBuf;

pub fn cleanup_resources(paths_to_remove: &[PathBuf]) {
    println!();
    println!("Removing the following :");
    println!();

    for path in paths_to_remove {
        println!("{}", path.to_str().expect("Not a valid path"));
    }

    println!();

    let errors = paths_to_remove
        .iter()
        .map(fs::remove_dir_all)
        .filter(|deletion| deletion.is_err())
        .map(|deletion| deletion.expect_err("Expecting an error").to_string())
        .collect::<Vec<String>>();

    if !errors.is_empty() {
        println!("Some of the target repositories were not removed (not found)")
    }
}

#[cfg(test)]
mod tests {
    use crate::disk::cleanup_resources;
    use std::fs;
    use std::path::PathBuf;
    use temp_dir::TempDir;

    #[test]
    fn should_remove_target_paths_ignoring_errors() {
        let temp_dir = TempDir::new().expect("Cant create temp dir");
        let fake_user_home = temp_dir.path();

        let folders = [".gradle", ".gradle/caches", ".gradle/daemon"];

        for folder in folders {
            fs::create_dir(fake_user_home.join(folder)).expect("Cant create temporary fixture folder");
        }

        let to_remove = [
            ".gradle/caches",
            ".gradle/daemon",
            ".gradle/jdks",
            ".m2",
            "AndroidStudioProjects/my-project/build",
        ];

        let paths = to_remove.iter().map(PathBuf::from).collect::<Vec<_>>();

        cleanup_resources(&paths);

        paths.into_iter().for_each(|path| assert!(!path.exists()))
    }
}
