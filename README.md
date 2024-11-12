# eye³

睁开第三只眼 👁 轻取[隐写](https://github.com/cenglin123/SteganographierGUI)文件

## ⭐ Features

- 开箱即用，无多余操作
- 无感运行，完成后显示桌面通知
- 跨平台，支持 x86_64 架构 Windows 和 Linux

## 💡 Usage

完整组件包括：

1. 可执行文件 `eye3rd.exe`（Linux 上为 `eye3rd`，暂不支持 macOS）
2. 密码库文件 `eye3rd.db.txt`，未指定路径时将依次在程序目录、程序的上上级目录和用户家目录下寻找
3. 日志文件保存在程序目录下的 `eye3rd.log`（会自动创建）

### 解手模式

右键点击待处理的文件，选择用本程序打开即可

**Notice**: 解手模式需要使用密码库：

- 每行表示一个密码条目
- 一行由 `频率`、`分隔符` 和 `密码` 三部分组成
  1. `频率` 为该密码被使用的次数，由程序自动统计并排序
  2. `分隔符` 为**英文逗号**
  3. `密码` 为一串字符

密码库示例如下：

```txt
113,Ao82s9jNk
72,6$hu!,4
0,i5l.6?rt07
```

若要给密码库添加新密码，只需在末尾添加一行，注意 `频率` 应该为 0

### 终端模式

由于我将 Windows 平台的模式设为了桌面程序（不会弹出终端窗口），导致其在终端不会有输出，包括 `--help` 和 `--version`，但程序可以接受参数并正确运行，参数如下：

```pwsh
Usage: eye3rd.exe [OPTIONS] <FILE>

Arguments:
  <FILE>  指定输入文件路径

Options:
  -p, --pw <PASSWORD>  指定密码
  -d, --db <FILE>      指定密码库路径
  -h, --help           Print help
  -V, --version        Print version
```

## 📝 Todo

- [ ] 目前仅支持处理 mp4 文件，暂未测试 mkv 文件
- [ ] 使用的 [zip](https://github.com/zip-rs/zip2) 库在通过 `ZipArchive::new()` 读入待处理文件时花费了较长时间（且偶发性剧增），需要优化
- [ ] 实际上也支持普通的未加密或使用 AES-256 加密的 zip 文件，但仍不支持 7z、rar 等压缩格式

## ❤️ Thanks

本程序仅出于自用方便考虑，感谢隐写者的作者为探索可行的网盘保存方式所做出的大量实践和考证
