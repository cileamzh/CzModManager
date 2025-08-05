# CzModManager


<p align="center">
  <img src="modmanager.png" alt="CzModManager Logo" width="150"/>
</p>

<p align="center">
  <a href="https://github.com/cileamzh/CzModManager/releases">
    <img src="https://img.shields.io/github/v/release/cileamzh/CzModManager?label=release" alt="Release Version">
  </a>
  <a href="https://github.com/cileamzh/CzModManager/stargazers">
    <img src="https://img.shields.io/github/stars/cileamzh/CzModManager" alt="Stars">
  </a>
  <a href="https://github.com/cileamzh/CzModManager/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/cileamzh/CzModManager" alt="License">
  </a>
</p>

> 🛠️ 一款简洁高效的 3DMigoto Mod 管理器，支持启用/禁用、分类、预览和快速切换 Mod。

---

## ✨ 项目介绍

**CzModManager** 是一款用于管理 [3DMigoto](https://github.com/bo3b/3Dmigoto) Mod 的桌面程序，适用于需要快速整理、启用和禁用游戏 Mod 的玩家。

支持多种游戏，包括但不限于 Unity、Unreal 和 DX11 平台游戏。

---

## 🧩 功能特性

- ✅ 一键启用 / 禁用 Mod（无需手动改名）
- 🗂️ 支持按类型/游戏分类管理
- 🖼️ Mod 图片预览（如有）
- 🌐 多语言支持（可选）

---

## 📦 安装方式

### 方法一：从 Release 页面下载

1. 前往 [Release 页面](https://github.com/cileamzh/CzModManager/releases)
2. 下载对应系统的压缩包或安装包
3. 解压后运行 `CzModManager.exe`（Windows）

### 方法二：源码构建（适用于开发者）

```bash
git clone https://github.com/cileamzh/CzModManager.git
cd CzModManager
# 根据项目类型执行构建，例如：
npm install
npm run tauri dev
