use std::env;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    // 从命令行获取目标目录和输出文件路径
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("用法: {} <目录路径> [输出文件路径]", args[0]);
        std::process::exit(1);
    }
    let target_dir = &args[1];
    let output_path = if args.len() >= 3 {
        &args[2]
    } else {
        "all_files.md"
    };

    // 1. 先把输出文件的绝对路径记录下来，供后面忽略用
    let output_abs = fs::canonicalize(output_path)
        .unwrap_or_else(|_| PathBuf::from(output_path));

    // 2. 收集所有需要的文件
    let all_files = collect_all_files(Path::new(target_dir), &output_abs)?;

    // 3. 打开输出文件
    //    注意，这里用 create(...) 会直接覆盖/生成 output_path
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    // 4. 依次输出到 Markdown
    for file_path in &all_files {
        // 以字节形式读取文件
        let bytes = fs::read(file_path)?;

        // 如果可能是二进制文件，则跳过
        if is_likely_binary(&bytes) {
            eprintln!("跳过二进制文件: {}", file_path.display());
            continue;
        }

        // 否则将其视为文本，用 lossless 转换（非法字符会用 '�' 替代）
        let content = String::from_utf8_lossy(&bytes);

        // 相对于输入目录的相对路径（用于在 Markdown 中显示）
        let rel_path = file_path.strip_prefix(target_dir).unwrap_or(file_path);

        // 根据扩展名猜测语言高亮
        let code_block_lang = match file_path.extension().and_then(|ext| ext.to_str()) {
            Some("rs") => "rust",
            Some("js") => "javascript",
            Some("ts") => "typescript",
            Some("py") => "python",
            Some("java") => "java",
            Some("cpp") | Some("cc") => "cpp",
            Some("c") => "c",
            Some("html") => "html",
            Some("css") => "css",
            Some("json") => "json",
            Some("toml") => "toml",
            Some("yaml") | Some("yml") => "yaml",
            // 更多扩展名 => 语言高亮，可按需自行添加
            _ => "text",
        };

        // 写入 Markdown 标题
        writeln!(writer, "# 文件路径: {}", rel_path.display())?;
        // 写入代码块标记
        writeln!(writer, "```{}", code_block_lang)?;

        // 写入文件内容
        writer.write_all(content.as_bytes())?;

        // 结束代码块
        writeln!(writer, "\n```\n")?;
    }

    println!("所有文件已写入到: {}", output_path);
    Ok(())
}

/// 递归收集指定目录下的所有文件
/// - 忽略隐藏文件/文件夹
/// - 忽略 node_modules, target, dist, build 等目录
/// - 忽略 package-lock.json, pnpm-lock.yaml, yarn.lock 等文件
/// - 忽略当前要写出的那个 output_abs (防止读到自己)
fn collect_all_files(dir: &Path, output_abs: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut result = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        // 如果需要忽略此路径，则 continue
        if should_ignore(path, dir, output_abs) {
            continue;
        }

        // 如果是文件，则加入结果列表
        if path.is_file() {
            result.push(path.to_path_buf());
        }
    }
    // 按路径排序，方便输出
    result.sort();
    Ok(result)
}

/// 判断给定路径是否应该被忽略
fn should_ignore(path: &Path, root_dir: &Path, output_abs: &PathBuf) -> bool {
    // 如果就是我们要输出的那个文件，忽略
    if let Ok(canon) = path.canonicalize() {
        if canon == *output_abs {
            return true;
        }
    }

    if is_hidden(path, root_dir) {
        return true;
    }
    if is_ignored_dir(path, root_dir) {
        return true;
    }
    if is_ignored_file(path) {
        return true;
    }
    false
}

/// 判断是否为隐藏文件/文件夹：
/// 任意一级目录（相对于 root_dir）或文件名以 '.' 开头，则视为隐藏
fn is_hidden(path: &Path, root_dir: &Path) -> bool {
    if let Ok(subpath) = path.strip_prefix(root_dir) {
        for component in subpath.components() {
            if let Some(os_str) = component.as_os_str().to_str() {
                if os_str.starts_with('.') {
                    return true;
                }
            }
        }
    }
    false
}

/// 判断是否是要忽略的特定目录
fn is_ignored_dir(path: &Path, root_dir: &Path) -> bool {
    if let Ok(subpath) = path.strip_prefix(root_dir) {
        for component in subpath.components() {
            if let Some(name) = component.as_os_str().to_str() {
                // 可在此处自由扩展要忽略的目录
                if ["node_modules", "target", "dist", "build"].contains(&name) {
                    return true;
                }
            }
        }
    }
    false
}

/// 判断是否是要忽略的特定文件
fn is_ignored_file(path: &Path) -> bool {
    if let Some(fname) = path.file_name().and_then(|s| s.to_str()) {
        // 可在此处扩展更多文件名
        if ["package-lock.json", "pnpm-lock.yaml", "yarn.lock", "Cargo.lock"].contains(&fname) {
            return true;
        }
    }
    false
}

/// 判断内容是否“可能是”二进制文件 (仅判断是否含 '\0' 字节)
fn is_likely_binary(buf: &[u8]) -> bool {
    buf.contains(&0)
}