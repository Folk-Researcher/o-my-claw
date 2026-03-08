# 设置环境变量，添加cargo路径
$env:PATH += ";C:\Users\liche\.cargo\bin"

# 调试信息
Write-Host "PATH环境变量: $env:PATH"
Write-Host "cargo版本:"
& cargo --version
Write-Host "开始运行Tauri开发环境..."

# 运行Tauri开发环境
npx tauri dev