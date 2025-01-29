/**
 * Returns a list of all executables in the PATH environment variable
 */
pub fn read_path() -> Vec<std::path::PathBuf> {
    let path = std::env::var("PATH").unwrap();
    let paths: Vec<&str> = path.split(":").collect();

    let mut executables = Vec::new();

    for path in paths {
        let entries = std::fs::read_dir(path).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                executables.push(path);
            }
        }
    }
    return executables;
}

/**
 * Returns a list of all files in the current directory
 */
pub fn read_current_dir_files(dir: Option<&str>) -> Vec<std::path::PathBuf> {
    let dir = dir.unwrap_or(".");
    let entries = std::fs::read_dir(dir).unwrap();
    let mut executables = Vec::new();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            executables.push(path);
        }
    }
    return executables;
}

pub fn read_current_dir_dirs(dir: Option<&str>) -> Vec<std::path::PathBuf> {
    let dir = dir.unwrap_or(".");
    let entries = std::fs::read_dir(dir).unwrap();
    let mut executables = Vec::new();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            executables.push(path);
        }
    }
    return executables;
}