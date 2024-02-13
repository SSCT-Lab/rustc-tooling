use std::path::PathBuf;

use rustc_span::{FileName, FileNameDisplayPreference};

pub fn filename_to_pathbuf(file_name: &FileName) -> PathBuf {
    match file_name {
        FileName::Real(path) => PathBuf::from(path.to_string_lossy(FileNameDisplayPreference::Local).into_owned()),
        _ => PathBuf::new()
    }
}

