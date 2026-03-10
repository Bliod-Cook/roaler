# Roaler

Roaler 是一个开源、自部署、Web 优先的信息流平台，目标是提供类似 Folo 的核心阅读体验：统一时间线、订阅源管理、全文阅读、搜索、OPML、AI 摘要与翻译，以及 RSSHub 接入。

## 技术栈
- Rust workspace：`roaler-domain`、`roaler-api`、`roaler-worker`
- React + TypeScript：`apps/web`
- PostgreSQL + Redis + 本地文件存储
- Docker Compose + Caddy

## 当前仓库结构
- `crates/roaler-domain`：领域模型、数据库迁移、服务逻辑
- `crates/roaler-api`：Axum REST API 与 OpenAPI
- `crates/roaler-worker`：抓取、全文抽取、AI 任务、WebSub 协调
- `apps/web`：管理与阅读 Web 界面
- `deployment`：容器与反向代理配置

## 本地开发
1. 复制环境变量：`cp .env.example .env`
2. 启动基础依赖：`docker compose up -d postgres redis`
3. 启动 API：`cargo run -p roaler-api`
4. 启动 Worker：`cargo run -p roaler-worker`
5. 启动 Web：`pnpm install && pnpm dev:web`

## 自部署
1. 复制环境变量：`cp .env.example .env`
2. 启动完整栈：`docker compose up --build`
3. 打开 `http://localhost:8080`

## GHCR 镜像发布与部署
- 仓库新增了 `/.github/workflows/docker-publish.yml`，会在推送到 `main` / `master`、推送 `v*` 标签或手动触发时构建并发布 3 个镜像到 GHCR。
- 当前仓库发布出的镜像名称为：`ghcr.io/bliod-cook/roaler-api`、`ghcr.io/bliod-cook/roaler-worker`、`ghcr.io/bliod-cook/roaler-web`。
- 部署时先复制环境变量：`cp .env.example .env`，再在 shell 或 `.env` 中显式设置：
  - `ROALER_IMAGE_TAG=<latest|vX.Y.Z|sha-...>`
- 使用示例编排启动：`docker compose -f docker-compose.ghcr.yml up -d`
- 如果 GHCR 包是私有的，先执行：`echo "$CR_PAT" | docker login ghcr.io -u <github-user> --password-stdin`

## 特性范围
- 单管理员初始化与登录
- RSS / Atom / JSON Feed / RSSHub 源管理
- 时间线、收藏、稍后读、集合归类
- 全文抽取与 PostgreSQL 全文搜索
- OPML 导入导出
- BYOK OpenAI 兼容 AI 摘要、翻译、标签、合集摘要
- WebSub/Webhook 近实时更新

## 注意事项
- RSSHub 需要外部实例，本项目不会自带 RSSHub 服务。
- WebSub 近实时更新需要实例具备可被 hub 回调的公网地址。
- 默认许可证为 `AGPL-3.0`。
