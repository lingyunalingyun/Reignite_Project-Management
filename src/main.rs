mod core;
use rfd::FileDialog; // 引入系统文件夹选择对话框
use slint::{ModelRc, SharedString, VecModel};
use std::path::Path;
slint::include_modules!();  //将 build.rs 中编译的 slint 文件包含进来

fn main() -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;  //创建一个 MainWindow 实例
    let window_weak = window.as_weak(); // 创建窗口弱引用，供回调中使用
    window.on_select_folder_clicked(move || {  // 注册选择文件夹按钮的回调
        if let Some(folder) = FileDialog::new()
            .set_title("选择目标文件夹")
            .pick_folder()
        { // Some 表示用户选择了一个文件夹
            let folder_text = folder.to_string_lossy().to_string(); // 将 PathBuf 转成文字路径

            let files = core::scanner::scan_and_sort(Path::new(&folder_text));
            let file_names: Vec<SharedString> = files
                .into_iter()
                .map(|file| file.path.into())
                .collect();
            let file_count = file_names.len();

            if let Some(window) = window_weak.upgrade() { // 确认窗口仍然存在
                window.set_target_folder(folder_text.clone().into()); // 将路径回填到 Slint 输入框
                window.set_scanned_files(ModelRc::new(VecModel::from(file_names)));
                window.set_status_text(format!("扫描完成，共 {} 个文件", file_count).into());
            }

            println!("目标文件夹：{}", folder_text);
        } else if let Some(window) = window_weak.upgrade() { // None 表示用户取消了选择
            window.set_status_text("已取消选择".into());
        }
    });
    window.run()  //让窗口运行起来
}
