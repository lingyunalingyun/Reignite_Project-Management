fn main() {
    slint_build::compile("ui/main.slint").unwrap();  //在项目编译时将 main.slint 编译为 Rust 代码
}