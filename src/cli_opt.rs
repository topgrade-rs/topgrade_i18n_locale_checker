//! This module defines this tool's CLI options.

use clap::Parser;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
pub(crate) struct Cli {
    /// The path to the locale file
    #[arg(long)]
    locale_file: PathBuf,
    /// Rust files to check.
    ///
    /// If any path points to a directory, then all the Rust files in that directory
    /// will be checked.
    #[arg(long, required = true)]
    rust_src_to_check: Vec<PathBuf>,
}

impl Cli {
    /// Accesses the `--locale-file` option.
    pub(crate) fn locale_file(&self) -> &Path {
        &self.locale_file
    }

    /// Flattens the input paths and returns it.
    ///
    /// * For directories, it will walk through the directory and get all the Rust
    ///   files.
    /// * For symbolic links, it will convert it to the path it points to.
    pub(crate) fn rust_src_to_check(&self) -> Vec<Cow<Path>> {
        let mut rust_files_to_check = Vec::with_capacity(self.rust_src_to_check.len());

        for entry_path in self.rust_src_to_check.iter() {
            let entry_metadata = std::fs::symlink_metadata(&entry_path).unwrap_or_else(|e| {
                panic!(
                    "Error: cannot get the metadata of the specified file {} due to error {:?}",
                    entry_path.display(),
                    e
                )
            });

            if entry_metadata.is_file() {
                if is_rust_file(entry_path) {
                    rust_files_to_check.push(Cow::Borrowed(entry_path.as_path()));
                }
            } else if entry_metadata.is_dir() {
                let walk_dir_iter = walkdir::WalkDir::new(entry_path);
                for res_entry in walk_dir_iter {
                    let entry = res_entry.unwrap_or_else(|e| {
                        panic!(
                            "Error: cannot get the entry of the specified file due to error {:?}",
                            e
                        )
                    });

                    let entry_path = entry.path();
                    let entry_metadata = entry.metadata().unwrap_or_else(|e| {
                        panic!(
                            "Error: cannot get the metadata of the specified file {} due to error {:?}",
                            entry_path.display(),
                            e
                        )
                    });

                    if entry_metadata.is_file() {
                        if is_rust_file(entry_path) {
                            rust_files_to_check.push(Cow::Owned(entry_path.to_path_buf()));
                        }
                    }
                }
            } else if entry_metadata.is_symlink() {
                let file = std::fs::read_link(&entry_path).unwrap_or_else(|e| {
                    panic!(
                        "Error: cannot read the symlink of the specified file {} due to error {:?}",
                        entry_path.display(),
                        e
                    )
                });
                if is_rust_file(&file) {
                    rust_files_to_check.push(Cow::Owned(file));
                }
            }
        }

        rust_files_to_check
    }
}

fn is_rust_file<P: AsRef<Path> + ?Sized>(file_path: &P) -> bool {
    const RUST_FILE_EXTENSION: &str = "rs";

    if let Some(extension) = file_path.as_ref().extension() {
        if extension == RUST_FILE_EXTENSION {
            return true;
        }
    }

    false
}
