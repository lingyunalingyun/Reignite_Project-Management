//递归扫描目录

use std::fs;
use std::path::Path;
use std::time::SystemTime;

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



pub fn get_file_info(path: &Path) -> Option<(SystemTime, u64)> {              //获取文件信息函数，返回修改时间和大小
    let metadata = fs::metadata(path).ok()?;
    let modified = metadata.modified().ok()?;
    let size = metadata.len();
    Some((modified, size))
}



fn scan_dir(dir: &Path, results: &mut Vec<FileInfo>) {                       //扫描文件夹函数
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries {
        let path = match entry {
            Ok(e) => e.path(),
            Err(_) => continue,
        };

        if path.is_dir() {
                                             // 是文件夹，递归进入
            scan_dir(&path, results);
        } else if path.is_file() {
                                             // 是文件，获取修改时间并保存
            if let Some((modified, size)) = get_file_info(&path) {
                results.push(FileInfo {
                    path: path.to_string_lossy().to_string(),
                    modified,
                    size,
                    modify_count: 0,  // 修改次数暂时用0占位
                });
            }
        }
    }
}



fn sort_files(files: &mut Vec<FileInfo>, method: &SortMethod) {                  //排序函数
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



pub fn scan_and_sort(dir: &Path, method: Option<SortMethod>) -> Vec<FileInfo> {              //对外公开的入口函数
    let mut files = Vec::new();
    scan_dir(dir, &mut files);
    let method = method.unwrap_or(SortMethod::ModifiedTimeAsc);
    sort_files(&mut files, &method);
    files
}



