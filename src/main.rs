mod core;
slint::include_modules!();  //将 build.rs 中编译的 slint 文件包含进来

fn main() -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;  //创建一个 MainWindow 实例
    window.set_app_title("Reignite".into());  // Rust 设置 Slint 的标题属性
    window.set_app_description("Manage forgotten projects and bring them back to life.".into());  // Rust 设置 Slint 的说明属性
    window.on_revive_clicked(|| {  // 注册 Slint 的点击回调
        println!("回调按钮被点击");
    });
    window.run()  //让窗口运行起来
}
