slint::include_modules!();  // 将 build.rs 中编译的 slint 文件包含进来

fn main() -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;  //创建一个 MainWindow 实例
    window.run()  //让窗口运行起来
}