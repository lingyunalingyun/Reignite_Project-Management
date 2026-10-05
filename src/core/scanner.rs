//递归扫描目录

use std::fs;
use std::path::Path;
use std::time::SystemTime;

// 命中任意一个文件/后缀即视为"项目目录"，扫描时整体跳过、不递归进入
pub const PROJECT_MARKERS: &[&str] = &[
    "cargo.toml",
    "package.json",
    "go.mod",
    "cmakelists.txt",
    "setup.py",
    "pyproject.toml",
    "mix.exs",
    "gemfile",
];
// 额外允许的后缀匹配（小写）
pub const PROJECT_SUFFIXES: &[&str] = &[".sln"];

pub enum SortMethod {
    ModifiedTimeAsc,    // 修改时间升序
    FileSizeDesc,       // 文件大小降序
    ModifyCountDesc,    // 修改次数降序
}

pub struct FileInfo {                               //定义文件和修改时间的结构体
    pub path: String,
    pub modified: SystemTime,
    pub size: u64,
    pub modify_count: u32,
}

// 扫描失败的原因分类
pub enum ScanErrorKind {
    PermissionDenied,  // 权限不足
    NotFound,          // 路径不存在
    Other,             // 其他
}

fn err_to_kind(err: &std::io::Error) -> ScanErrorKind {
    match err.kind() {
        std::io::ErrorKind::PermissionDenied => ScanErrorKind::PermissionDenied,
        std::io::ErrorKind::NotFound => ScanErrorKind::NotFound,
        _ => ScanErrorKind::Other,
    }
}

// 单条失败记录
pub struct FileScanError {
    pub path: String,
    pub kind: ScanErrorKind,
    pub message: String,
}

// 项目目录的简洁展示：只记录目录完整路径 + 第一层条目名
pub struct ProjectSummary {
    pub path: String,      // 项目目录完整路径
    pub items: Vec<String> // 目录下第一层的所有条目名（文件+子目录）
}

// 扫描返回值：正常文件 + 失败记录（失败记录将在 UI 中展示在所有文件之后）+ 项目目录
pub struct ScanResult {
    pub files: Vec<FileInfo>,
    pub errors: Vec<FileScanError>,
    pub projects: Vec<ProjectSummary>,
}


pub fn get_file_info(path: &Path) -> Result<(SystemTime, u64), std::io::Error> {     //获取文件信息函数，返回修改时间和大小
    let metadata = fs::metadata(path).map_err(|e| std::io::Error::new(e.kind(), format!("{}: {}", path.display(), e)))?;
    let modified = metadata.modified().map_err(|e| std::io::Error::new(e.kind(), format!("{}: {}", path.display(), e)))?;
    let size = metadata.len();
    Ok((modified, size))
}



fn is_project_dir(dir: &Path) -> bool {                                 //判断目录是否为项目目录（含标志文件即命中）
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return false,
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let name = match entry.file_name().to_str() {
            Some(n) => n.to_lowercase(),
            None => continue,
        };
        if PROJECT_MARKERS.contains(&name.as_str()) {
            return true;
        }
        if PROJECT_SUFFIXES.iter().any(|suffix| name.ends_with(suffix)) {
            return true;
        }
    }
    false
}


fn scan_dir(dir: &Path, results: &mut Vec<FileInfo>, errors: &mut Vec<FileScanError>, projects: &mut Vec<ProjectSummary>) {         //扫描文件夹函数
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            // 目录读取失败（如权限不足），记录该目录本身后直接返回
            errors.push(FileScanError {
                path: dir.to_string_lossy().to_string(),
                kind: err_to_kind(&e),
                message: e.to_string(),
            });
            return;
        }
    };

    for entry in entries {
        let path = match entry {
            Ok(e) => e.path(),
            Err(_) => continue,
        };

        if path.is_dir() {
                                             // 是文件夹：命中项目标志文件则收集为项目摘要并跳过递归，否则正常递归进入
            if is_project_dir(&path) {
                let mut items: Vec<String> = Vec::new();
                if let Ok(entries_iter) = fs::read_dir(&path) {
                    for e in entries_iter {
                        if let Ok(e) = e {
                            if let Some(name) = e.file_name().to_str() {
                                items.push(name.to_string());
                            }
                        }
                    }
                }
                projects.push(ProjectSummary {
                    path: path.to_string_lossy().to_string(),
                    items,
                });
            } else {
                scan_dir(&path, results, errors, projects);
            }
        } else if path.is_file() {
                                             // 是文件，获取修改时间并保存（失败则记录原因）
            match get_file_info(&path) {
                Ok((modified, size)) => {
                    results.push(FileInfo {
                        path: path.to_string_lossy().to_string(),
                        modified,
                        size,
                        modify_count: 0,  // 修改次数暂时用0占位
                    });
                }
                Err(e) => {
                    errors.push(FileScanError {
                        path: path.to_string_lossy().to_string(),
                        kind: err_to_kind(&e),
                        message: e.to_string(),
                    });
                }
            }
        }
    }
}



pub fn sort(files: &mut Vec<FileInfo>, method: &SortMethod) {                  //排序函数
    match method {
        SortMethod::ModifiedTimeAsc => {
            files.sort_by(|a, b| a.modified.cmp(&b.modified));
        }
        SortMethod::FileSizeDesc => {
            files.sort_by(|a, b| b.size.cmp(&a.size));
        }
        SortMethod::ModifyCountDesc => {
            // 修改次数排序暂时按0占位，后续实现
            files.sort_by(|a, b| b.modify_count.cmp(&a.modify_count));
        }
    }
}



pub fn scan(dir: &Path) -> ScanResult {                                               //对外公开的入口函数，只扫描不排序
    let mut files = Vec::new();
    let mut errors = Vec::new();
    let mut projects = Vec::new();
    scan_dir(dir, &mut files, &mut errors, &mut projects);
    ScanResult { files, errors, projects }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_scan_current_dir() {
        let cwd = Path::new(".");
        let result = scan(cwd);
        let _file_count = result.files.len();
        let _err_count = result.errors.len();
        let _project_count = result.projects.len();
    }

    #[test]
    fn smoke_sort_by_modified() {
        let cwd = Path::new(".");
        let mut result = scan(cwd);
        sort(&mut result.files, &SortMethod::ModifiedTimeAsc);
        let _ = result.files.len();
    }
}



