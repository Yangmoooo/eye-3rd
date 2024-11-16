# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.0.0] - 2024-11-16

为了统一隐写文件和普通压缩文件的处理方式，将逐步添加对 zip、7z 和 rar 格式的支持

当然，出于性能方面的考量，对于大文件（如 4GB 以上）的处理，仍然建议使用专业的解压软件

### Added

- 支持解压 zip 格式

### Changed

- 重构代码，提供统一的 `extract()` 方法用于解压文件

- 密码库的处理移动至 `decompress.rs`

- 格式有关的内容移动至 `format` 模块

## [1.0.1] - 2024-11-11

### Added

- 在桌面通知和日志中显示版本号

## [1.0.0] - 2024-11-11

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security
