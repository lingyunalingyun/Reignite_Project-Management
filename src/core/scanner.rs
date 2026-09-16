//递归扫描目录

use std::fs;
use std::path::Path;
use std::time::SystemTime;

pub struct FileInfo{                               //定义文件和修改时间的结构体
    path: String,
    modified: SystemTime,
}



fn get_modified_time(path:&path)->Option<SystemTime>{                     //获取文件修改时间函数
    let metadata=fs::metadata(path).ok()?;
    metadata.modified().ok()
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
            if let Some(modified) = get_modified_time(&path) {
                results.push(FileInfo {
                    path: path.to_string_lossy().to_string(),
                    modified,
                });
            }
        }
    }
}



fn sort_by_modified(files: &mut Vec<FileInfo>) {                       //排序函数
    files.sort_by(|a, b| a.modified.cmp(&b.modified));
}



pub fn scan_and_sort(dir: &Path) -> Vec<FileInfo> {              //对外公开的入口函数
    let mut files = Vec::new();
    scan_dir(dir, &mut files);
    sort_by_modified(&mut files);
    files
}

