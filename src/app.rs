use std::path::{Path, PathBuf};

pub fn app_file<S: AsRef<str>, P: AsRef<Path>>(
    app_name: S,
    file_name: P,
) -> std::io::Result<PathBuf> {
    let app_dir = directories_next::BaseDirs::new()
        .unwrap()
        .home_dir()
        .join(format!(".{}", app_name.as_ref()));
    std::fs::create_dir_all(&app_dir)?;
    Ok(app_dir.join(file_name))
}
