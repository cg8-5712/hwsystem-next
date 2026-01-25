use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=frontend/dist");

    // 获取项目根目录
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dist_path = Path::new(&manifest_dir).join("frontend/dist");

    if !dist_path.exists() {
        eprintln!("Warning: frontend/dist directory not found!");
        eprintln!("Please build the frontend first:");
        eprintln!("  cd frontend && bun install && bun run build");

        create_fallback_files(&dist_path);
    }
}

fn create_fallback_files(dist_path: &Path) {
    fs::create_dir_all(dist_path).expect("Failed to create dist directory");

    let fallback_html = r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>作业管理系统 - 前端未构建</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            max-width: 600px;
            margin: 100px auto;
            padding: 20px;
            text-align: center;
        }
        .warning {
            background: #fff3cd;
            border: 1px solid #ffeaa7;
            padding: 20px;
            border-radius: 8px;
            margin: 20px 0;
        }
        code {
            background: #f1f3f4;
            padding: 2px 6px;
            border-radius: 4px;
            font-family: monospace;
        }
    </style>
</head>
<body>
    <h1>作业管理系统</h1>
    <div class="warning">
        <h2>前端未构建</h2>
        <p>需要先构建前端才能运行服务器。</p>
        <p>请执行以下命令：</p>
        <p><code>cd frontend && bun install && bun run build</code></p>
    </div>
</body>
</html>"#;

    fs::write(dist_path.join("index.html"), fallback_html)
        .expect("Failed to write fallback index.html");

    // 创建空的 favicon
    fs::write(dist_path.join("favicon.ico"), []).expect("Failed to write fallback favicon");

    fs::create_dir_all(dist_path.join("assets")).expect("Failed to create assets directory");
}
