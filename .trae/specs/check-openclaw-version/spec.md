# OpenClaw 版本检查功能 Spec

## Why
用户需要能够检查系统中已安装的 OpenClaw 版本，以便确认 OpenClaw 是否正确安装、版本是否兼容，以及是否需要更新。

## What Changes
- 添加后端命令 `check_openclaw_version` 用于检测系统中 OpenClaw 的安装状态和版本信息
- 添加前端界面展示 OpenClaw 版本检查结果
- 支持检测 OpenClaw 是否已安装、安装路径、版本号
- **BREAKING**: 无破坏性变更

## Impact
- Affected specs: 启动器核心功能、版本管理
- Affected code: 
  - `src-tauri/src/commands/` - 新增版本检查命令
  - `src-tauri/src/services/` - 新增 OpenClaw 检测服务
  - `src/pages/` 或 `src/components/` - 新增版本检查 UI

## ADDED Requirements

### Requirement: OpenClaw 版本检测
The system SHALL 提供检测系统中 OpenClaw 版本的能力。

#### Scenario: OpenClaw 已安装
- **GIVEN** OpenClaw 已安装在系统中
- **WHEN** 用户触发版本检查
- **THEN** 系统返回 OpenClaw 的版本号、安装路径和安装状态

#### Scenario: OpenClaw 未安装
- **GIVEN** OpenClaw 未安装在系统中
- **WHEN** 用户触发版本检查
- **THEN** 系统返回 "未安装" 状态，并提示用户安装

#### Scenario: OpenClaw 安装但版本不兼容
- **GIVEN** OpenClaw 已安装但版本过旧
- **WHEN** 用户触发版本检查
- **THEN** 系统返回当前版本号，并提示需要更新

### Requirement: 版本检查 UI
The system SHALL 提供用户界面展示版本检查结果。

#### Scenario: 成功检测到版本
- **WHEN** 版本检查完成
- **THEN** UI 显示版本号、安装路径、状态图标（绿色表示正常）

#### Scenario: 未检测到 OpenClaw
- **WHEN** 版本检查发现未安装
- **THEN** UI 显示红色警告状态，并提供"如何安装"链接

## MODIFIED Requirements
无

## REMOVED Requirements
无
