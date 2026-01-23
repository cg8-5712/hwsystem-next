//! 前端静态资源路由
//!
//! 使用 rust-embed 嵌入前端构建产物，支持：
//! - SPA fallback（未找到的路由返回 index.html）
//! - 自定义前端目录覆盖（开发用）
//! - PWA 资源服务
//! - %BASE_PATH% 占位符替换

use actix_web::{HttpRequest, HttpResponse, Result as ActixResult, web};
use rust_embed::Embed;
use std::path::Path;

use crate::config::AppConfig;

/// 嵌入前端静态资源
/// 编译时从 frontend/dist/ 目录读取文件
#[derive(Embed)]
#[folder = "frontend/dist/"]
struct FrontendAssets;

/// 获取文件的 MIME 类型
fn get_mime_type(path: &str) -> &'static str {
    let ext = Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    match ext {
        "html" => "text/html; charset=utf-8",
        "js" => "application/javascript; charset=utf-8",
        "mjs" => "application/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ttf" => "font/ttf",
        "eot" => "application/vnd.ms-fontobject",
        "webp" => "image/webp",
        "webm" => "video/webm",
        "mp4" => "video/mp4",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "pdf" => "application/pdf",
        "xml" => "application/xml",
        "txt" => "text/plain; charset=utf-8",
        "wasm" => "application/wasm",
        "map" => "application/json",
        _ => "application/octet-stream",
    }
}

/// 检查是否应该设置缓存
fn should_cache(path: &str) -> bool {
    let ext = Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    // 静态资源（带 hash 的）可以长期缓存
    matches!(
        ext,
        "js" | "css"
            | "woff"
            | "woff2"
            | "ttf"
            | "eot"
            | "png"
            | "jpg"
            | "jpeg"
            | "gif"
            | "svg"
            | "webp"
    )
}

/// 处理 HTML 文件中的占位符替换
fn process_html(content: &[u8], _config: &AppConfig) -> Vec<u8> {
    let html = String::from_utf8_lossy(content);
    // 如果未来需要支持 base_path，可以在这里添加配置项
    // 目前直接返回原始内容
    let processed = html.replace("%BASE_PATH%", "");
    processed.into_bytes()
}

/// 尝试从自定义目录读取文件（开发用）
fn try_custom_file(path: &str) -> Option<Vec<u8>> {
    let custom_path = format!("./frontend-custom/{}", path);
    std::fs::read(&custom_path).ok()
}

/// 尝试从嵌入的资源中获取文件
fn get_embedded_file(path: &str) -> Option<Vec<u8>> {
    FrontendAssets::get(path).map(|f| f.data.to_vec())
}

/// 获取文件内容（优先自定义目录，然后嵌入资源）
fn get_file(path: &str) -> Option<Vec<u8>> {
    try_custom_file(path).or_else(|| get_embedded_file(path))
}

/// 前端资源请求处理
pub async fn serve_frontend(req: HttpRequest) -> ActixResult<HttpResponse> {
    let path = req.match_info().query("tail").trim_start_matches('/');
    let config = AppConfig::get();

    // 尝试获取请求的文件
    let (content, file_path) = if path.is_empty() || path == "/" {
        // 根路径返回 index.html
        (get_file("index.html"), "index.html")
    } else if let Some(content) = get_file(path) {
        // 找到请求的文件
        (Some(content), path)
    } else {
        // SPA fallback: 未找到的路由返回 index.html
        (get_file("index.html"), "index.html")
    };

    match content {
        Some(mut data) => {
            let mime = get_mime_type(file_path);

            // 处理 HTML 文件中的占位符
            if mime.starts_with("text/html") {
                data = process_html(&data, config);
            }

            let mut response = HttpResponse::Ok();
            response.content_type(mime);

            // 设置缓存头
            if should_cache(file_path) {
                response.insert_header(("Cache-Control", "public, max-age=31536000, immutable"));
            } else {
                response.insert_header(("Cache-Control", "no-cache, no-store, must-revalidate"));
            }

            Ok(response.body(data))
        }
        None => {
            // 如果连 index.html 都没有，返回一个简单的提示
            Ok(HttpResponse::NotFound()
                .content_type("text/html; charset=utf-8")
                .body(
                    r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>HWSystem</title>
</head>
<body>
    <h1>Frontend Not Found</h1>
    <p>The frontend assets have not been built or embedded.</p>
    <p>Please build the frontend first:</p>
    <pre>cd frontend && npm run build</pre>
</body>
</html>"#,
                ))
        }
    }
}

/// 配置前端路由
pub fn configure_frontend_routes(cfg: &mut web::ServiceConfig) {
    // 所有非 API 路由都交给前端处理
    cfg.route("/{tail:.*}", web::get().to(serve_frontend));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mime_type() {
        assert_eq!(get_mime_type("index.html"), "text/html; charset=utf-8");
        assert_eq!(
            get_mime_type("app.js"),
            "application/javascript; charset=utf-8"
        );
        assert_eq!(get_mime_type("style.css"), "text/css; charset=utf-8");
        assert_eq!(get_mime_type("image.png"), "image/png");
        assert_eq!(get_mime_type("unknown.xyz"), "application/octet-stream");
    }

    #[test]
    fn test_should_cache() {
        assert!(should_cache("app.js"));
        assert!(should_cache("style.css"));
        assert!(should_cache("logo.png"));
        assert!(!should_cache("index.html"));
        assert!(!should_cache("manifest.json"));
    }
}
