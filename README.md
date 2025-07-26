# rspfind

**[English](#english) | [中文](#中文)**

---

## English

### Overview
`rspfind` is a high-performance command-line tool written in Rust for searching content within files. It supports both single file and directory searches with parallel processing capabilities for improved performance.

### Features
- 🔍 **Fast Search**: Utilizes parallel processing with Rayon for efficient file searching
- 📁 **Flexible Input**: Search in individual files or entire directories
- 🔤 **Case Insensitive**: Optional case-insensitive search mode
- 🎯 **Precise Results**: Shows exact line numbers and highlighted matches
- 📊 **Progress Tracking**: Real-time progress indicators for large directory searches
- 💾 **Output Options**: Save results to file or display in terminal
- 🌈 **Colorized Output**: Syntax highlighting for better readability

### Installation

#### Prerequisites
- Rust 1.70 or higher
- Cargo

#### Build from Source
```bash
git clone <repository-url>
cd rspfind
cargo build --release
```

The binary will be available at `target/release/rspfind`

### Usage

#### Basic Search
```bash
# Search in a single file
rspfind find --query "search_term" --file-path "path/to/file.txt"

# Search in multiple files
rspfind find --query "pattern" --file-path "file1.txt" "file2.txt" "file3.txt"

# Search in a directory
rspfind find --query "keyword" --dir "path/to/directory"
```

#### Advanced Options
```bash
# Case-insensitive search
rspfind find --query "Hello" --dir "./src" --ignore-case

# Save results to file
rspfind find --query "TODO" --dir "./src" --output "./results"

# Combine options
rspfind find --query "error" --file-path "*.log" --ignore-case --output "./reports"
```

#### Command Structure
```
rspfind <COMMAND> [OPTIONS]

Commands:
  find    Search for content in files
  diff    Compare two files (coming soon)

Options:
  -q, --query <QUERY>        Search query string
  -f, --file-path <PATH>     Specific file(s) to search
  -d, --dir <DIR>            Directory to search
  -i, --ignore-case          Case-insensitive search
  -o, --output <DIR>         Output directory for results
  -h, --help                 Print help information
  -V, --version              Print version information
```

### Examples

#### Example 1: Search for a function name
```bash
rspfind find --query "calculate_total" --dir "./src"
```

#### Example 2: Find all TODO comments
```bash
rspfind find --query "TODO" --dir "./" --ignore-case
```

#### Example 3: Search in specific files
```bash
rspfind find --query "config" --file-path "app.js" "config.json" "settings.yaml"
```

### Output Format
Results are displayed with:
- **File name** (highlighted in yellow)
- **Line number** where match was found
- **Full line content** with the search term highlighted

Example output:
```
在文件 'main.rs' 中找到以下匹配: 
Line 45:     let result = search_query("example", &data);
Line 127:     if search_query(&input, &database) {
```

### Performance
- **Parallel Processing**: Uses Rayon for multi-threaded file processing
- **Memory Efficient**: Streams file content to minimize memory usage
- **Progress Indicators**: Shows search progress for large directories

### Dependencies
- `clap` - Command line argument parsing
- `rayon` - Data parallelism
- `indicatif` - Progress bars
- `owo-colors` - Terminal colors
- `walkdir` - Directory traversal
- `dashmap` - Concurrent HashMap
- `anyhow` - Error handling

---

## 中文

### 项目简介
`rspfind` 是一个用 Rust 编写的高性能命令行文件内容搜索工具。支持单文件和目录搜索，具备并行处理能力以提升性能。

### 主要特性
- 🔍 **快速搜索**: 使用 Rayon 实现并行处理，高效搜索文件内容
- 📁 **灵活输入**: 支持单个文件或整个目录搜索
- 🔤 **忽略大小写**: 可选的忽略大小写搜索模式
- 🎯 **精确结果**: 显示准确的行号和高亮匹配内容
- 📊 **进度跟踪**: 大目录搜索时显示实时进度指示器
- 💾 **输出选项**: 可将结果保存到文件或在终端显示
- 🌈 **彩色输出**: 语法高亮，提升可读性

### 安装方法

#### 前置要求
- Rust 1.70 或更高版本
- Cargo

#### 从源码构建
```bash
git clone <repository-url>
cd rspfind
cargo build --release
```

编译后的二进制文件位于 `target/release/rspfind`

### 使用方法

#### 基础搜索
```bash
# 在单个文件中搜索
rspfind find --query "搜索词" --file-path "路径/文件.txt"

# 在多个文件中搜索
rspfind find --query "模式" --file-path "文件1.txt" "文件2.txt" "文件3.txt"

# 在目录中搜索
rspfind find --query "关键词" --dir "路径/目录"
```

#### 高级选项
```bash
# 忽略大小写搜索
rspfind find --query "Hello" --dir "./src" --ignore-case

# 将结果保存到文件
rspfind find --query "TODO" --dir "./src" --output "./results"

# 组合使用选项
rspfind find --query "错误" --file-path "*.log" --ignore-case --output "./reports"
```

#### 命令结构
```
rspfind <命令> [选项]

命令:
  find    在文件中搜索内容
  diff    比较两个文件（即将推出）

选项:
  -q, --query <查询>        搜索查询字符串
  -f, --file-path <路径>     要搜索的特定文件
  -d, --dir <目录>          要搜索的目录
  -i, --ignore-case         忽略大小写搜索
  -o, --output <目录>        结果输出目录
  -h, --help                打印帮助信息
  -V, --version             打印版本信息
```

### 使用示例

#### 示例1：搜索函数名
```bash
rspfind find --query "calculate_total" --dir "./src"
```

#### 示例2：查找所有TODO注释
```bash
rspfind find --query "TODO" --dir "./" --ignore-case
```

#### 示例3：在特定文件中搜索
```bash
rspfind find --query "配置" --file-path "app.js" "config.json" "settings.yaml"
```

### 输出格式
结果显示包含：
- **文件名**（黄色高亮显示）
- **匹配行号**
- **完整行内容**，搜索词高亮显示

输出示例：
```
在文件 'main.rs' 中找到以下匹配: 
Line 45:     let result = search_query("example", &data);
Line 127:     if search_query(&input, &database) {
```

### 性能特点
- **并行处理**: 使用 Rayon 实现多线程文件处理
- **内存高效**: 流式处理文件内容，最小化内存使用
- **进度指示**: 大目录搜索时显示搜索进度

### 依赖库
- `clap` - 命令行参数解析
- `rayon` - 数据并行处理
- `indicatif` - 进度条显示
- `owo-colors` - 终端颜色
- `walkdir` - 目录遍历
- `dashmap` - 并发 HashMap
- `anyhow` - 错误处理

### 许可证
MIT License - 详见 LICENSE 文件

### 贡献
欢迎提交 Issue 和 Pull Request！
