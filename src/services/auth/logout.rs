use actix_web::{HttpResponse, Result as ActixResult};

use crate::models::ApiResponse;
use crate::utils::jwt::JwtUtils;

/// 处理用户登出
/// 通过设置空的 refresh_token cookie 来清除客户端的登录状态
pub async fn handle_logout() -> ActixResult<HttpResponse> {
    // 创建空的 refresh_token cookie（max_age=0 会让浏览器删除该 cookie）
    let empty_cookie = JwtUtils::create_empty_refresh_token_cookie();

    Ok(HttpResponse::Ok()
        .cookie(empty_cookie)
        .json(ApiResponse::<()>::success_empty("登出成功")))
}
