use serde::Deserialize;
use ts_rs::TS;

// 用户登录请求（来自HTTP请求）
#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "../frontend/src/types/generated/auth.ts")]
pub struct LoginRequest {
    /// 用户名或邮箱
    pub username: String,
    /// 密码
    pub password: String,
    /// 是否记住我
    #[serde(default)]
    pub remember_me: bool,
}
