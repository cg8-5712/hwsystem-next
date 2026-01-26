use actix_web::{HttpRequest, HttpResponse, Result as ActixResult, middleware, web};
use once_cell::sync::Lazy;

use crate::middlewares;
use crate::models::users::entities::UserRole;
use crate::services::SystemService;
use crate::services::system::settings;

// 懒加载的全局 SystemService 实例
static SYSTEM_SERVICE: Lazy<SystemService> = Lazy::new(SystemService::new_lazy);

pub async fn get_settings(request: HttpRequest) -> ActixResult<HttpResponse> {
    SYSTEM_SERVICE.get_settings(&request).await
}

// 配置路由
pub fn configure_system_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/system")
            .wrap(middleware::Compress::default())
            .wrap(middlewares::RequireJWT)
            // 公开设置（只读，登录用户可访问）
            .route("/settings", web::get().to(get_settings))
            // 管理员设置路由
            .service(
                web::scope("/admin/settings")
                    .wrap(middlewares::RequireRole::new_any(UserRole::admin_roles()))
                    .route("", web::get().to(settings::get_admin_settings))
                    .route("/{key}", web::put().to(settings::update_setting))
                    .route("/audit", web::get().to(settings::get_setting_audits)),
            ),
    );
}
