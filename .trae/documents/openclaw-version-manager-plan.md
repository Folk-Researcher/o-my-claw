# OpenClaw 版本管理功能实现计划

## 目标

实现一个完整的 OpenClaw 版本管理功能，包括：

1. 从 GitHub 获取 OpenClaw releases 列表
2. 检出指定版本的源代码
3. 编译 OpenClaw
4. 从源码启动 OpenClaw

## 技术架构

### 后端 (Rust)

* **GitHub 服务**: 调用 GitHub API 获取 releases

* **Git 服务**: 克隆仓库、检出指定版本

* **进程管理服务**: 编译和启动 OpenClaw

### 前端 (React)

* **ReleasesList 组件**: 显示所有可用版本

* **VersionDetail 组件**: 版本详情和操作按钮

* **ProcessConsole 组件**: 显示编译/运行日志

## 实现步骤

### Phase 1: 后端基础服务

#### Step 1.1: 添加 Rust 依赖

在 `src-tauri/Cargo.toml` 中添加：

* `reqwest` - HTTP 客户端

* `git2` - Git 操作

* `tokio` - 异步运行时和进程管理

#### Step 1.2: 创建 GitHub 服务

文件: `src-tauri/src/services/github.rs`

* 定义 `Release` 结构体

* 实现 `get_releases()` 函数获取 releases 列表

* 实现 `get_release_by_tag()` 函数获取指定版本

#### Step 1.3: 创建 Git 服务

文件: `src-tauri/src/services/git.rs`

* 定义 `clone_repository()` 函数克隆 OpenClaw 仓库

* 实现 `checkout_tag()` 函数检出指定 tag

* 实现 `pull_updates()` 函数更新代码

#### Step 1.4: 创建进程管理服务

文件: `src-tauri/src/services/process.rs`

* 实现 `compile_openclaw()` 函数编译 OpenClaw

* 实现 `start_openclaw()` 函数启动 OpenClaw

* 实现 `stop_openclaw()` 函数停止 OpenClaw

* 实现日志捕获和输出

#### Step 1.5: 创建 Tauri 命令

文件: `src-tauri/src/commands/version_manager.rs`

* `list_releases()` - 获取 releases 列表

* `checkout_version(tag)` - 检出指定版本

* `compile_version(tag)` - 编译指定版本

* `start_openclaw(tag)` - 启动 OpenClaw

* `stop_openclaw()` - 停止 OpenClaw

* `get_process_output()` - 获取进程输出

#### Step 1.6: 注册命令

更新 `src-tauri/src/main.rs` 注册新命令

### Phase 2: 前端界面

#### Step 2.1: 创建 ReleasesList 组件

文件: `src/components/ReleasesList.tsx`

* 显示 releases 列表

* 支持选择版本

* 显示版本状态（未安装/已下载/已编译/运行中）

#### Step 2.2: 创建 VersionDetail 组件

文件: `src/components/VersionDetail.tsx`

* 显示选中版本的详细信息

* 操作按钮：检出源码、编译、启动、停止

* 显示当前状态

#### Step 2.3: 创建 ProcessConsole 组件

文件: `src/components/ProcessConsole.tsx`

* 实时显示编译/运行日志

* 支持清空日志

* 自动滚动到底部

#### Step 2.4: 创建 VersionManager 页面

文件: `src/pages/VersionManager.tsx`

* 整合 ReleasesList、VersionDetail、ProcessConsole

* 管理状态和数据流

#### Step 2.5: 添加到路由

更新路由配置，添加版本管理页面

### Phase 3: 集成和测试

#### Step 3.1: 集成到主界面

在 `App.tsx` 或 Dashboard 中添加版本管理入口

#### Step 3.2: 测试后端命令

测试所有 Tauri 命令是否正常工作

#### Step 3.3: 测试前端界面

测试 UI 交互和数据展示

#### Step 3.4: 端到端测试

完整流程测试：获取 releases → 检出 → 编译 → 启动

## 数据结构设计

### Rust 结构体

```rust
// GitHub Release
pub struct Release {
    pub tag_name: String,
    pub name: String,
    pub body: String,
    pub published_at: String,
    pub html_url: String,
    pub tarball_url: String,
    pub zipball_url: String,
}

// 本地版本状态
pub struct LocalVersion {
    pub tag_name: String,
    pub source_path: String,
    pub status: VersionStatus,
    pub last_compiled: Option<String>,
}

pub enum VersionStatus {
    NotCloned,
    Cloned,
    Compiling,
    Compiled,
    Running,
    Error(String),
}
```

### TypeScript 类型

```typescript
interface Release {
  tag_name: string;
  name: string;
  body: string;
  published_at: string;
  html_url: string;
}

interface LocalVersion {
  tag_name: string;
  source_path: string;
  status: 'not_cloned' | 'cloned' | 'compiling' | 'compiled' | 'running' | 'error';
  last_compiled?: string;
}
```

## 界面设计

```
┌─────────────────────────────────────────────────────────────┐
│  o my claw - 版本管理                                        │
├──────────────────────┬──────────────────────────────────────┤
│                      │                                      │
│  ┌────────────────┐  │  ┌────────────────────────────────┐  │
│  │ 🔍 搜索版本...  │  │  │ 版本: v1.0.0                   │  │
│  └────────────────┘  │  │ 发布日期: 2024-01-15            │  │
│                      │  │ 说明: 重大更新...                │  │
│  ○ v1.0.0  ●       │  │                                │  │
│  ○ v0.9.2          │  │ 状态: 🔴 未安装                  │  │
│  ○ v0.9.1          │  │                                │  │
│  ○ v0.9.0          │  │ [检出源码] [编译] [启动] [停止]  │  │
│  ○ v0.8.5          │  │                                │  │
│                      │  │ ┌────────────────────────────┐ │  │
│                      │  │ │ 编译日志:                   │ │  │
│                      │  │ │ > git clone...             │ │  │
│                      │  │ │ > cargo build...           │ │  │
│                      │  │ │ > ...                      │ │  │
│                      │  │ └────────────────────────────┘ │  │
│                      │  └────────────────────────────────┘  │
└──────────────────────┴──────────────────────────────────────┘
```

## 依赖项

### Rust 依赖 (Cargo.toml)

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
git2 = "0.18"
tokio = { version = "1", features = ["full", "process"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 风险与注意事项

1. **GitHub API 限制**: 未认证用户每小时 60 次请求，建议添加 GitHub Token 配置
2. **编译时间**: OpenClaw 编译可能需要较长时间，需要显示进度
3. **磁盘空间**: 源码和编译产物占用空间较大，需要考虑清理机制
4. **权限问题**: 某些系统可能需要管理员权限才能运行编译后的程序

## 后续优化

1. 添加缓存机制，避免重复下载
2. 支持增量更新（git pull）
3. 添加版本对比功能
4. 支持回滚到之前的版本
5. 添加自动更新检测

