//! 提交概览服务（按学生聚合）

use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use crate::middlewares::RequireJWT;
use crate::models::users::entities::UserRole;
use crate::models::{ApiResponse, ErrorCode};
use crate::services::submissions::SubmissionService;

/// 获取作业提交概览（按学生聚合）
///
/// 权限：教师和管理员
pub async fn get_submission_summary(
    service: &SubmissionService,
    request: &HttpRequest,
    homework_id: i64,
    page: Option<i64>,
    size: Option<i64>,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(request);

    // 获取当前用户信息
    let current_user = match RequireJWT::extract_user_claims(request) {
        Some(user) => user,
        None => {
            return Ok(HttpResponse::Unauthorized()
                .json(ApiResponse::error_empty(ErrorCode::Unauthorized, "未登录")));
        }
    };

    // 验证作业存在
    let homework = storage
        .get_homework_by_id(homework_id)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("查询作业失败: {e}")))?;

    let homework = match homework {
        Some(hw) => hw,
        None => {
            return Ok(
                HttpResponse::NotFound().json(ApiResponse::<()>::error_empty(
                    ErrorCode::NotFound,
                    "作业不存在",
                )),
            );
        }
    };

    // 权限检查：仅教师和管理员可访问
    match current_user.role {
        UserRole::Admin => {
            // 管理员可以访问任何作业
        }
        UserRole::Teacher => {
            // 教师只能访问自己班级的作业
            let has_access =
                check_teacher_class_access(&storage, current_user.id, homework.class_id).await;
            if !has_access {
                return Ok(
                    HttpResponse::Forbidden().json(ApiResponse::<()>::error_empty(
                        ErrorCode::Forbidden,
                        "您无权查看此作业的提交概览",
                    )),
                );
            }
        }
        UserRole::User => {
            return Ok(
                HttpResponse::Forbidden().json(ApiResponse::<()>::error_empty(
                    ErrorCode::Forbidden,
                    "学生无权查看提交概览",
                )),
            );
        }
    }

    // 获取提交概览
    let page = page.unwrap_or(1);
    let size = size.unwrap_or(20);

    let summary = storage
        .get_submission_summary(homework_id, page, size)
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("查询提交概览失败: {e}"))
        })?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(summary, "查询成功")))
}

/// 获取某学生某作业的所有提交版本（教师视角）
///
/// 权限：教师和管理员
pub async fn list_user_submissions_for_teacher(
    service: &SubmissionService,
    request: &HttpRequest,
    homework_id: i64,
    user_id: i64,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(request);

    // 获取当前用户信息
    let current_user = match RequireJWT::extract_user_claims(request) {
        Some(user) => user,
        None => {
            return Ok(HttpResponse::Unauthorized()
                .json(ApiResponse::error_empty(ErrorCode::Unauthorized, "未登录")));
        }
    };

    // 验证作业存在
    let homework = storage
        .get_homework_by_id(homework_id)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("查询作业失败: {e}")))?;

    let homework = match homework {
        Some(hw) => hw,
        None => {
            return Ok(
                HttpResponse::NotFound().json(ApiResponse::<()>::error_empty(
                    ErrorCode::NotFound,
                    "作业不存在",
                )),
            );
        }
    };

    // 权限检查：仅教师和管理员可访问
    match current_user.role {
        UserRole::Admin => {
            // 管理员可以访问任何作业
        }
        UserRole::Teacher => {
            // 教师只能访问自己班级的作业
            let has_access =
                check_teacher_class_access(&storage, current_user.id, homework.class_id).await;
            if !has_access {
                return Ok(
                    HttpResponse::Forbidden().json(ApiResponse::<()>::error_empty(
                        ErrorCode::Forbidden,
                        "您无权查看此学生的提交历史",
                    )),
                );
            }
        }
        UserRole::User => {
            return Ok(
                HttpResponse::Forbidden().json(ApiResponse::<()>::error_empty(
                    ErrorCode::Forbidden,
                    "学生无权查看其他学生的提交历史",
                )),
            );
        }
    }

    // 获取学生提交历史
    let submissions = storage
        .list_user_submissions_for_teacher(homework_id, user_id)
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("查询提交历史失败: {e}"))
        })?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(
        crate::models::submissions::responses::UserSubmissionHistoryResponse { items: submissions },
        "查询成功",
    )))
}

/// 检查教师是否有班级访问权限
async fn check_teacher_class_access(
    storage: &std::sync::Arc<dyn crate::storage::Storage>,
    teacher_id: i64,
    class_id: i64,
) -> bool {
    // 先检查是否是班级成员
    if let Ok(Some(_)) = storage
        .get_class_user_by_user_id_and_class_id(teacher_id, class_id)
        .await
    {
        return true;
    }

    // 再检查是否是班级创建者（teacher_id）
    if let Ok(Some(class)) = storage.get_class_by_id(class_id).await
        && class.teacher_id == teacher_id
    {
        return true;
    }

    false
}
