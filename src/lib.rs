//! HWSystem - 教育管理平台后端服务
//!
//! 基于 Actix Web 构建的高性能作业管理系统后端。
//!
//! # 架构
//! - `cache`: 缓存层（Moka/Redis）
//! - `config`: 配置管理
//! - `entity`: SeaORM 数据库实体
//! - `errors`: 统一错误处理
//! - `middlewares`: 认证授权中间件
//! - `models`: 数据模型定义
//! - `routes`: API 路由层
//! - `runtime`: 运行时生命周期管理
//! - `services`: 业务逻辑层
//! - `storage`: 数据存储层（SeaORM）
//! - `utils`: 工具函数

pub mod cache;
pub mod config;
pub mod entity;
pub mod errors;
pub mod middlewares;
pub mod models;
pub mod routes;
pub mod runtime;
pub mod services;
pub mod storage;
pub mod utils;
