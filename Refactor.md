# rust-hwsystem-next 重构计划

基于 shortlinker 项目的架构模式，针对 hwsystem 的特殊需求进行选择性重构。

## 业务差异分析

| 维度 | shortlinker | hwsystem |
|------|-------------|----------|
| **核心业务** | URL 短链接（读多写少） | 教育管理平台（CRUD 复杂） |
| **实体数量** | 2-3 个 | 5+ 个（User/Class/ClassUser/Homework/File） |
| **权限模型** | 单角色（Admin/User） | **双层角色**（全局 + 班级） |
| **业务流程** | 创建 → 重定向 | 创建 → 加入 → 提交 → 批改 → 统计 |
| **文件处理** | 无 | Token 化存储 + 多文件上传 |

### hwsystem 特有需求

1. **双层权限体系**
   - 全局角色：User / Teacher / Admin
   - 班级角色：Student / ClassRepresentative / Teacher
   - `RequireClassRole` 中间件需要从 URL 提取 `class_id` 并查数据库

2. **复杂业务实体关系**
   - User ↔ Class ↔ ClassUser 多对多关联
   - Homework 关联 Class 和 User
   - File 关联 User（作业附件）

3. **文件上传系统**
   - Token 化存储（防路径遍历）
   - 文件类型/大小校验
   - 已有完善实现，无需改动

---

## 重构任务

### ✅ 直接复制：前端二进制嵌入

从 shortlinker 复制前端嵌入方案：
- `rust-embed` 嵌入 `frontend/dist/`
- SPA fallback 支持
- `%BASE_PATH%` 占位符替换

---

### ✅ 适合移植：错误处理宏化

使用 `define_hwsystem_errors!` 宏生成错误枚举：
- 错误代码 (E001-E013)
- `format_colored()` 方法
- `paste` 依赖用于宏标识符转换

---

### ✅ 适合移植：项目结构调整

| 当前 | 目标 | 原因 |
|------|------|------|
| `src/domain/` | `src/services/` | services 更准确描述业务逻辑层 |
| `src/repository/` | `src/storage/` | storage 更通用；统一命名 |
| `src/system/app_config/` | `src/config/` | 配置模块提升到顶层 |
| `src/system/lifetime/` | `src/runtime/lifetime/` | 启动/关闭逻辑属于运行时管理 |

---

### ✅ SQLx → SeaORM 迁移

- 创建 `migration/` 子项目（workspace member）
- 创建 `src/entity/` 目录存放 SeaORM 实体
- 实现 SeaORM Storage 适配层 (`src/storage/sea_orm_storage/`)
- 移除 SQLx 直接依赖和所有相关代码
- 移除旧的插件注册系统（`register.rs`、`macros.rs`、`backends/`）
- `StorageFactory` 简化为 `create_storage()` 函数，直接创建 SeaOrmStorage
- 从 URL 自动推断数据库类型（无需手动配置 `database.type`）
- SQLite 使用特制化连接（WAL 模式 + pragma 优化）

---

### ❌ 不适合移植

1. **三级缓存（Bloom Filter + Negative Cache）** - hwsystem 是 CRUD 场景
2. **IPC 通信（CLI/TUI 模式）** - Web 应用不需要

---

## 实施步骤

### Phase 1：基础结构（不影响功能） ✅

1. [x] 创建 `src/lib.rs`
2. [x] 重命名目录
3. [x] 更新所有 `mod` 声明和 `use` 语句
4. [x] `cargo build` 验证

### Phase 2：错误处理改进 ✅

1. [x] 将 `errors.rs` 改为宏生成
2. [x] 添加错误代码 (E001-E013)
3. [x] 添加 `format_colored()` 方法

### Phase 3：SQLx → SeaORM 迁移 ✅

1. [x] 添加 `migration/` 子项目
2. [x] 创建迁移文件
3. [x] 创建 `src/entity/` 实体定义
4. [x] 实现 SeaORM Storage 适配层
5. [x] 删除旧 SQLx backends 代码
6. [x] 删除插件注册系统
7. [x] 移除 sqlx 直接依赖
8. [x] 清理 models 中的 sqlx 属性

### Phase 4：前端嵌入 ✅

1. [x] 添加 `rust-embed` 依赖
2. [x] 创建 `src/routes/frontend.rs`
3. [x] 配置路由
4. [x] 添加 frontend submodule (`https://github.com/The-ESAP-Project/hwsystem-frontend.git`)

### Phase 5：配置和存储优化 ✅

1. [x] 移除 `database.type` 配置字段，改为从 URL 自动推断数据库类型
2. [x] SQLite 特制化连接（WAL 模式 + pragma 优化）
3. [x] 添加 MySQL 支持（`sqlx-mysql` feature）
4. [x] 清理未使用依赖（移除 7 个：`env`, `sha2`, `serde_with`, `base64`, `futures`, `mime_guess`, `actix-files`）

### Phase 6：ts-rs TypeScript 类型自动生成 ✅

