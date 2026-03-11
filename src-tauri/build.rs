//! Tauri 构建脚本
//! 
//! 这个文件在编译时运行，用于执行 Tauri 特定的构建任务，
//! 例如生成资源文件、设置图标等。

/// 构建脚本入口函数
/// 
/// 调用 tauri_build::build() 执行 Tauri 的构建流程
fn main() {
    tauri_build::build()
}
