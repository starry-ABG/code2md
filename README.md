# code2md

A simple CLI tool that recursively collects files (excluding hidden & certain folders/files, skipping binary files) and outputs them into a single Markdown file.

(中文版请向下滚动查看)

## Features
	•	Ignore hidden files/folders (names starting with .)
	•	Skip certain build/output directories by default (e.g. node_modules, target, dist, build)
	•	Skip common lock files (e.g. package-lock.json, pnpm-lock.yaml, yarn.lock)
	•	Automatically detect and skip binary files (contains \0 bytes)
	•	Output each file’s content in a code block, with syntax highlighting based on the file extension (Rust, JS, TS, etc.)

## Installation
	1.	Ensure you have Rust installed.
	2.	Clone this repository or download the source code.
	3.	In the project root, run:

cargo install --path .

This installs the code2md binary into your local Cargo bin (usually ~/.cargo/bin).

## Usage

From your terminal:

code2md <directory> [output_file]

	•	<directory>: The path you want to scan (e.g. . for current directory).
	•	[output_file]: (Optional) The Markdown file to create/overwrite. Default is all_files.md.

## Example

code2md . my_project_files.md

After running, you’ll get a my_project_files.md containing all relevant text-based files from the current directory (and subdirectories), organized by file path and enclosed in appropriate code fences.

## License

Licensed under the MIT License.

## 中文说明

这是一个简单的命令行工具，可递归遍历指定目录，把所有（排除隐藏文件、部分编译/中间产物文件、常见锁文件、并且自动跳过二进制文件）的文本文件内容，合并输出到一个 Markdown 文件中。

## 特性
	•	忽略隐藏文件/文件夹（名称以 . 开头）
	•	跳过常见的构建/输出目录（如 node_modules, target, dist, build 等）
	•	跳过常见的锁文件（如 package-lock.json, pnpm-lock.yaml, yarn.lock）
	•	自动识别并跳过二进制文件（检测到 \0 字节）
	•	将文本文件内容按 Markdown 代码块输出，并根据扩展名作简单的语法高亮（Rust, JS, TS 等）

## 安装
	1.	确保已安装 Rust。
	2.	下载或克隆本项目到本地。
	3.	在项目根目录下执行：

cargo install --path .

这样会将可执行文件 code2md 安装到本地 Cargo bin 目录（通常是 ~/.cargo/bin）。

## 使用方法

在终端中执行：

code2md <目录路径> [输出文件]

	•	<目录路径>：要扫描的目标路径，例如当前目录可使用 .。
	•	[输出文件]：可选参数，默认为 all_files.md。如果指定了文件名，会将结果写到这个文件中。

示例

code2md . my_project_files.md

运行结束后，会在当前目录下生成一个 my_project_files.md 文件，里面包含扫描到的所有文本文件，按照文件路径顺序排版，并使用合适的代码块标记高亮。

## 许可证

使用 MIT License 许可证开源。根据许可证，你可以自由使用、复制、修改和分发本项目的代码，但需保留许可证信息。