1. [x] 添加 `ts-rs` 依赖（features: `chrono-impl`, `serde-json-impl`）
2. [x] 为所有 API 模型添加 `#[derive(TS)]` 和 `#[ts(export)]`
3. [x] 创建 `frontend/src/types/generated/` 目录
4. [x] 运行 `cargo test` 生成 TypeScript 类型文件
5. [x] 验证前端 TypeScript 编译通过

**修改的 Rust 模块：**
- `src/models/common/` - ApiResponse, PaginationQuery, PaginationInfo, PaginatedResponse
- `src/models/users/` - User, UserRole, UserStatus, UserProfile, 请求/响应类型
- `src/models/auth/` - LoginRequest, LoginResponse, RefreshTokenResponse 等
- `src/models/classes/` - Class, CreateClassRequest, ClassListResponse 等
- `src/models/class_users/` - ClassUser, ClassUserRole, JoinClassRequest 等
- `src/models/homeworks/` - Homework, HomeworkListQuery, HomeworkResponse 等
- `src/models/files/` - File, FileUploadResponse
- `src/models/system/` - SystemSettingsResponse

**生成的 TypeScript 文件：**
```
frontend/src/types/generated/
├── index.ts      # 统一导出
├── api.ts        # ApiResponse<T>
├── pagination.ts # PaginationQuery, PaginationInfo
├── user.ts       # User, UserRole, UserStatus 等
├── auth.ts       # LoginRequest, LoginResponse 等
├── class.ts      # Class, CreateClassRequest 等
├── class-user.ts # ClassUser, ClassUserRole 等
├── homework.ts   # Homework, HomeworkResponse 等
├── file.ts       # File, FileUploadResponse
└── system.ts     # SystemSettingsResponse
```

**使用方式：**
```bash
# 重新生成类型
cargo test --lib export_bindings
```

```typescript
// 前端导入
import { User, UserRole, LoginRequest, ApiResponse } from '@/types/generated'
```

### Phase 7：待完成

1. [ ] 中间件缓存优化（减少 `RequireClassRole` 的 DB 查询）
2. [ ] 动态配置（按需）

---

## 最终目录结构

```
src/
├── lib.rs
├── main.rs
├── cache/                    # 缓存层（Moka/Redis）
├── config/                   # 配置管理
├── entity/                   # SeaORM 实体定义
│   ├── mod.rs
│   ├── prelude.rs
│   ├── users.rs
│   ├── classes.rs
│   ├── class_users.rs
│   ├── homeworks.rs
│   ├── submissions.rs
│   ├── grades.rs
│   └── files.rs
├── errors.rs                 # 统一错误处理（宏生成）
├── middlewares/              # 认证授权中间件
├── models/                   # 业务数据模型（带 ts-rs 导出）
├── routes/                   # API 路由层
│   └── frontend.rs           # 前端静态资源
├── runtime/lifetime/         # 运行时生命周期
├── services/                 # 业务逻辑层
├── storage/
│   ├── mod.rs                # Storage trait + create_storage()
│   └── sea_orm_storage/      # SeaORM 实现
│       ├── mod.rs
│       ├── users.rs
│       ├── classes.rs
│       ├── class_users.rs
│       ├── homeworks.rs
│       └── files.rs
└── utils/

migration/                    # SeaORM 迁移
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── main.rs
    └── m20250123_000001_create_tables.rs

frontend/src/types/
├── generated/                # ts-rs 自动生成（勿手工编辑）
│   ├── index.ts
│   ├── api.ts
│   ├── pagination.ts
│   ├── user.ts
│   ├── auth.ts
│   ├── class.ts
│   ├── class-user.ts
│   ├── homework.ts
│   ├── file.ts
│   └── system.ts
└── manual/                   # 手工维护（仅前端特有类型）
    ├── router.ts
    └── notification.ts
```

---

## 验证结果

- [x] `cargo build` 零错误
- [x] `cargo clippy` 零警告
- [x] 移除了所有 SQLx 直接依赖
- [x] 移除了插件注册系统（ctor 仍用于 cache 模块）
- [x] 存储层直接使用 SeaOrmStorage，无需配置 backend
- [x] 数据库类型自动推断（支持 SQLite/PostgreSQL/MySQL）
- [x] SQLite 使用 WAL 模式和性能优化（64MB cache, 512MB mmap）
- [x] 清理了 7 个未使用依赖，减少编译时间和二进制体积
- [x] ts-rs 类型导出：39 个导出测试全部通过
- [x] 前端 TypeScript 编译通过（`npx tsc --noEmit`）

---

## 总结

### 移植的
- ✅ 前端二进制嵌入（rust-embed）
- ✅ 错误处理宏化
- ✅ 项目结构（lib.rs、目录命名）
- ✅ SQLx → SeaORM 完全迁移
- ✅ 数据库 URL 自动推断（参考 shortlinker）
- ✅ SQLite 特制化连接（WAL + pragma 优化）
- ✅ ts-rs TypeScript 类型自动生成（前后端类型同步）

### 保留的
- ✅ 双层权限中间件（hwsystem 特有）
- ✅ 单层缓存（简化）
- ✅ 文件上传系统

### 不移植的
- ❌ 三级缓存
- ❌ IPC/CLI/TUI
- ❌ 动态配置（暂时）
